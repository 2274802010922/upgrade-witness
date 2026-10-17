use crate::types::WitnessConfig;
use anyhow::{Context, Result, bail};
use std::{
    fs,
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    thread,
    time::{Duration, Instant},
};

fn configure(command: &mut Command) {
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x08000000);
    }
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        command.process_group(0);
    }
}

fn stop_child(child: &mut Child) -> Result<()> {
    let pid = child.id();
    #[cfg(windows)]
    {
        if child.try_wait()?.is_none() {
            let mut killer = Command::new("taskkill");
            configure(&mut killer);
            let status = killer
                .args(["/PID", &pid.to_string(), "/T", "/F"])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()?;
            if !status.success() && child.try_wait()?.is_none() {
                bail!("Could not stop the owned process tree");
            }
        }
    }
    #[cfg(unix)]
    {
        let group = format!("-{pid}");
        let _ = Command::new("kill")
            .args(["-TERM", "--", &group])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
        thread::sleep(Duration::from_millis(100));
        let _ = Command::new("kill")
            .args(["-KILL", "--", &group])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
    let deadline = Instant::now() + Duration::from_secs(5);
    while child.try_wait()?.is_none() {
        if Instant::now() >= deadline {
            bail!("Owned process termination could not be confirmed");
        }
        thread::sleep(Duration::from_millis(25));
    }
    Ok(())
}

fn wait(child: &mut Child, timeout: Duration, label: &str) -> Result<()> {
    let deadline = Instant::now() + timeout;
    loop {
        if let Some(status) = child.try_wait()? {
            if status.success() {
                return Ok(());
            }
            bail!(
                "{label} exited unsuccessfully; run the trusted command separately for diagnostics"
            );
        }
        if Instant::now() >= deadline {
            stop_child(child)?;
            bail!("{label} timed out");
        }
        thread::sleep(Duration::from_millis(30));
    }
}

fn git_path_text(value: &str) -> String {
    #[cfg(windows)]
    {
        if let Some(rest) = value.strip_prefix("\\\\?\\UNC\\") {
            return format!("\\\\{rest}");
        }
        if let Some(rest) = value.strip_prefix("\\\\?\\") {
            return rest.to_string();
        }
    }
    value.to_string()
}

pub fn git_output(repo: &Path, args: &[&str]) -> Result<String> {
    let mut command = Command::new("git");
    command
        .arg("-C")
        .arg(git_path_text(&repo.to_string_lossy()))
        .args(args.iter().map(|value| git_path_text(value)))
        .stdout(Stdio::piped())
        .stderr(Stdio::null());
    configure(&mut command);
    let mut child = command.spawn().context("Could not start Git")?;
    let stdout = child.stdout.take().context("Missing Git output stream")?;
    let reader = thread::spawn(move || {
        let mut text = String::new();
        stdout
            .take(4 * 1024 * 1024)
            .read_to_string(&mut text)
            .map(|_| text)
    });
    let result = wait(&mut child, Duration::from_secs(30), "Git operation");
    let output = reader
        .join()
        .map_err(|_| anyhow::anyhow!("Git output reader failed"))??;
    result?;
    Ok(output)
}

fn resolve_commit(repo: &Path, reference: &str) -> Result<String> {
    if reference.is_empty() || reference.starts_with('-') || reference.contains('\0') {
        bail!("Invalid Git revision");
    }
    let sha = git_output(
        repo,
        &[
            "rev-parse",
            "--verify",
            "--end-of-options",
            &format!("{reference}^{{commit}}"),
        ],
    )?
    .trim()
    .to_string();
    if ![40, 64].contains(&sha.len()) || !sha.chars().all(|c| c.is_ascii_hexdigit()) {
        bail!("Invalid commit identity");
    }
    Ok(sha)
}

pub struct RevisionWorkspaces {
    pub base_dir: PathBuf,
    pub head_dir: PathBuf,
    pub base_sha: String,
    pub head_sha: String,
    pub dirty_files: Vec<String>,
    repo: PathBuf,
    run_root: PathBuf,
    owned_root: PathBuf,
    registered: Vec<PathBuf>,
    cleaned: bool,
}
impl RevisionWorkspaces {
    fn check_owned(&self) -> Result<()> {
        if self.owned_root.parent() != Some(self.run_root.as_path())
            || fs::symlink_metadata(&self.owned_root)?
                .file_type()
                .is_symlink()
            || fs::canonicalize(&self.owned_root)? != self.owned_root
        {
            bail!("Refusing cleanup outside the owned directory");
        }
        Ok(())
    }
    pub fn cleanup(&mut self) -> Result<()> {
        if self.cleaned {
            return Ok(());
        }
        self.check_owned()?;
        while let Some(directory) = self.registered.last() {
            if directory.parent() != Some(self.owned_root.as_path()) {
                bail!("Invalid worktree cleanup boundary");
            }
            git_output(
                &self.repo,
                &[
                    "worktree",
                    "remove",
                    "--force",
                    "--",
                    directory.to_str().context("Invalid workspace path")?,
                ],
            )?;
            self.registered.pop();
        }
        self.check_owned()?;
        fs::remove_dir_all(&self.owned_root)?;
        self.cleaned = true;
        Ok(())
    }
}
impl Drop for RevisionWorkspaces {
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}

pub fn create_revision_workspaces(
    repo: &Path,
    base_ref: &str,
    head_ref: &str,
    run: &Path,
) -> Result<RevisionWorkspaces> {
    let repo = fs::canonicalize(repo)?;
    git_output(&repo, &["rev-parse", "--show-toplevel"])?;
    let base_sha = resolve_commit(&repo, base_ref)?;
    let head_sha = resolve_commit(&repo, head_ref)?;
    let status = git_output(
        &repo,
        &["status", "--porcelain=v1", "-z", "--untracked-files=all"],
    )?;
    let mut dirty_files = Vec::new();
    let mut entries = status.split('\0');
    while let Some(entry) = entries.next() {
        if entry.len() < 3 {
            continue;
        }
        dirty_files.push(entry[3..].into());
        if entry[..2].contains(['R', 'C']) {
            if let Some(old) = entries.next() {
                dirty_files.push(old.into());
            }
        }
    }
    dirty_files.sort();
    dirty_files.dedup();
    fs::create_dir_all(run)?;
    let run_root = fs::canonicalize(run)?;
    let owned_root = tempfile::Builder::new()
        .prefix("worktrees-")
        .tempdir_in(&run_root)?
        .keep();
    let mut workspaces = RevisionWorkspaces {
        base_dir: owned_root.join("base"),
        head_dir: owned_root.join("head"),
        base_sha,
        head_sha,
        dirty_files,
        repo,
        run_root,
        owned_root,
        registered: vec![],
        cleaned: false,
    };
    for (directory, sha) in [
        (workspaces.base_dir.clone(), workspaces.base_sha.clone()),
        (workspaces.head_dir.clone(), workspaces.head_sha.clone()),
    ] {
        git_output(
            &workspaces.repo,
            &[
                "worktree",
                "add",
                "--detach",
                "--",
                directory.to_str().context("Invalid workspace path")?,
                &sha,
            ],
        )?;
        workspaces.registered.push(directory);
    }
    Ok(workspaces)
}

#[derive(Clone, Copy)]
pub struct StartOptions {
    pub install: bool,
    pub port: Option<u16>,
}
impl Default for StartOptions {
    fn default() -> Self {
        Self {
            install: true,
            port: None,
        }
    }
}

pub struct ProjectServer {
    pub url: String,
    child: Child,
    stopped: bool,
}
impl ProjectServer {
    pub fn stop(&mut self) -> Result<()> {
        if !self.stopped {
            stop_child(&mut self.child)?;
            self.stopped = true;
        }
        Ok(())
    }
}
impl Drop for ProjectServer {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

fn npm_command() -> Result<Command> {
    #[cfg(windows)]
    {
        use std::env;
        let mut candidates = Vec::new();
        if let Some(value) = env::var_os("npm_execpath") {
            candidates.push(PathBuf::from(value));
        }
        for entry in env::split_paths(&env::var_os("PATH").unwrap_or_default()) {
            candidates.push(entry.join("node_modules/npm/bin/npm-cli.js"));
        }
        for candidate in candidates {
            if candidate.is_file() && candidate.file_name().is_some_and(|n| n == "npm-cli.js") {
                let mut command = Command::new("node");
                command.arg(candidate);
                return Ok(command);
            }
        }
        bail!("Cannot locate npm-cli.js; install Node/npm for the target application");
    }
    #[cfg(not(windows))]
    {
        Ok(Command::new("npm"))
    }
}

fn start_npm(directory: &Path, args: &[&str]) -> Result<Child> {
    let mut command = npm_command()?;
    command
        .args(args)
        .current_dir(directory)
        .env("CI", "1")
        .env("BROWSER", "none")
        .env("npm_config_update_notifier", "false")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    configure(&mut command);
    command
        .spawn()
        .context("Could not start npm for the target project")
}

fn reachable(port: u16) -> bool {
    let address = SocketAddr::from(([127, 0, 0, 1], port));
    let Ok(mut stream) = TcpStream::connect_timeout(&address, Duration::from_millis(300)) else {
        return false;
    };
    let _ = stream.set_read_timeout(Some(Duration::from_millis(300)));
    let _ = stream.set_write_timeout(Some(Duration::from_millis(300)));
    if stream
        .write_all(b"GET / HTTP/1.0\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n")
        .is_err()
    {
        return false;
    }
    let mut bytes = [0; 128];
    let Ok(n) = stream.read(&mut bytes) else {
        return false;
    };
    let line = String::from_utf8_lossy(&bytes[..n]);
    line.split_whitespace()
        .nth(1)
        .and_then(|s| s.parse::<u16>().ok())
        .is_some_and(|s| (200..500).contains(&s))
}

pub fn prepare_and_start(
    project: &Path,
    config: &WitnessConfig,
    options: StartOptions,
) -> Result<ProjectServer> {
    let directory = fs::canonicalize(project)?;
    for file in ["package.json", "package-lock.json"] {
        let metadata = fs::symlink_metadata(directory.join(file))
            .with_context(|| format!("A regular {file} is required in each revision"))?;
        if !metadata.is_file() || metadata.file_type().is_symlink() {
            bail!("A regular {file} is required in each revision");
        }
    }
    let manifest: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(directory.join("package.json"))?)?;
    let _: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(directory.join("package-lock.json"))?)?;
    for script in
        std::iter::once(&config.project.start_script).chain(config.project.build_script.iter())
    {
        if manifest["scripts"][script].as_str().is_none() {
            bail!("Configured npm script is missing");
        }
    }
    if options.port.is_some_and(|p| p < 1024) {
        bail!("Requested port must be at least 1024");
    }
    if options.install {
        wait(
            &mut start_npm(&directory, &["ci", "--no-audit", "--no-fund"])?,
            Duration::from_secs(300),
            "npm ci",
        )?;
    }
    if let Some(script) = &config.project.build_script {
        wait(
            &mut start_npm(&directory, &["run", script])?,
            Duration::from_secs(180),
            "Project build",
        )?;
    }
    let listener = TcpListener::bind(("127.0.0.1", options.port.unwrap_or(0)))
        .context("Requested local port is unavailable")?;
    let port = listener.local_addr()?.port();
    drop(listener);
    let child = start_npm(
        &directory,
        &[
            "run",
            &config.project.start_script,
            "--",
            "--host",
            "127.0.0.1",
            "--port",
            &port.to_string(),
            "--strictPort",
        ],
    )?;
    let mut server = ProjectServer {
        url: format!("http://127.0.0.1:{port}"),
        child,
        stopped: false,
    };
    let deadline = Instant::now() + Duration::from_millis(config.timeout_ms);
    while Instant::now() < deadline {
        if server.child.try_wait()?.is_some() {
            bail!("Project server stopped before readiness");
        }
        if reachable(port) && server.child.try_wait()?.is_none() {
            return Ok(server);
        }
        thread::sleep(Duration::from_millis(50));
    }
    bail!("Project server readiness timed out")
}
