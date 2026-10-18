use hap::{
    Config, Pin,
    accessory::{
        AccessoryCategory, AccessoryInformation, bridge::BridgeAccessory, switch::SwitchAccessory,
        window_covering::WindowCoveringAccessory,
    },
    server::{IpServer, Server},
    storage::{FileStorage, Storage},
};
use rand::{Rng, rngs::OsRng};
use std::{error::Error, net::IpAddr, path::Path, time::Duration};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn info(name: &str) -> AccessoryInformation {
    AccessoryInformation {
        name: name.into(),
        manufacturer: "P04 Fictional".into(),
        model: "Simulation".into(),
        serial_number: name.into(),
        ..Default::default()
    }
}

fn covering() -> hap::Result<WindowCoveringAccessory> {
    WindowCoveringAccessory::new(3, info("P04 Fictional Covering"))
}

fn parse_config(bytes: &[u8]) -> Result<Config> {
    Ok(hap::serde_json::from_slice(bytes)?)
}

async fn load_config(dir: &Path, host: IpAddr) -> Result<Config> {
    let path = dir.join("config.json");
    let mut config = match std::fs::read(&path) {
        Ok(bytes) => parse_config(&bytes)?,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            if dir.join("pairings").exists() {
                return Err(
                    "missing identity with existing pairing storage; refusing reset".into(),
                );
            }
            let mut config = Config::default();
            loop {
                let digits = std::array::from_fn(|_| OsRng.gen_range(0, 10));
                if let Ok(pin) = Pin::new(digits) {
                    config.pin = pin;
                    break;
                }
            }
            config
        }
        Err(error) => return Err(error.into()),
    };
    config.host = host;
    config.port = 52104;
    config.name = "P04 Fictional Bridge".into();
    config.category = AccessoryCategory::Bridge;
    config.max_peers = Some(4);
    Ok(config)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 2
        || args[0] != "--serve-isolated-lan"
        || args.len() > 3
        || (args.len() == 3 && args[2] != "--without-covering")
    {
        return Err("owner-run only: --serve-isolated-lan TEST_INTERFACE_IP [--without-covering]; read docs/hardware/hap-proof.md first".into());
    }
    let host: IpAddr = args[1].parse()?;
    if host.is_unspecified() || host.is_multicast() {
        return Err("use one explicit isolated test interface address".into());
    }
    // A fixed, ignored directory prevents selecting Homebridge production storage.
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../private/hap-proof");
    use std::os::unix::fs::{DirBuilderExt, PermissionsExt};
    std::fs::DirBuilder::new()
        .recursive(true)
        .mode(0o700)
        .create(&dir)?;
    let metadata = std::fs::symlink_metadata(&dir)?;
    if !metadata.is_dir() || metadata.permissions().mode() & 0o077 != 0 {
        return Err("test state must be a real directory accessible only to its owner".into());
    }
    let config = load_config(&dir, host).await?;
    let mut storage = FileStorage::new(dir.as_os_str()).await?;
    storage.save_config(&config).await?;
    let server = IpServer::new(config, storage).await?;
    server
        .add_accessory(BridgeAccessory::new(1, info("P04 Fictional Bridge"))?)
        .await?;
    let switch = server
        .add_accessory(SwitchAccessory::new(2, info("P04 Fictional Switch"))?)
        .await?;
    let covering = if args.len() == 2 {
        Some(server.add_accessory(covering()?).await?)
    } else {
        None
    };
    // Values are fictional. No integration, CommandRouter, or hardware path exists.
    let simulation = async {
        let mut tick = tokio::time::interval(Duration::from_secs(5));
        let mut on = false;
        loop {
            tick.tick().await;
            on = !on;
            let mut accessory = switch.lock().await;
            accessory
                .get_mut_service(hap::HapType::Switch)
                .ok_or("missing switch")?
                .get_mut_characteristic(hap::HapType::PowerState)
                .ok_or("missing power")?
                .set_value(on.into())
                .await?;
            drop(accessory);
            if let Some(covering) = &covering {
                let mut accessory = covering.lock().await;
                let service = accessory
                    .get_mut_service(hap::HapType::WindowCovering)
                    .ok_or("missing covering")?;
                let target = service
                    .get_mut_characteristic(hap::HapType::TargetPosition)
                    .ok_or("missing target")?
                    .get_value()
                    .await?;
                service
                    .get_mut_characteristic(hap::HapType::CurrentPosition)
                    .ok_or("missing position")?
                    .set_value(target)
                    .await?;
            }
        }
        #[allow(unreachable_code)]
        Ok::<(), Box<dyn Error>>(())
    };
    tokio::select! {
        result = server.run_handle() => result?,
        result = simulation => result?,
        result = tokio::signal::ctrl_c() => result?,
    }
    // Dropping futures is an experiment cancellation path, not proven graceful HAP shutdown.
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use hap::characteristic::HapCharacteristic;

    #[tokio::test]
    async fn fictional_covering_reads_and_writes() {
        let mut covering = covering().unwrap();
        covering
            .window_covering
            .target_position
            .set_value(50.into())
            .await
            .unwrap();
        assert_eq!(
            covering
                .window_covering
                .target_position
                .get_value()
                .await
                .unwrap(),
            50
        );
    }

    #[tokio::test]
    async fn fictional_switch_reads_and_writes() {
        let mut switch = SwitchAccessory::new(2, info("P04 Fictional Switch")).unwrap();
        switch
            .switch
            .power_state
            .set_value(true.into())
            .await
            .unwrap();
        assert_eq!(switch.switch.power_state.get_value().await.unwrap(), true);
    }

    #[test]
    fn malformed_tlv_is_a_known_candidate_failure() {
        for input in [vec![6], vec![6, 2, 1]] {
            assert!(std::panic::catch_unwind(|| hap::tlv::decode(&input)).is_err());
        }
    }

    #[tokio::test]
    async fn out_of_range_is_a_known_candidate_failure() {
        let mut covering = covering().unwrap();
        // Passing means the defect is reproduced, not protocol conformance.
        assert!(
            covering
                .window_covering
                .target_position
                .set_value(101.into())
                .await
                .is_ok()
        );
    }

    #[test]
    fn corrupt_config_must_not_reset_identity() {
        assert!(parse_config(b"invalid").is_err());
    }
    #[tokio::test]
    async fn storage_restart_and_controller_removal() {
        use hap::pairing::{Pairing, Permissions};
        use std::os::unix::fs::DirBuilderExt;
        let dir = std::env::temp_dir().join(format!("p04-hap-{}", uuid::Uuid::new_v4()));
        std::fs::DirBuilder::new().mode(0o700).create(&dir).unwrap();
        let config = load_config(&dir, "127.0.0.1".parse().unwrap())
            .await
            .unwrap();
        let id = config.device_id;
        let public_key = config.device_ed25519_keypair.public.to_bytes();
        let pin = config.pin.to_string();
        let mut storage = FileStorage::new(dir.as_os_str()).await.unwrap();
        storage.save_config(&config).await.unwrap();
        let pairing = Pairing::new(uuid::Uuid::new_v4(), Permissions::Admin, public_key);
        storage.save_pairing(&pairing).await.unwrap();
        drop(storage);
        let config = load_config(&dir, "127.0.0.1".parse().unwrap())
            .await
            .unwrap();
        assert_eq!(config.device_id, id);
        assert_eq!(config.device_ed25519_keypair.public.to_bytes(), public_key);
        assert!(config.pin.to_string() == pin);
        let mut storage = FileStorage::new(dir.as_os_str()).await.unwrap();
        assert_eq!(storage.load_pairing(&pairing.id).await.unwrap(), pairing);
        storage.delete_pairing(&pairing.id).await.unwrap();
        assert_eq!(storage.count_pairings().await.unwrap(), 0);
        std::fs::write(dir.join("config.json"), b"invalid").unwrap();
        assert!(
            load_config(&dir, "127.0.0.1".parse().unwrap())
                .await
                .is_err()
        );
        std::fs::remove_file(dir.join("config.json")).unwrap();
        assert!(
            load_config(&dir, "127.0.0.1".parse().unwrap())
                .await
                .is_err()
        );
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[tokio::test]
    #[ignore = "qualification gate: upstream does not validate advertised bounds"]
    async fn qualification_rejects_out_of_range() {
        let mut covering = covering().unwrap();
        assert!(
            covering
                .window_covering
                .target_position
                .set_value(101.into())
                .await
                .is_err()
        );
    }

    #[test]
    #[ignore = "qualification gate: upstream TLV decoder panics on truncated data"]
    fn qualification_malformed_tlv_does_not_panic() {
        assert!(std::panic::catch_unwind(|| hap::tlv::decode(&[6])).is_ok());
    }
}
