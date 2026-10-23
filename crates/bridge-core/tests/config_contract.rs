use bridge_core::{
    config::{load_config, save_config_atomic},
    error::BridgeError,
    secrets::load_secret,
};
use std::{
    fs,
    os::unix::fs::{PermissionsExt, symlink},
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};
static NEXT: AtomicU64 = AtomicU64::new(0);
struct Temp(PathBuf);
impl Temp {
    fn new() -> Self {
        let p = std::env::temp_dir().join(format!(
            "p07-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&p).unwrap();
        fs::set_permissions(&p, fs::Permissions::from_mode(0o700)).unwrap();
        Self(p.canonicalize().unwrap())
    }
    fn config(&self) -> PathBuf {
        let p = self.0.join("config.json");
        fs::write(&p, include_bytes!("../../../config.example.json")).unwrap();
        fs::set_permissions(&p, fs::Permissions::from_mode(0o600)).unwrap();
        p
    }
}
impl Drop for Temp {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}
fn mode(p: &Path) -> u32 {
    fs::metadata(p).unwrap().permissions().mode() & 0o777
}
#[test]
fn defaults_and_private_roundtrip_without_loading_credentials() {
    let t = Temp::new();
    let p = t.config();
    let config = load_config(&p).unwrap();
    assert_eq!(config.schema_version, 1);
    assert_eq!(config.listen.to_string(), "127.0.0.1:8582");
    assert_eq!(config.homekit.port, 51827);
    let dest = t.0.join("nested/config.json");
    save_config_atomic(&dest, &config).unwrap();
    assert_eq!(mode(dest.parent().unwrap()), 0o700);
    assert_eq!(mode(&dest), 0o600);
    assert!(load_config(&dest).unwrap() == config);
    assert!(!fs::read_to_string(dest).unwrap().contains("password"));
}
#[test]
fn invalid_input_and_failed_save_preserve_previous_bytes() {
    let t = Temp::new();
    let p = t.config();
    let previous = fs::read(&p).unwrap();
    let mut config = load_config(&p).unwrap();
    config.schema_version = 2;
    assert_eq!(
        save_config_atomic(&p, &config),
        Err(BridgeError::Validation)
    );
    assert_eq!(fs::read(&p).unwrap(), previous);
    for change in [
        serde_json::json!({"schema_version": 2}),
        serde_json::json!({"listen":"127.0.0.1:51827"}),
        serde_json::json!({"password":"SECRET-SENTINEL"}),
    ] {
        let mut value: serde_json::Value = serde_json::from_slice(&previous).unwrap();
        for (k, v) in change.as_object().unwrap() {
            value[k] = v.clone();
        }
        fs::write(&p, serde_json::to_vec(&value).unwrap()).unwrap();
        assert!(matches!(load_config(&p), Err(BridgeError::Validation)));
    }
}
#[test]
fn mixed_authentication_modes_and_secret_fields_are_rejected() {
    let t = Temp::new();
    let p = t.config();
    let mut value: serde_json::Value = serde_json::from_slice(&fs::read(&p).unwrap()).unwrap();
    value["tahoma"]["credentials_file"] = "other.json".into();
    fs::write(&p, serde_json::to_vec(&value).unwrap()).unwrap();
    assert!(matches!(load_config(&p), Err(BridgeError::Validation)));
}
#[test]
fn secrets_require_private_regular_files_and_redact_diagnostics() {
    let t = Temp::new();
    let p = t.0.join("token");
    fs::write(&p, "SECRET-SENTINEL").unwrap();
    fs::set_permissions(&p, fs::Permissions::from_mode(0o600)).unwrap();
    let secret = load_secret(&p).unwrap();
    assert_eq!(secret.expose(), "SECRET-SENTINEL");
    assert!(!format!("{secret:?}").contains("SECRET-SENTINEL"));
    fs::set_permissions(&p, fs::Permissions::from_mode(0o644)).unwrap();
    assert!(load_secret(&p).is_err());
    fs::set_permissions(&p, fs::Permissions::from_mode(0o600)).unwrap();
    fs::write(&p, " ").unwrap();
    assert!(load_secret(&p).is_err());
    fs::write(&p, vec![b'a'; 65537]).unwrap();
    assert!(load_secret(&p).is_err());
    assert!(load_secret(Path::new("https://example.invalid/token")).is_err());
}
#[test]
fn symlinks_and_unsafe_replacement_are_rejected() {
    let t = Temp::new();
    let p = t.config();
    let config = load_config(&p).unwrap();
    let link = t.0.join("link");
    symlink(&p, &link).unwrap();
    let previous = fs::read(&p).unwrap();
    assert!(load_config(&link).is_err());
    assert!(load_secret(&link).is_err());
    assert!(save_config_atomic(&link, &config).is_err());
    assert_eq!(fs::read(&p).unwrap(), previous);
    let directory_link = t.0.join("dir-link");
    symlink(&t.0, &directory_link).unwrap();
    assert!(save_config_atomic(&directory_link.join("other"), &config).is_err());
    assert!(!t.0.join("other").exists());
    fs::set_permissions(&p, fs::Permissions::from_mode(0o644)).unwrap();
    assert!(save_config_atomic(&p, &config).is_err());
    assert_eq!(fs::read(&p).unwrap(), previous);
}

#[test]
fn account_mode_roundtrips_references_without_serializing_loaded_credentials() {
    let t = Temp::new();
    let p = t.config();
    let mut value: serde_json::Value = serde_json::from_slice(&fs::read(&p).unwrap()).unwrap();
    let credentials = t.0.join("account.json");
    fs::write(
        &credentials,
        r#"{"username":"SECRET-USER","password":"SECRET-PASSWORD"}"#,
    )
    .unwrap();
    fs::set_permissions(&credentials, fs::Permissions::from_mode(0o600)).unwrap();
    value["tahoma"] = serde_json::json!({"mode":"somfy_account", "integration_id":"fictional", "provider":"somfy_europe", "gateway_pin":"0000-0000-0000", "credentials_file":credentials});
    fs::write(&p, serde_json::to_vec(&value).unwrap()).unwrap();
    let config = load_config(&p).unwrap();
    let secret = load_secret(&credentials).unwrap();
    assert!(secret.expose().contains("SECRET-PASSWORD"));
    save_config_atomic(&p, &config).unwrap();
    let written = fs::read_to_string(&p).unwrap();
    assert!(!written.contains("SECRET-USER"));
    assert!(!written.contains("SECRET-PASSWORD"));
    assert!(load_config(&p).unwrap() == config);
}
#[test]
fn invalid_config_does_not_create_parent_and_private_checks_do_not_chmod() {
    let t = Temp::new();
    let p = t.config();
    let mut config = load_config(&p).unwrap();
    config.homekit.port = 0;
    assert!(save_config_atomic(&t.0.join("missing/config.json"), &config).is_err());
    assert!(!t.0.join("missing").exists());
    let config = load_config(&p).unwrap();
    fs::set_permissions(&t.0, fs::Permissions::from_mode(0o755)).unwrap();
    assert!(save_config_atomic(&p, &config).is_err());
    assert_eq!(mode(&t.0), 0o755);
    assert!(load_secret(&p).is_err());
}

#[test]
fn non_utf8_paths_are_validation_errors_instead_of_serialization_panics() {
    use std::{ffi::OsString, os::unix::ffi::OsStringExt};
    let t = Temp::new();
    let p = t.config();
    let mut config = load_config(&p).unwrap();
    config.data_dir = OsString::from_vec(vec![0xff]).into();
    assert_eq!(
        save_config_atomic(&p, &config),
        Err(BridgeError::Validation)
    );
}
