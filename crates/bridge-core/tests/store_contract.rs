mod support;
use bridge_core::{error::BridgeError, event::BridgeEvent, model::*, store::Store};
use support::*;
#[tokio::test]
async fn identity_survives_rename_restart() {
    let dir = Temp::new();
    let store = Store::open(&dir.0).await.unwrap();
    let id = native(&store).await;
    let hap = store
        .allocate_hap_ids(id, vec!["cover".into(), "tilt".into()])
        .await
        .unwrap();
    store.upsert_discovered("test", vec![]).await.unwrap();
    let s = store
        .upsert_discovered("test", vec![device("renamed")])
        .await
        .unwrap();
    assert_eq!(s.devices[0].id, id);
    assert_eq!(s.devices[0].ownership, Ownership::Native);
    assert_eq!(
        store
            .allocate_hap_ids(id, vec!["cover".into()])
            .await
            .unwrap(),
        hap
    );
    let rev = s.revision;
    drop(store);
    let store = reopen(&dir).await;
    assert_eq!(store.snapshot().await.unwrap().revision, rev);
    assert_eq!(
        store
            .allocate_hap_ids(id, vec!["tilt".into(), "cover".into()])
            .await
            .unwrap(),
        hap
    );
}
#[tokio::test]
async fn revision_and_state_commit_together() {
    let dir = Temp::new();
    let store = Store::open(&dir.0).await.unwrap();
    let mut events = store.subscribe();
    let s = store
        .upsert_discovered("test", vec![device("first")])
        .await
        .unwrap();
    let e = events.try_recv().unwrap();
    assert!(matches!(e.event,BridgeEvent::SnapshotChanged{revision,..} if revision==s.revision));
    assert_eq!(store.snapshot().await.unwrap(), s);
    assert_eq!(
        store
            .set_ownership(s.devices[0].id, Ownership::Native, 0)
            .await,
        Err(BridgeError::Conflict)
    );
    assert!(events.try_recv().is_err());
    let update = DeviceUpdate {
        device_id: s.devices[0].id,
        available: Some(false),
        states: Default::default(),
    };
    let sample = StateSample {
        value: Some(Scalar::Integer(35)),
        quality: StateQuality::Confirmed,
        observed_at_ms: Some(123),
    };
    let mut update = update;
    update.states.insert(StateKey::PositionPct, sample.clone());
    let next = store.apply_updates(vec![update]).await.unwrap();
    assert_eq!(next.revision, s.revision + 1);
    assert_eq!(next.devices[0].states[&StateKey::PositionPct], sample);
    assert!(!next.devices[0].available);
    store
        .set_integration_availability("test", false)
        .await
        .unwrap();
    assert!(matches!(
        events.try_recv().unwrap().event,
        BridgeEvent::SnapshotChanged { .. }
    ));
    assert!(matches!(
        events.try_recv().unwrap().event,
        BridgeEvent::IntegrationChanged {
            available: false,
            ..
        }
    ));
}
#[tokio::test]
async fn idempotency_persists_and_interrupted_command_becomes_unknown() {
    let dir = Temp::new();
    let store = Store::open(&dir.0).await.unwrap();
    let id = native(&store).await;
    let req = request(id);
    let admission = store.accept_command(&req).await.unwrap();
    assert!(admission.is_new);
    let duplicate = store.accept_command(&req).await.unwrap();
    assert!(!duplicate.is_new);
    assert_eq!(duplicate.receipt, admission.receipt);
    let mut conflict = req.clone();
    conflict.command = Command::Close;
    assert_eq!(
        store.accept_command(&conflict).await,
        Err(BridgeError::Conflict)
    );
    store
        .mark_sent(admission.receipt.command_id, "fictional-execution")
        .await
        .unwrap();
    let record = store.command(admission.receipt.command_id).await.unwrap();
    assert_eq!(record.receipt.status, CommandStatus::Sent);
    assert_eq!(
        record.upstream_execution_id.as_deref(),
        Some("fictional-execution")
    );
    drop(store);
    let store = reopen(&dir).await;
    let record = store.command(admission.receipt.command_id).await.unwrap();
    assert_eq!(record.receipt.status, CommandStatus::Unknown);
    let duplicate = store.accept_command(&req).await.unwrap();
    assert!(!duplicate.is_new);
    assert_eq!(duplicate.receipt.status, CommandStatus::Unknown);
}
#[tokio::test]
async fn command_admission_checks_current_device_and_backup_is_consistent() {
    let dir = Temp::new();
    let store = Store::open(&dir.0).await.unwrap();
    let s = store
        .upsert_discovered("test", vec![device("first")])
        .await
        .unwrap();
    let id = s.devices[0].id;
    assert_eq!(
        store.accept_command(&request(id)).await,
        Err(BridgeError::NotNative)
    );
    let s = store
        .set_ownership(id, Ownership::Native, s.revision)
        .await
        .unwrap();
    let backup = dir.0.join("backup.sqlite");
    store.backup_to(&backup).await.unwrap();
    let c = rusqlite::Connection::open(backup).unwrap();
    let revision: String = c
        .query_row("SELECT value FROM settings WHERE key='revision'", [], |r| {
            r.get(0)
        })
        .unwrap();
    assert_eq!(revision.parse::<u64>().unwrap(), s.revision);
    store
        .set_integration_availability("test", false)
        .await
        .unwrap();
    assert_eq!(
        store.accept_command(&request(id)).await,
        Err(BridgeError::Unavailable)
    );
}

