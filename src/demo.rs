use crate::workspace::git_output;
use anyhow::{Result, bail};
use std::{
    fs,
    path::Path,
    process::{Command, Stdio},
};

fn copy_tree(source: &Path, destination: &Path) -> Result<()> {
    fs::create_dir_all(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let name = entry.file_name();
        if ["node_modules", "dist", ".git", ".uw"]
            .iter()
            .any(|n| name == *n)
        {
            continue;
        }
        let kind = entry.file_type()?;
        if kind.is_symlink() {
            bail!("Demo fixture symlinks are not supported");
        }
        if kind.is_dir() {
            copy_tree(&entry.path(), &destination.join(name))?;
        } else if kind.is_file() {
            fs::copy(entry.path(), destination.join(name))?;
        }
    }
    Ok(())
}
fn commit(repo: &Path, message: &str) -> Result<String> {
    git_output(repo, &["add", "--all"])?;
    let mut command = Command::new("git");
    command
        .arg("-C")
        .arg(repo)
        .args([
            "-c",
            "commit.gpgsign=false",
            "-c",
            "core.hooksPath=",
            "commit",
            "--quiet",
            "-m",
            message,
        ])
        .env("GIT_AUTHOR_NAME", "Upgrade Witness Synthetic Demo")
        .env("GIT_AUTHOR_EMAIL", "synthetic-demo@upgrade-witness.invalid")
        .env("GIT_COMMITTER_NAME", "Upgrade Witness Synthetic Demo")
        .env(
            "GIT_COMMITTER_EMAIL",
            "synthetic-demo@upgrade-witness.invalid",
        )
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x08000000);
    }
    if !command.status()?.success() {
        bail!("Could not commit the synthetic demo state");
    }
    Ok(git_output(repo, &["rev-parse", "HEAD"])?.trim().into())
}
pub fn run_demo(fixtures: &Path, out: &Path) -> Result<i32> {
    if out.exists() {
        bail!("Demo output must be a new directory");
    }
    if !fixtures.join("base/package-lock.json").is_file()
        || !fixtures.join("head/package-lock.json").is_file()
    {
        bail!("Run demo from the repository or provide --fixtures");
    }
    fs::create_dir_all(out)?;
    let source = out.join("source");
    copy_tree(&fixtures.join("base"), &source)?;
    git_output(&source, &["init", "--quiet", "--initial-branch=demo"])?;
    let base = commit(&source, "test: tạo trạng thái gốc cho demo Rust")?;
    copy_tree(&fixtures.join("head"), &source)?;
    let head = commit(&source, "test: tạo trạng thái nâng cấp cho demo Rust")?;
    let report_dir = out.join("report");
    let report = crate::verify::verify(
        &source,
        &base,
        &head,
        &fixtures.join("witness.config.json"),
        &report_dir,
    )?;
    let expected = ["column-count", "primary-variant"];
    let found = expected.iter().all(|id| {
        report.findings.iter().any(|f| {
            f.kind == "regression"
                && f.scenario_id == "desktop-overview"
                && f.assertion_id.as_deref() == Some(id)
        })
    });
    println!(
        "{}",
        serde_json::to_string_pretty(
            &serde_json::json!({"verifierExit":report.exit_code,"complete":report.complete,"coverage":report.coverage,"report":report_dir.join("report.html")})
        )?
    );
    if report.exit_code != 1 || report.coverage.not_run != 0 || !found {
        bail!("Rust demo did not confirm the expected two regressions; inspect its report");
    }
    println!(
        "Rust demo passed: two expected assertion regressions; visual differences remain review items."
    );
    Ok(0)
}
