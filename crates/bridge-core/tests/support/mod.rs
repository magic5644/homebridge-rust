use bridge_core::{model::*, store::Store};
use std::{collections::BTreeMap, path::PathBuf};
pub struct Temp(pub PathBuf);
impl Temp {
    pub fn new() -> Self {
        Self(std::env::temp_dir().join(format!("bridge-store-{}", uuid::Uuid::new_v4())))
    }
}
impl Drop for Temp {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
pub fn device(name: &str) -> DiscoveredDevice {
    DiscoveredDevice {
        external_id: "fictional-device".into(),
        name: name.into(),
        capabilities: vec![Capability::OpenCloseStop],
        limits: BTreeMap::new(),
        available: true,
        states: BTreeMap::new(),
    }
}
pub async fn reopen(dir: &Temp) -> Store {
    for _ in 0..100 {
        if let Ok(store) = Store::open(&dir.0).await {
            return store;
        }
        tokio::time::sleep(std::time::Duration::from_millis(2)).await;
    }
    panic!("store did not release its lock");
}
pub async fn native(store: &Store) -> DeviceId {
    let s = store
        .upsert_discovered("test", vec![device("first")])
        .await
        .unwrap();
    let id = s.devices[0].id;
    store
        .set_ownership(id, Ownership::Native, s.revision)
        .await
        .unwrap();
    id
}
pub fn request(id: DeviceId) -> CommandRequest {
    CommandRequest {
        request_id: uuid::Uuid::new_v4(),
        device_id: id,
        command: Command::Open,
        origin: CommandOrigin::Test,
    }
}
