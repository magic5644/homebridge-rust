use std::{fs, process::Command};

#[test]
fn cli_version() {
    let output = Command::new(env!("CARGO_BIN_EXE_bridge-server"))
        .arg("--version")
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(
        String::from_utf8(output.stdout)
            .unwrap()
            .contains(env!("CARGO_PKG_VERSION"))
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn missing_config_fails_without_creating_files() {
    let directory = std::env::temp_dir().join(format!("bridge-missing-{}", std::process::id()));
    fs::create_dir(&directory).unwrap();
    let config = directory.join("missing config.toml");
    let output = Command::new(env!("CARGO_BIN_EXE_bridge-server"))
        .args(["check", "--config"])
        .arg(&config)
        .output()
        .unwrap();
    assert!(!output.status.success());
    let error = String::from_utf8(output.stderr).unwrap();
    assert!(error.contains("configuration"), "{error}");
    assert!(error.contains("missing config.toml"), "{error}");
    assert!(output.stdout.is_empty());
    assert_eq!(fs::read_dir(&directory).unwrap().count(), 0);
    fs::remove_dir(directory).unwrap();
}

#[test]
fn readable_config_is_not_overwritten() {
    let config = std::env::temp_dir().join(format!("bridge-config-{}.toml", std::process::id()));
    let contents = b"# Fictional configuration; schema belongs to a later ticket.\n";
    fs::write(&config, contents).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_bridge-server"))
        .args(["check", "--config"])
        .arg(&config)
        .output()
        .unwrap();
    assert!(output.status.success(), "{:?}", output);
    assert!(
        String::from_utf8(output.stdout)
            .unwrap()
            .contains("schema validation")
    );
    assert!(output.stderr.is_empty());
    assert_eq!(fs::read(&config).unwrap(), contents);
    fs::remove_file(config).unwrap();
}

#[test]
fn config_argument_is_required() {
    let output = Command::new(env!("CARGO_BIN_EXE_bridge-server"))
        .arg("check")
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(
        String::from_utf8(output.stderr)
            .unwrap()
            .contains("--config")
    );
}

#[test]
fn config_directory_is_rejected() {
    let output = Command::new(env!("CARGO_BIN_EXE_bridge-server"))
        .args(["check", "--config"])
        .arg(env!("CARGO_MANIFEST_DIR"))
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(
        String::from_utf8(output.stderr)
            .unwrap()
            .contains("configuration")
    );
}