#[tokio::test]
async fn concurrent_duplicate_has_one_sender_and_terminal_state_does_not_reopen() {
    let dir = Temp::new();
    let store = Store::open(&dir.0).await.unwrap();
    let id = native(&store).await;
    let req = request(id);
    let (a, b) = tokio::join!(store.accept_command(&req), store.accept_command(&req));
    let a = a.unwrap();
    let b = b.unwrap();
    assert_ne!(a.is_new, b.is_new);
    assert_eq!(a.receipt, b.receipt);
    store
        .set_command_status(a.receipt.command_id, CommandStatus::Unknown)
        .await
        .unwrap();
    assert_eq!(
        store.mark_sent(a.receipt.command_id, "late").await,
        Err(BridgeError::Conflict)
    );
}
#[tokio::test]
async fn lagged_subscriber_rereads_committed_snapshot() {
    let dir = Temp::new();
    let store = Store::open(&dir.0).await.unwrap();
    let mut subscriber = store.subscribe();
    for i in 0..260 {
        store
            .upsert_discovered("test", vec![device(&format!("name-{i}"))])
            .await
            .unwrap();
    }
    assert!(matches!(
        subscriber.try_recv(),
        Err(tokio::sync::broadcast::error::TryRecvError::Lagged(_))
    ));
    assert_eq!(store.snapshot().await.unwrap().devices[0].name, "name-259");
}
#[tokio::test]
async fn invalid_batch_rolls_back_all_devices_and_events() {
    let dir = Temp::new();
    let store = Store::open(&dir.0).await.unwrap();
    let id = native(&store).await;
    let before = store.snapshot().await.unwrap();
    let mut subscriber = store.subscribe();
    let updates = vec![
        DeviceUpdate {
            device_id: id,
            available: Some(false),
            states: Default::default(),
        },
        DeviceUpdate {
            device_id: uuid::Uuid::new_v4(),
            available: Some(false),
            states: Default::default(),
        },
    ];
    assert_eq!(
        store.apply_updates(updates).await,
        Err(BridgeError::NotFound)
    );
    assert_eq!(store.snapshot().await.unwrap(), before);
    assert!(subscriber.try_recv().is_err());
}

