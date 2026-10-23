mod support;
use bridge_core::{error::BridgeError, model::*, store::Store};
use support::*;
#[tokio::test]
async fn second_writer_refused() {
    let dir = Temp::new();
    let store = Store::open(&dir.0).await.unwrap();
    assert!(matches!(
        Store::open(&dir.0).await,
        Err(BridgeError::Storage)
    ));
    drop(store);
    let _store = reopen(&dir).await;
}
#[tokio::test]
async fn disk_full_preserves_previous_state() {
    let dir = Temp::new();
    let store = Store::open(&dir.0).await.unwrap();
    let id = native(&store).await;
    let before = store.snapshot().await.unwrap();
    let mut events = store.subscribe();
    let c = rusqlite::Connection::open(dir.0.join("bridge.sqlite3")).unwrap();
    c.execute_batch("CREATE TRIGGER simulated_disk_full BEFORE UPDATE ON devices BEGIN SELECT RAISE(ABORT, 'simulated full disk'); END;").unwrap();
    assert_eq!(
        store
            .apply_updates(vec![DeviceUpdate {
                device_id: id,
                available: Some(false),
                states: Default::default()
            }])
            .await,
        Err(BridgeError::Storage)
    );
    assert_eq!(store.snapshot().await.unwrap(), before);
    assert!(events.try_recv().is_err());
    drop(store);
    let store = reopen(&dir).await;
    assert_eq!(store.snapshot().await.unwrap(), before);
}
#[tokio::test]
async fn migration_failure_preserves_old_database() {
    let dir = Temp::new();
    std::fs::create_dir_all(&dir.0).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&dir.0, std::fs::Permissions::from_mode(0o700)).unwrap();
    }
    let c = rusqlite::Connection::open(dir.0.join("bridge.sqlite3")).unwrap();
    c.execute_batch("CREATE TABLE devices (sentinel TEXT); INSERT INTO devices VALUES ('keep');")
        .unwrap();
    drop(c);
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(
            dir.0.join("bridge.sqlite3"),
            std::fs::Permissions::from_mode(0o600),
        )
        .unwrap();
    }
    assert!(matches!(
        Store::open(&dir.0).await,
        Err(BridgeError::Storage)
    ));
    let c = rusqlite::Connection::open(dir.0.join("bridge.sqlite3")).unwrap();
    let value: String = c
        .query_row("SELECT sentinel FROM devices", [], |r| r.get(0))
        .unwrap();
    assert_eq!(value, "keep");
    let migrations: i64 = c
        .query_row(
            "SELECT count(*) FROM sqlite_master WHERE name='schema_migrations'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(migrations, 0);
}

#[tokio::test]
async fn failed_command_write_does_not_admit_or_emit() {
    let dir = Temp::new();
    let store = Store::open(&dir.0).await.unwrap();
    let id = native(&store).await;
    let req = request(id);
    let mut events = store.subscribe();
    let c = rusqlite::Connection::open(dir.0.join("bridge.sqlite3")).unwrap();
    c.execute_batch("CREATE TRIGGER fail_command BEFORE INSERT ON commands BEGIN SELECT RAISE(ABORT,'write failed'); END").unwrap();
    assert_eq!(store.accept_command(&req).await, Err(BridgeError::Storage));
    assert!(events.try_recv().is_err());
    c.execute_batch("DROP TRIGGER fail_command").unwrap();
    assert!(store.accept_command(&req).await.unwrap().is_new);
}
#[tokio::test]
async fn retention_refuses_saturation_and_keeps_live_keys() {
    let dir = Temp::new();
    let store = Store::open(&dir.0).await.unwrap();
    let id = native(&store).await;
    let first = request(id);
    let admitted = store.accept_command(&first).await.unwrap();
    let mut c = rusqlite::Connection::open(dir.0.join("bridge.sqlite3")).unwrap();
    let tx = c.transaction().unwrap();
    let record = store.command(admitted.receipt.command_id).await.unwrap();
    for _ in 1..10000 {
        let mut record = record.clone();
        record.receipt.command_id = uuid::Uuid::new_v4();
        record.request.request_id = uuid::Uuid::new_v4();
        record.receipt.request_id = record.request.request_id;
        tx.execute(
            "INSERT INTO commands(id,request_id,created_at_ms,body) VALUES(?1,?2,?3,?4)",
            rusqlite::params![
                record.receipt.command_id.to_string(),
                record.request.request_id.to_string(),
                record.created_at_ms as i64,
                serde_json::to_string(&record).unwrap()
            ],
        )
        .unwrap();
    }
    tx.commit().unwrap();
    assert_eq!(
        store.accept_command(&request(id)).await,
        Err(BridgeError::Busy)
    );
    assert!(!store.accept_command(&first).await.unwrap().is_new);
    // Expired terminal receipts can be removed, but an old in-flight receipt cannot.
    let mut expired = record.clone();
    expired.created_at_ms = 1;
    expired.receipt.status = CommandStatus::Succeeded;
    c.execute(
        "UPDATE commands SET created_at_ms=1,body=?1 WHERE id=?2",
        rusqlite::params![
            serde_json::to_string(&expired).unwrap(),
            expired.receipt.command_id.to_string()
        ],
    )
    .unwrap();
    assert!(store.accept_command(&request(id)).await.unwrap().is_new);
    assert_eq!(
        store.command(expired.receipt.command_id).await,
        Err(BridgeError::NotFound)
    );
}

