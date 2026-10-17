use std::process::Command;
#[test]
fn help_and_version_are_native_rust_commands() {
    let exe = env!("CARGO_BIN_EXE_uw");
    let help = Command::new(exe).arg("--help").output().unwrap();
    assert!(help.status.success());
    assert!(String::from_utf8_lossy(&help.stdout).contains("Native Rust"));
    let v = Command::new(exe).arg("--version").output().unwrap();
    assert!(v.status.success());
    assert!(String::from_utf8_lossy(&v.stdout).contains("0.2.0-alpha.1"));
}
#[test]
fn missing_trust_prevents_execution() {
    let output = Command::new(env!("CARGO_BIN_EXE_uw"))
        .args([
            "verify", "--repo", "unused", "--base", "base", "--head", "head", "--config", "unused",
            "--out", "unused",
        ])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(64));
    assert!(String::from_utf8_lossy(&output.stderr).contains("trust-project"));
}
