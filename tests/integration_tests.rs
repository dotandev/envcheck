use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::NamedTempFile;
use std::io::Write;

#[test]
fn test_cli_help() {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_envcheck"));
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Validate your development environment"));
}

#[test]
fn test_cli_config_not_found() {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_envcheck"));
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

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_envcheck"));
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

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_envcheck"));
    cmd.arg("--config").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Port 9999 is available"));
}

#[test]
fn test_cli_init() {
    let temp_dir = tempfile::tempdir().unwrap();
    let config_path = temp_dir.path().join(".envcheck.yaml");
    
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_envcheck"));
    cmd.current_dir(temp_dir.path()).arg("init");
    cmd.assert().success();
    
    assert!(config_path.exists());
    let content = std::fs::read_to_string(config_path).unwrap();
    assert!(content.contains("version: \"1\""));
    assert!(content.contains("tools:"));
}

#[test]
fn test_cli_json_output() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(
        file,
        r#"version: "1"
env_vars:
  - name: PATH
    required: true
"#
    ).unwrap();

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_envcheck"));
    cmd.arg("--config").arg(file.path()).arg("--json");
    
    let output = cmd.assert().success().get_output().stdout.clone();
    let stdout_str = String::from_utf8(output).unwrap();
    
    // Check if it's valid JSON and has expected fields
    let v: serde_json::Value = serde_json::from_str(&stdout_str).unwrap();
    assert!(v["results"].is_array());
    assert!(v["summary"].is_object());
    assert!(v["passed"].is_boolean());
}

#[test]
fn test_cli_env_regex() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(
        file,
        r#"version: "1"
env_vars:
  - name: TEST_REGEX_VAR
    pattern: "^[0-9]{{3}}$"
    required: true
"#
    ).unwrap();

    // Test failure
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_envcheck"));
    cmd.arg("--config").arg(file.path())
       .env("TEST_REGEX_VAR", "abc");
    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("TEST_REGEX_VAR is set but does not match pattern"));

    // Test success
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_envcheck"));
    cmd.arg("--config").arg(file.path())
       .env("TEST_REGEX_VAR", "123");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("TEST_REGEX_VAR is set and matches pattern"));
}

#[test]
fn test_cli_directory_check() {
    let temp_dir = tempfile::tempdir().unwrap();
    let dir_path = temp_dir.path().to_str().unwrap();
    
    let mut file = NamedTempFile::new().unwrap();
    writeln!(
        file,
        r#"version: "1"
files:
  - path: "{}"
    is_directory: true
    required: true
"#,
        dir_path
    ).unwrap();

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_envcheck"));
    cmd.arg("--config").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!("Directory {} exists", dir_path)));
}