#[tokio::test]
async fn integration_outage_is_visible_after_resync_without_invented_recovery() {
    let dir = Temp::new();
    let store = Store::open(&dir.0).await.unwrap();
    let id = native(&store).await;
    let before = store.snapshot().await.unwrap();
    let mut events = store.subscribe();
    store
        .set_integration_availability("test", false)
        .await
        .unwrap();
    let offline = store.snapshot().await.unwrap();
    assert_eq!(offline.revision, before.revision + 1);
    assert!(!offline.devices[0].available);
    assert!(matches!(
        events.try_recv().unwrap().event,
        BridgeEvent::SnapshotChanged { .. }
    ));
    assert!(matches!(
        events.try_recv().unwrap().event,
        BridgeEvent::IntegrationChanged {
            available: false,
            ..
        }
    ));
    store
        .apply_updates(vec![DeviceUpdate {
            device_id: id,
            available: Some(true),
            states: Default::default(),
        }])
        .await
        .unwrap();
    assert!(!store.snapshot().await.unwrap().devices[0].available);
    store
        .upsert_discovered("test", vec![device("rediscovered")])
        .await
        .unwrap();
    assert!(!store.snapshot().await.unwrap().devices[0].available);
    store
        .set_integration_availability("test", true)
        .await
        .unwrap();
    assert!(!store.snapshot().await.unwrap().devices[0].available);
    assert_eq!(
        store.accept_command(&request(id)).await,
        Err(BridgeError::Unavailable)
    );
    store
        .apply_updates(vec![DeviceUpdate {
            device_id: id,
            available: Some(true),
            states: Default::default(),
        }])
        .await
        .unwrap();
    assert!(store.accept_command(&request(id)).await.unwrap().is_new);
}

#[tokio::test]
async fn inventory_order_does_not_recycle_device_or_hap_identity() {
    let dir = Temp::new();
    let store = Store::open(&dir.0).await.unwrap();
    let mut second = device("second");
    second.external_id = "second-source".into();
    let first = store
        .upsert_discovered("test", vec![device("first"), second.clone()])
        .await
        .unwrap();
    let identities = first
        .devices
        .iter()
        .map(|d| (d.external_id.clone(), d.id))
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut hap = std::collections::BTreeMap::new();
    for d in &first.devices {
        hap.insert(
            d.id,
            store
                .allocate_hap_ids(d.id, vec!["cover".into()])
                .await
                .unwrap(),
        );
    }
    let next = store
        .upsert_discovered("test", vec![second.clone(), device("renamed")])
        .await
        .unwrap();
    for d in &next.devices {
        assert_eq!(identities[&d.external_id], d.id);
        assert_eq!(
            hap[&d.id],
            store
                .allocate_hap_ids(d.id, vec!["cover".into()])
                .await
                .unwrap()
        );
    }
    store.upsert_discovered("test", vec![]).await.unwrap();
    let mut third = device("third");
    third.external_id = "third-source".into();
    let last = store.upsert_discovered("test", vec![third]).await.unwrap();
    let new = last
        .devices
        .iter()
        .find(|d| d.external_id == "third-source")
        .unwrap();
    assert!(!identities.values().any(|id| *id == new.id));
    let new_hap = store
        .allocate_hap_ids(new.id, vec!["cover".into()])
        .await
        .unwrap();
    assert!(hap.values().all(|old| new_hap.aid > old.aid));
}

#[tokio::test]
async fn late_evidence_resolves_unknown_without_reopening_transmission() {
    let dir = Temp::new();
    let store = Store::open(&dir.0).await.unwrap();
    let id = native(&store).await;
    let req = request(id);
    let command = store.accept_command(&req).await.unwrap().receipt.command_id;
    store
        .set_command_status(command, CommandStatus::Unknown)
        .await
        .unwrap();
    let duplicate = store.accept_command(&req).await.unwrap();
    assert!(!duplicate.is_new);
    assert_eq!(duplicate.receipt.status, CommandStatus::Unknown);
    assert_eq!(
        store
            .set_command_status(command, CommandStatus::Running)
            .await,
        Err(BridgeError::Conflict)
    );
    assert_eq!(
        store.mark_sent(command, "late").await,
        Err(BridgeError::Conflict)
    );
    store
        .set_command_status(command, CommandStatus::Succeeded)
        .await
        .unwrap();
    let duplicate = store.accept_command(&req).await.unwrap();
    assert!(!duplicate.is_new);
    assert_eq!(duplicate.receipt.status, CommandStatus::Succeeded);
    assert_eq!(
        store
            .set_command_status(command, CommandStatus::Failed)
            .await,
        Err(BridgeError::Conflict)
    );
}
