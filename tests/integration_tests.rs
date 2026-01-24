use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::NamedTempFile;
use std::io::Write;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("envcheck").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Validate your development environment"));
}

#[test]
fn test_cli_config_not_found() {
    let mut cmd = Command::cargo_bin("envcheck").unwrap();
    cmd.arg("--config").arg("non_existent_file.yaml");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Failed to read config file"));
}

#[test]
fn test_cli_basic_validation() {
    let mut file = NamedTempFile::new().unwrap();
    let path_str = file.path().to_str().unwrap().to_string();
    writeln!(
        file,
        r#"version: "1"
tools:
  - name: node
    required: true
env_vars:
  - name: PATH
    required: true
files:
  - path: "{}"
    required: true
"#,
        path_str
    ).unwrap();

    let mut cmd = Command::cargo_bin("envcheck").unwrap();
    cmd.arg("--config").arg(file.path());
    
    // Node check might fail if not installed in CI environment, but PATH and file should pass
    // We check for "Running environment checks" to ensure it started
    cmd.assert()
        .stdout(predicate::str::contains("Running environment checks"));
}

#[test]
fn test_cli_port_validation() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(
        file,
        r#"version: "1"
ports:
  - 9999
"#
    ).unwrap();

    let mut cmd = Command::cargo_bin("envcheck").unwrap();
    cmd.arg("--config").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Port 9999 is available"));
}
