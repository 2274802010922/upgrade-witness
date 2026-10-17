use std::{
    fs,
    io::{Read, Write},
    net::TcpStream,
    path::Path,
};
use upgrade_witness::{
    types::{Project, WitnessConfig},
    workspace::{StartOptions, create_revision_workspaces, git_output, prepare_and_start},
};

fn git(repo: &Path, args: &[&str]) -> String {
    git_output(repo, args).unwrap().trim().to_string()
}
fn config() -> WitnessConfig {
    WitnessConfig {
        schema_version: 1,
        project: Project::default(),
        scenarios: vec![],
        intended_changes: vec![],
        timeout_ms: 5000,
    }
}
fn package(dir: &Path) {
    fs::write(
        dir.join("package.json"),
        r#"{"name":"uw-test","version":"1.0.0","scripts":{"dev":"node server.cjs"}}"#,
    )
    .unwrap();
    fs::write(dir.join("package-lock.json"),r#"{"name":"uw-test","version":"1.0.0","lockfileVersion":3,"packages":{"":{"name":"uw-test","version":"1.0.0"}}}"#).unwrap();
}

#[test]
fn detached_revisions_preserve_dirty_source_and_cleanup() {
    let repo = tempfile::Builder::new()
        .prefix("uw Rust workspace ")
        .tempdir()
        .unwrap();
    let runs = tempfile::tempdir().unwrap();
    git(repo.path(), &["init"]);
    git(
        repo.path(),
        &["config", "user.name", "Synthetic Workspace Test"],
    );
    git(
        repo.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    fs::write(repo.path().join("app"), "base").unwrap();
    git(repo.path(), &["add", "app"]);
    git(repo.path(), &["commit", "-m", "test: tạo bản gốc"]);
    let base = git(repo.path(), &["rev-parse", "HEAD"]);
    fs::write(repo.path().join("app"), "head").unwrap();
    git(repo.path(), &["commit", "-am", "test: tạo bản mới"]);
    let head = git(repo.path(), &["rev-parse", "HEAD"]);
    fs::write(repo.path().join("app"), "private change").unwrap();
    fs::write(repo.path().join("untracked"), "private").unwrap();
    fs::write(repo.path().join("staged"), "private").unwrap();
    git(repo.path(), &["add", "staged"]);
    let status = git(repo.path(), &["status", "--porcelain"]);
    let branch = git(repo.path(), &["branch", "--show-current"]);
    let mut work = create_revision_workspaces(repo.path(), &base, &head, runs.path()).unwrap();
    assert_eq!(
        fs::read_to_string(work.base_dir.join("app")).unwrap(),
        "base"
    );
    assert_eq!(
        fs::read_to_string(work.head_dir.join("app")).unwrap(),
        "head"
    );
    assert_eq!(work.dirty_files.len(), 3);
    assert_eq!(git(&work.base_dir, &["branch", "--show-current"]), "");
    work.cleanup().unwrap();
    work.cleanup().unwrap();
    assert!(!work.base_dir.exists());
    assert_eq!(git(repo.path(), &["status", "--porcelain"]), status);
    assert_eq!(git(repo.path(), &["branch", "--show-current"]), branch);
}
#[test]
fn invalid_ref_does_not_create_worktrees() {
    let repo = tempfile::tempdir().unwrap();
    let out = tempfile::tempdir().unwrap();
    git(repo.path(), &["init"]);
    assert!(create_revision_workspaces(repo.path(), "--help", "HEAD", out.path()).is_err());
    assert_eq!(fs::read_dir(out.path()).unwrap().count(), 0);
}
#[test]
fn missing_lockfile_prevents_project_execution() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("package.json"), "{}").unwrap();
    assert!(
        prepare_and_start(
            dir.path(),
            &config(),
            StartOptions {
                install: false,
                port: None
            }
        )
        .is_err()
    );
}
#[test]
fn npm_server_runs_stops_and_reuses_origin() {
    let dir = tempfile::Builder::new()
        .prefix("uw node fixture ")
        .tempdir()
        .unwrap();
    package(dir.path());
    fs::write(dir.path().join("server.cjs"),"const http=require('node:http'); const port=Number(process.argv[process.argv.indexOf('--port')+1]);http.createServer((q,r)=>r.end('ready')).listen(port,'127.0.0.1');").unwrap();
    let mut server = prepare_and_start(
        dir.path(),
        &config(),
        StartOptions {
            install: false,
            port: None,
        },
    )
    .unwrap();
    let address = url::Url::parse(&server.url).unwrap();
    let port = address.port().unwrap();
    let mut stream = TcpStream::connect(("127.0.0.1", port)).unwrap();
    stream
        .write_all(b"GET / HTTP/1.0\r\nHost: localhost\r\n\r\n")
        .unwrap();
    let mut data = String::new();
    stream.read_to_string(&mut data).unwrap();
    assert!(data.contains("ready"));
    drop(stream);
    server.stop().unwrap();
    server.stop().unwrap();
    assert!(TcpStream::connect(("127.0.0.1", port)).is_err());
    let mut again = prepare_and_start(
        dir.path(),
        &config(),
        StartOptions {
            install: false,
            port: Some(port),
        },
    )
    .unwrap();
    assert_eq!(again.url, server.url);
    assert!(
        prepare_and_start(
            dir.path(),
            &config(),
            StartOptions {
                install: false,
                port: Some(port)
            }
        )
        .is_err()
    );
    again.stop().unwrap();
}