#[tokio::test]
async fn second_process_writer_refused() {
    let dir = Temp::new();
    let _store = Store::open(&dir.0).await.unwrap();
    let status = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "second_writer_child", "--nocapture"])
        .env("BRIDGE_TEST_LOCK_DIR", &dir.0)
        .status()
        .unwrap();
    assert!(status.success());
}
#[tokio::test]
async fn second_writer_child() {
    if let Some(dir) = std::env::var_os("BRIDGE_TEST_LOCK_DIR") {
        assert!(matches!(
            Store::open(std::path::Path::new(&dir)).await,
            Err(BridgeError::Storage)
        ));
    }
}

#[cfg(unix)]
#[tokio::test]
async fn storage_is_private_and_rejects_unsafe_existing_paths() {
    use std::os::unix::fs::{PermissionsExt, symlink};
    let dir = Temp::new();
    let store = Store::open(&dir.0).await.unwrap();
    assert_eq!(
        std::fs::metadata(&dir.0).unwrap().permissions().mode() & 0o777,
        0o700
    );
    for name in ["bridge.sqlite3", "bridge.lock"] {
        assert_eq!(
            std::fs::metadata(dir.0.join(name))
                .unwrap()
                .permissions()
                .mode()
                & 0o777,
            0o600
        );
    }
    let backup = dir.0.join("backup.sqlite");
    store.backup_to(&backup).await.unwrap();
    assert_eq!(
        std::fs::metadata(backup).unwrap().permissions().mode() & 0o777,
        0o600
    );
    let unsafe_dir = Temp::new();
    std::fs::create_dir(&unsafe_dir.0).unwrap();
    std::fs::set_permissions(&unsafe_dir.0, std::fs::Permissions::from_mode(0o755)).unwrap();
    assert!(matches!(
        Store::open(&unsafe_dir.0).await,
        Err(BridgeError::Storage)
    ));
    assert_eq!(
        std::fs::metadata(&unsafe_dir.0)
            .unwrap()
            .permissions()
            .mode()
            & 0o777,
        0o755
    );
    assert!(!unsafe_dir.0.join("bridge.lock").exists());
    let alias = Temp::new();
    let target = Temp::new();
    std::fs::create_dir(&target.0).unwrap();
    std::fs::set_permissions(&target.0, std::fs::Permissions::from_mode(0o700)).unwrap();
    symlink(&target.0, &alias.0).unwrap();
    assert!(matches!(
        Store::open(&alias.0).await,
        Err(BridgeError::Storage)
    ));
    assert!(matches!(
        Store::open(&alias.0.join(".")).await,
        Err(BridgeError::Storage)
    ));
    assert!(!target.0.join("bridge.lock").exists());
    std::fs::remove_file(&alias.0).unwrap();
    for name in ["bridge.sqlite3", "bridge.lock"] {
        let bad = Temp::new();
        std::fs::create_dir(&bad.0).unwrap();
        std::fs::set_permissions(&bad.0, std::fs::Permissions::from_mode(0o700)).unwrap();
        symlink(dir.0.join(name), bad.0.join(name)).unwrap();
        assert!(matches!(
            Store::open(&bad.0).await,
            Err(BridgeError::Storage)
        ));
        std::fs::remove_file(bad.0.join(name)).unwrap();
        std::fs::write(bad.0.join(name), b"preserve").unwrap();
        std::fs::set_permissions(bad.0.join(name), std::fs::Permissions::from_mode(0o644)).unwrap();
        assert!(matches!(
            Store::open(&bad.0).await,
            Err(BridgeError::Storage)
        ));
        assert_eq!(std::fs::read(bad.0.join(name)).unwrap(), b"preserve");
    }
}
