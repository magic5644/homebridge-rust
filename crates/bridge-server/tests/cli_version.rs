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
    assert!(output.stdout.is_empty());
    assert_eq!(fs::read_dir(&directory).unwrap().count(), 0);
    fs::remove_dir(directory).unwrap();
}

#[test]
fn invalid_readable_config_is_rejected_without_overwriting() {
    let config = std::env::temp_dir().join(format!("bridge-config-{}.toml", std::process::id()));
    let contents = b"# Invalid fictional configuration.\n";
    fs::write(&config, contents).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_bridge-server"))
        .args(["check", "--config"])
        .arg(&config)
        .output()
        .unwrap();
    assert!(!output.status.success(), "{:?}", output);
    assert!(output.stdout.is_empty());
    assert!(
        !String::from_utf8(output.stderr)
            .unwrap()
            .contains("Fictional")
    );
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

#[test]
fn valid_config_check_is_read_only_and_does_not_load_referenced_credentials() {
    use std::os::unix::fs::PermissionsExt;
    let directory = std::env::temp_dir().join(format!("bridge-valid-{}", std::process::id()));
    fs::create_dir(&directory).unwrap();
    fs::set_permissions(&directory, fs::Permissions::from_mode(0o700)).unwrap();
    let directory = directory.canonicalize().unwrap();
    let config = directory.join("config.json");
    let contents = include_bytes!("../../../config.example.json");
    fs::write(&config, contents).unwrap();
    fs::set_permissions(&config, fs::Permissions::from_mode(0o600)).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_bridge-server"))
        .args(["check", "--config"])
        .arg(&config)
        .output()
        .unwrap();
    assert!(output.status.success(), "{:?}", output);
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "Configuration is valid.\n"
    );
    assert!(output.stderr.is_empty());
    assert_eq!(fs::read(config).unwrap(), contents);
    assert_eq!(fs::read_dir(&directory).unwrap().count(), 1);
    fs::remove_dir_all(directory).unwrap();
}
