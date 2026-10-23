//! Serialized SQLite access. Events describe committed state; queued work is never replayed.
use crate::{
    error::BridgeError,
    event::{BridgeEvent, EventEnvelope},
    model::*,
};
use rusqlite::{Connection, OptionalExtension, Transaction, params};
use serde::{Serialize, de::DeserializeOwned};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::File,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::sync::{broadcast, mpsc, oneshot};
use uuid::Uuid;

#[cfg(unix)]
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt, PermissionsExt};

fn private_directory(path: &Path) -> Result<(), BridgeError> {
    #[cfg(unix)]
    {
        match std::fs::symlink_metadata(path) {
            Ok(_) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                storage(
                    std::fs::DirBuilder::new()
                        .recursive(true)
                        .mode(0o700)
                        .create(path),
                )?;
            }
            Err(_) => return Err(BridgeError::Storage),
        }
        let metadata = storage(std::fs::symlink_metadata(path))?;
        if metadata.is_dir() && metadata.permissions().mode() & 0o7777 == 0o700 {
            return Ok(());
        }
    }
    Err(BridgeError::Storage)
}
fn private_file(path: &Path, create_new: bool) -> Result<File, BridgeError> {
    #[cfg(unix)]
    {
        match std::fs::symlink_metadata(path) {
            Ok(metadata)
                if metadata.is_file() && metadata.permissions().mode() & 0o7777 == 0o600 => {}
            Ok(_) => return Err(BridgeError::Storage),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(_) => return Err(BridgeError::Storage),
        }
        let file = storage(
            std::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create(!create_new)
                .create_new(create_new)
                .truncate(false)
                .mode(0o600)
                .open(path),
        )?;
        let metadata = storage(file.metadata())?;
        if metadata.is_file() && metadata.permissions().mode() & 0o7777 == 0o600 {
            return Ok(file);
        }
    }
    Err(BridgeError::Storage)
}

type Job = Box<dyn FnOnce(&mut Worker) + Send>;
#[derive(Clone)]
pub struct Store {
    jobs: mpsc::Sender<Job>,
    events: broadcast::Sender<EventEnvelope>,
}
struct Worker {
    db: Connection,
    _lock: File,
    events: broadcast::Sender<EventEnvelope>,
    sequence: u64,
}
fn storage<T>(value: Result<T, impl std::fmt::Debug>) -> Result<T, BridgeError> {
    value.map_err(|_| BridgeError::Storage)
}
fn encode(value: &impl Serialize) -> Result<String, BridgeError> {
    storage(serde_json::to_string(value))
}
fn decode<T: DeserializeOwned>(body: String) -> Result<T, BridgeError> {
    storage(serde_json::from_str(&body))
}
fn now() -> Result<u64, BridgeError> {
    storage(SystemTime::now().duration_since(UNIX_EPOCH))
        .and_then(|n| storage(u64::try_from(n.as_millis())))
}
fn terminal(s: CommandStatus) -> bool {
    matches!(
        s,
        CommandStatus::Succeeded
            | CommandStatus::Failed
            | CommandStatus::Unknown
            | CommandStatus::Cancelled
    )
}
fn snapshot(db: &Connection) -> Result<Snapshot, BridgeError> {
    let revision: String = storage(db.query_row(
        "SELECT value FROM settings WHERE key='revision'",
        [],
        |r| r.get(0),
    ))?;
    let mut statement = storage(db.prepare("SELECT body FROM devices ORDER BY id"))?;
    let rows = storage(statement.query_map([], |r| r.get::<_, String>(0)))?;
    let devices = rows
        .map(|r| decode(storage(r)?))
        .collect::<Result<Vec<Device>, BridgeError>>()?;
    Ok(Snapshot {
        revision: storage(revision.parse())?,
        devices,
    })
}
fn integration_available(db: &Connection, integration: &str) -> Result<bool, BridgeError> {
    let value: Option<String> = storage(
        db.query_row(
            "SELECT value FROM settings WHERE key=?1",
            [format!("integration:{integration}:available")],
            |r| r.get(0),
        )
        .optional(),
    )?;
    value.map_or(Ok(true), decode)
}
fn save_device(db: &Connection, d: &Device) -> Result<(), BridgeError> {
    storage(db.execute("INSERT INTO devices(id,integration_id,external_id,body) VALUES (?1,?2,?3,?4) ON CONFLICT(id) DO UPDATE SET body=excluded.body",params![d.id.to_string(),d.integration_id,d.external_id,encode(d)?]))?;
    Ok(())
}
fn command(db: &Connection, id: CommandId) -> Result<CommandRecord, BridgeError> {
    let body = storage(
        db.query_row(
            "SELECT body FROM commands WHERE id=?1",
            [id.to_string()],
            |r| r.get(0),
        )
        .optional(),
    )?
    .ok_or(BridgeError::NotFound)?;
    decode(body)
}
fn save_command(db: &Connection, r: &CommandRecord) -> Result<(), BridgeError> {
    storage(db.execute("INSERT INTO commands(id,request_id,created_at_ms,body) VALUES (?1,?2,?3,?4) ON CONFLICT(id) DO UPDATE SET body=excluded.body",params![r.receipt.command_id.to_string(),r.request.request_id.to_string(),storage(i64::try_from(r.created_at_ms))?,encode(r)?]))?;
    Ok(())
}
fn revision(
    db: &Connection,
    ids: Vec<DeviceId>,
) -> Result<(Snapshot, Vec<BridgeEvent>), BridgeError> {
    let current = snapshot(db)?.revision;
    let next = current.checked_add(1).ok_or(BridgeError::Storage)?;
    storage(db.execute(
        "UPDATE settings SET value=?1 WHERE key='revision'",
        [next.to_string()],
    ))?;
    Ok((
        snapshot(db)?,
        vec![BridgeEvent::SnapshotChanged {
            revision: next,
            device_ids: ids,
        }],
    ))
}
fn validate_states(
    states: &BTreeMap<StateKey, StateSample>,
    limits: &BTreeMap<StateKey, NumericLimits>,
) -> Result<(), BridgeError> {
    for limit in limits.values() {
        limit.validate()?;
    }
    for (key, sample) in states {
        sample.validate_for(*key)?;
        if let (Some(Scalar::Integer(value)), Some(limit)) = (&sample.value, limits.get(key)) {
            limit.validate_value(*value)?;
        }
    }
    Ok(())
}
impl Worker {
    fn open(dir: &Path, events: broadcast::Sender<EventEnvelope>) -> Result<Self, BridgeError> {
        private_directory(dir)?;
        let lock = private_file(&dir.join("bridge.lock"), false)?;
        storage(lock.try_lock())?;
        let database = dir.join("bridge.sqlite3");
        drop(private_file(&database, false)?);
        let mut db = storage(Connection::open(database))?;
        storage(db.execute_batch("PRAGMA foreign_keys=ON; PRAGMA synchronous=FULL;"))?;
        let version: i64 = storage(db.query_row("PRAGMA user_version", [], |r| r.get(0)))?;
        if version == 0 {
            let tx = storage(db.transaction())?;
            storage(tx.execute_batch(include_str!("migrations/001.sql")))?;
            storage(tx.commit())?;
        } else if version != 1 {
            return Err(BridgeError::Storage);
        }
        let migration: i64 = storage(db.query_row(
            "SELECT max(version) FROM schema_migrations",
            [],
            |r| r.get(0),
        ))?;
        if migration != 1 {
            return Err(BridgeError::Storage);
        }
        let tx = storage(db.transaction())?;
        let records = {
            let mut s = storage(tx.prepare("SELECT body FROM commands"))?;
            let rows = storage(s.query_map([], |r| r.get::<_, String>(0)))?;
            rows.map(|r| decode(storage(r)?))
                .collect::<Result<Vec<CommandRecord>, BridgeError>>()?
        };
        for mut record in records {
            if !terminal(record.receipt.status) {
                record.receipt.status = CommandStatus::Unknown;
                record.updated_at_ms = now()?;
                save_command(&tx, &record)?;
            }
        }
        storage(tx.commit())?;
        snapshot(&db)?;
        Ok(Self {
            db,
            _lock: lock,
            events,
            sequence: 0,
        })
    }
    fn write<T>(
        &mut self,
        op: impl FnOnce(&Transaction<'_>) -> Result<(T, Vec<BridgeEvent>), BridgeError>,
    ) -> Result<T, BridgeError> {
        let tx = storage(self.db.transaction())?;
        let (result, events) = op(&tx)?;
        storage(tx.commit())?;
        for event in events {
            self.sequence += 1;
            let _ = self.events.send(EventEnvelope {
                sequence: self.sequence,
                event,
            });
        }
        Ok(result)
    }
}
impl Store {
    pub async fn open(data_dir: &Path) -> Result<Self, BridgeError> {
        let dir: std::path::PathBuf = data_dir.components().collect();
        let (jobs, mut rx) = mpsc::channel::<Job>(64);
        let (events, _) = broadcast::channel(256);
        let publish = events.clone();
        let (ready, result) = oneshot::channel();
        storage(
            std::thread::Builder::new()
                .name("bridge-sqlite".into())
                .spawn(move || match Worker::open(&dir, publish) {
                    Ok(mut worker) => {
                        if ready.send(Ok(())).is_err() {
                            return;
                        }
                        while let Some(job) = rx.blocking_recv() {
                            job(&mut worker);
                        }
                    }
                    Err(e) => {
                        let _ = ready.send(Err(e));
                    }
                }),
        )?;
        storage(result.await)??;
        Ok(Self { jobs, events })
    }
    async fn call<T: Send + 'static>(
        &self,
        op: impl FnOnce(&mut Worker) -> Result<T, BridgeError> + Send + 'static,
    ) -> Result<T, BridgeError> {
        let (reply, result) = oneshot::channel();
        self.jobs
            .try_send(Box::new(move |worker| {
                let _ = reply.send(op(worker));
            }))
            .map_err(|e| match e {
                mpsc::error::TrySendError::Full(_) => BridgeError::Busy,
                mpsc::error::TrySendError::Closed(_) => BridgeError::Storage,
            })?;
        storage(result.await)?
    }
    pub async fn snapshot(&self) -> Result<Snapshot, BridgeError> {
        self.call(|w| snapshot(&w.db)).await
    }
    pub fn subscribe(&self) -> broadcast::Receiver<EventEnvelope> {
        self.events.subscribe()
    }
    pub async fn upsert_discovered(
        &self,
        integration_id: &str,
        devices: Vec<DiscoveredDevice>,
    ) -> Result<Snapshot, BridgeError> {
        let integration_id = integration_id.to_owned();
        self.call(move |w| {
            w.write(|tx| {
                if integration_id.trim().is_empty() {
                    return Err(BridgeError::Validation);
                }
                let mut seen = BTreeSet::new();
                for d in &devices {
                    if d.external_id.is_empty() || !seen.insert(d.external_id.clone()) {
                        return Err(BridgeError::Validation);
                    }
                    validate_states(&d.states, &d.limits)?;
                }
                let mut existing = snapshot(tx)?.devices;
                let mut changed = Vec::new();
                for old in existing
                    .iter_mut()
                    .filter(|d| d.integration_id == integration_id)
                {
                    if !seen.contains(&old.external_id) && old.available {
                        old.available = false;
                        save_device(tx, old)?;
                        changed.push(old.id);
                    }
                }
                for discovered in devices {
                    let old = existing.iter().find(|d| {
                        d.integration_id == integration_id
                            && d.external_id == discovered.external_id
                    });
                    let d = Device {
                        id: old.map_or_else(Uuid::new_v4, |d| d.id),
                        integration_id: integration_id.clone(),
                        external_id: discovered.external_id,
                        name: discovered.name,
                        capabilities: discovered.capabilities,
                        limits: discovered.limits,
                        ownership: old.map_or(Ownership::Observe, |d| d.ownership),
                        available: discovered.available
                            && integration_available(tx, &integration_id)?,
                        states: discovered.states,
                    };
                    if old != Some(&d) {
                        save_device(tx, &d)?;
                        changed.push(d.id);
                    }
                }
                if changed.is_empty() {
                    Ok((snapshot(tx)?, vec![]))
                } else {
                    revision(tx, changed)
                }
            })
        })
        .await
    }
    pub async fn set_ownership(
        &self,
        id: DeviceId,
        ownership: Ownership,
        expected_revision: u64,
    ) -> Result<Snapshot, BridgeError> {
        self.call(move |w| {
            w.write(|tx| {
                let mut s = snapshot(tx)?;
                if s.revision != expected_revision {
                    return Err(BridgeError::Conflict);
                }
                let d = s
                    .devices
                    .iter_mut()
                    .find(|d| d.id == id)
                    .ok_or(BridgeError::NotFound)?;
                if d.ownership == ownership {
                    return Ok((s, vec![]));
                }
                d.ownership = ownership;
                save_device(tx, d)?;
                revision(tx, vec![id])
            })
        })
        .await
    }
    pub async fn apply_updates(&self, updates: Vec<DeviceUpdate>) -> Result<Snapshot, BridgeError> {
        self.call(move |w| {
            w.write(|tx| {
                let mut s = snapshot(tx)?;
                let mut ids = Vec::new();
                for update in updates {
                    let d = s
                        .devices
                        .iter_mut()
                        .find(|d| d.id == update.device_id)
                        .ok_or(BridgeError::NotFound)?;
                    validate_states(&update.states, &d.limits)?;
                    let before = d.clone();
                    if let Some(available) = update.available {
                        d.available = available && integration_available(tx, &d.integration_id)?;
                    }
                    d.states.extend(update.states);
                    if before != *d {
                        save_device(tx, d)?;
                        ids.push(d.id);
                    }
                }
                if ids.is_empty() {
                    Ok((s, vec![]))
                } else {
                    ids.sort();
                    ids.dedup();
                    revision(tx, ids)
                }
            })
        })
        .await
    }
    pub async fn set_integration_availability(
        &self,
        integration_id: &str,
        available: bool,
    ) -> Result<(), BridgeError> {
        let integration_id = integration_id.to_owned();
        self.call(move |w| {
            w.write(|tx| {
                if integration_id.trim().is_empty() { return Err(BridgeError::Validation); }
                let key = format!("integration:{integration_id}:available");
                let value = encode(&available)?;
                let old: Option<String> = storage(tx.query_row(
                    "SELECT value FROM settings WHERE key=?1", [&key], |r| r.get(0),
                ).optional())?;
                let mut ids = Vec::new();
                if !available {
                    for mut device in snapshot(tx)?.devices {
                        if device.integration_id == integration_id && device.available {
                            device.available = false;
                            save_device(tx, &device)?;
                            ids.push(device.id);
                        }
                    }
                }
                let mut events = if ids.is_empty() { Vec::new() } else { revision(tx, ids)?.1 };
                if old.as_ref() != Some(&value) {
                    storage(tx.execute(
                        "INSERT INTO settings(key,value) VALUES(?1,?2) ON CONFLICT(key) DO UPDATE SET value=excluded.value",
                        params![key,value],
                    ))?;
                    events.push(BridgeEvent::IntegrationChanged { integration_id, available });
                }
                Ok(((), events))
            })
        }).await
    }
    pub async fn accept_command(
        &self,
        request: &CommandRequest,
    ) -> Result<CommandAdmission, BridgeError> {
        let request = request.clone();
        self.call(move |w| {
            w.write(|tx| {
                let existing: Option<String> = storage(
                    tx.query_row(
                        "SELECT body FROM commands WHERE request_id=?1",
                        [request.request_id.to_string()],
                        |r| r.get(0),
                    )
                    .optional(),
                )?;
                if let Some(body) = existing {
                    let r: CommandRecord = decode(body)?;
                    if r.request.device_id != request.device_id
                        || r.request.command != request.command
                    {
                        return Err(BridgeError::Conflict);
                    }
                    return Ok((
                        CommandAdmission {
                            receipt: r.receipt,
                            is_new: false,
                        },
                        vec![],
                    ));
                }
                request.command.validate()?;
                let s = snapshot(tx)?;
                let d = s
                    .devices
                    .iter()
                    .find(|d| d.id == request.device_id)
                    .ok_or(BridgeError::NotFound)?;
                request.command.validate_for(d)?;
                if !integration_available(tx, &d.integration_id)? {
                    return Err(BridgeError::Unavailable);
                }
                let time = now()?;
                let cutoff = time.saturating_sub(24 * 60 * 60 * 1000);
                let old = {
                    let mut s = storage(
                        tx.prepare("SELECT id,body FROM commands WHERE created_at_ms < ?1"),
                    )?;
                    let rows = storage(s.query_map([storage(i64::try_from(cutoff))?], |r| {
                        Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?))
                    }))?;
                    rows.collect::<Result<Vec<_>, _>>()
                        .map_err(|_| BridgeError::Storage)?
                };
                for (id, body) in old {
                    let record: CommandRecord = decode(body)?;
                    if terminal(record.receipt.status) {
                        storage(tx.execute("DELETE FROM commands WHERE id=?1", [id]))?;
                    }
                }
                let count: i64 =
                    storage(tx.query_row("SELECT count(*) FROM commands", [], |r| r.get(0)))?;
                if count >= 10000 {
                    return Err(BridgeError::Busy);
                }
                let receipt = CommandReceipt {
                    command_id: Uuid::new_v4(),
                    request_id: request.request_id,
                    status: CommandStatus::Accepted,
                };
                let record = CommandRecord {
                    request,
                    receipt: receipt.clone(),
                    created_at_ms: time,
                    updated_at_ms: time,
                    upstream_execution_id: None,
                };
                save_command(tx, &record)?;
                let event = BridgeEvent::CommandChanged {
                    command_id: receipt.command_id,
                    status: receipt.status,
                };
                Ok((
                    CommandAdmission {
                        receipt,
                        is_new: true,
                    },
                    vec![event],
                ))
            })
        })
        .await
    }
    pub async fn mark_sent(&self, id: CommandId, execution_id: &str) -> Result<(), BridgeError> {
        let execution_id = execution_id.to_owned();
        self.call(move |w| {
            w.write(|tx| {
                if execution_id.is_empty() {
                    return Err(BridgeError::Validation);
                }
                let mut r = command(tx, id)?;
                if r.receipt.status == CommandStatus::Sent
                    && r.upstream_execution_id.as_ref() == Some(&execution_id)
                {
                    return Ok(((), vec![]));
                }
                if r.receipt.status != CommandStatus::Accepted {
                    return Err(BridgeError::Conflict);
                }
                r.receipt.status = CommandStatus::Sent;
                r.upstream_execution_id = Some(execution_id);
                r.updated_at_ms = now()?;
                save_command(tx, &r)?;
                Ok((
                    (),
                    vec![BridgeEvent::CommandChanged {
                        command_id: id,
                        status: CommandStatus::Sent,
                    }],
                ))
            })
        })
        .await
    }
    /// Tracking callers must supply manufacturer evidence, including late resolution of Unknown.
    pub async fn set_command_status(
        &self,
        id: CommandId,
        status: CommandStatus,
    ) -> Result<(), BridgeError> {
        self.call(move |w| {
            w.write(|tx| {
                let mut r = command(tx, id)?;
                if r.receipt.status == status {
                    return Ok(((), vec![]));
                }
                let late_evidence = r.receipt.status == CommandStatus::Unknown
                    && matches!(
                        status,
                        CommandStatus::Succeeded | CommandStatus::Failed | CommandStatus::Cancelled
                    );
                if (terminal(r.receipt.status) && !late_evidence)
                    || matches!(status, CommandStatus::Accepted | CommandStatus::Sent)
                {
                    return Err(BridgeError::Conflict);
                }
                r.receipt.status = status;
                r.updated_at_ms = now()?;
                save_command(tx, &r)?;
                Ok((
                    (),
                    vec![BridgeEvent::CommandChanged {
                        command_id: id,
                        status,
                    }],
                ))
            })
        })
        .await
    }
    pub async fn command(&self, id: CommandId) -> Result<CommandRecord, BridgeError> {
        self.call(move |w| command(&w.db, id)).await
    }
    pub async fn allocate_hap_ids(
        &self,
        device: DeviceId,
        service_keys: Vec<String>,
    ) -> Result<HapIdentity, BridgeError> {
        self.call(move |w| {
            w.write(|tx| {
                if service_keys.iter().any(|s| s.is_empty()) { return Err(BridgeError::Validation); }
                if !snapshot(tx)?.devices.iter().any(|d| d.id == device) { return Err(BridgeError::NotFound); }
                let old: Option<String> = storage(tx.query_row(
                    "SELECT body FROM hap_ids WHERE device_id=?1", [device.to_string()], |r| r.get(0),
                ).optional())?;
                let mut identity = if let Some(body) = old {
                    decode(body)?
                } else {
                    let max: i64 = storage(tx.query_row("SELECT COALESCE(max(aid),1) FROM hap_ids", [], |r| r.get(0)))?;
                    HapIdentity { aid: storage(u64::try_from(max.checked_add(1).ok_or(BridgeError::Storage)?))?, iids: BTreeMap::new() }
                };
                for key in service_keys {
                    if !identity.iids.contains_key(&key) {
                        let iid = identity.iids.values().copied().max().unwrap_or(1).checked_add(1).ok_or(BridgeError::Storage)?;
                        identity.iids.insert(key, iid);
                    }
                }
                storage(tx.execute(
                    "INSERT INTO hap_ids(device_id,aid,body) VALUES(?1,?2,?3) ON CONFLICT(device_id) DO UPDATE SET body=excluded.body",
                    params![device.to_string(),storage(i64::try_from(identity.aid))?,encode(&identity)?],
                ))?;
                Ok((identity, vec![]))
            })
        }).await
    }
    pub async fn backup_to(&self, destination: &Path) -> Result<(), BridgeError> {
        let destination = destination.to_owned();
        self.call(move |w| {
            // Back up to a new file so a failed copy cannot damage an existing backup.
            let reserved = private_file(&destination, true)?;
            drop(reserved);
            if let Err(error) = w.db.backup(rusqlite::MAIN_DB, &destination, None) {
                let _ = std::fs::remove_file(&destination);
                return storage::<()>(Err(error));
            }
            Ok(())
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn full_disk_keeps_revision_and_published_state() {
        let dir = std::env::temp_dir().join(format!("bridge-full-{}", Uuid::new_v4()));
        let store = Store::open(&dir).await.unwrap();
        let before = store.snapshot().await.unwrap();
        let mut events = store.subscribe();
        store
            .call(|w| {
                let pages: i64 = storage(w.db.query_row("PRAGMA page_count", [], |r| r.get(0)))?;
                storage(w.db.pragma_update(None, "max_page_count", pages))?;
                Ok(())
            })
            .await
            .unwrap();
        let discovered = DiscoveredDevice {
            external_id: "fictional".into(),
            name: "x".repeat(1_000_000),
            capabilities: vec![],
            limits: BTreeMap::new(),
            available: true,
            states: BTreeMap::new(),
        };
        assert_eq!(
            store.upsert_discovered("test", vec![discovered]).await,
            Err(BridgeError::Storage)
        );
        assert_eq!(store.snapshot().await.unwrap(), before);
        assert!(events.try_recv().is_err());
        drop(store);
        let _ = std::fs::remove_dir_all(dir);
    }
    #[tokio::test]
    async fn queue_rejects_the_sixty_fifth_waiting_job() {
        let dir = std::env::temp_dir().join(format!("bridge-queue-{}", Uuid::new_v4()));
        let store = Store::open(&dir).await.unwrap();
        let (entered, ready) = oneshot::channel();
        let (release, wait) = std::sync::mpsc::channel();
        store
            .jobs
            .try_send(Box::new(move |_| {
                let _ = entered.send(());
                let _ = wait.recv();
            }))
            .unwrap();
        ready.await.unwrap();
        for _ in 0..64 {
            store.jobs.try_send(Box::new(|_| {})).unwrap();
        }
        assert_eq!(store.snapshot().await, Err(BridgeError::Busy));
        release.send(()).unwrap();
        // Drain the backlog before cleaning the temporary database.
        while let Err(BridgeError::Busy) = store.snapshot().await {
            tokio::task::yield_now().await;
        }
        drop(store);
        let _ = std::fs::remove_dir_all(dir);
    }
}
