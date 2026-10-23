use crate::{error::BridgeError, model::IntegrationId};
use serde::{Deserialize, Serialize};
use std::{net::IpAddr, path::PathBuf};

// No Debug or serialization: discovery includes a private gateway PIN.
#[derive(Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DiscoveredGateway {
    pub tls_dns_name: String,
    pub port: u16,
    pub expected_pin: String,
    pub resolved_ip: Option<IpAddr>,
    pub integration_id: IntegrationId,
}

// Paths reference private files; their contents are never loaded here.
#[derive(Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case", deny_unknown_fields)]
pub enum TahomaConfig {
    LocalToken {
        integration_id: IntegrationId,
        tls_dns_name: String,
        port: u16,
        expected_pin: String,
        resolved_ip: Option<IpAddr>,
        token_file: PathBuf,
        ca_file: PathBuf,
    },
    SomfyAccount {
        integration_id: IntegrationId,
        provider: SomfyProvider,
        gateway_pin: String,
        credentials_file: PathBuf,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SomfyProvider {
    SomfyEurope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HomeKitBackend {
    StopAndReview,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HomeKitConfig {
    pub name: String,
    #[serde(default = "default_hap_port")]
    pub port: u16,
    pub interface: Option<String>,
    pub state_dir: PathBuf,
    pub backend: HomeKitBackend,
}
fn validate_gateway(
    integration_id: &str,
    tls_dns_name: &str,
    port: u16,
    pin: &str,
) -> Result<(), BridgeError> {
    let valid_dns = !tls_dns_name.is_empty()
        && tls_dns_name.len() <= 253
        && tls_dns_name.parse::<IpAddr>().is_err()
        && tls_dns_name.split('.').all(|label| {
            !label.is_empty()
                && label.len() <= 63
                && !label.starts_with('-')
                && !label.ends_with('-')
                && label
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b == b'-')
        });
    if integration_id.trim().is_empty() || !valid_dns || port == 0 || pin.trim().is_empty() {
        return Err(BridgeError::Validation);
    }
    Ok(())
}

impl TahomaConfig {
    pub fn validate(&self) -> Result<(), BridgeError> {
        match self {
            Self::LocalToken {
                integration_id,
                tls_dns_name,
                port,
                expected_pin,
                token_file,
                ca_file,
                ..
            } => {
                validate_gateway(integration_id, tls_dns_name, *port, expected_pin)?;
                if token_file.as_os_str().is_empty() || ca_file.as_os_str().is_empty() {
                    return Err(BridgeError::Validation);
                }
            }
            Self::SomfyAccount {
                integration_id,
                gateway_pin,
                credentials_file,
                ..
            } => {
                if integration_id.trim().is_empty()
                    || gateway_pin.trim().is_empty()
                    || credentials_file.as_os_str().is_empty()
                {
                    return Err(BridgeError::Validation);
                }
            }
        }
        Ok(())
    }
}
impl DiscoveredGateway {
    pub fn validate(&self) -> Result<(), BridgeError> {
        validate_gateway(
            &self.integration_id,
            &self.tls_dns_name,
            self.port,
            &self.expected_pin,
        )
    }
}
impl HomeKitConfig {
    pub fn validate(&self) -> Result<(), BridgeError> {
        if self.name.trim().is_empty()
            || self.port == 0
            || self.state_dir.as_os_str().is_empty()
            || self.interface.as_ref().is_some_and(|v| v.trim().is_empty())
        {
            return Err(BridgeError::Validation);
        }
        Ok(())
    }
}

fn default_hap_port() -> u16 {
    51827
}
fn default_listen() -> std::net::SocketAddr {
    std::net::SocketAddr::from(([127, 0, 0, 1], 8582))
}

// This is a private on-disk configuration, never a public API response.
#[derive(Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AppConfig {
    pub schema_version: u32,
    #[serde(default = "default_listen")]
    pub listen: std::net::SocketAddr,
    pub data_dir: PathBuf,
    pub tahoma: TahomaConfig,
    pub homekit: HomeKitConfig,
}
impl AppConfig {
    pub fn validate(&self) -> Result<(), BridgeError> {
        self.tahoma.validate()?;
        self.homekit.validate()?;
        if self.schema_version != 1
            || self.listen.port() == 0
            || self.listen.port() == self.homekit.port
        {
            return Err(BridgeError::Validation);
        }
        let mut paths = vec![&self.data_dir, &self.homekit.state_dir];
        match &self.tahoma {
            TahomaConfig::LocalToken {
                token_file,
                ca_file,
                ..
            } => paths.extend([token_file, ca_file]),
            TahomaConfig::SomfyAccount {
                credentials_file, ..
            } => paths.push(credentials_file),
        };
        if paths
            .iter()
            .any(|p| p.to_str().is_none_or(|s| s.is_empty() || s.contains("://")))
        {
            return Err(BridgeError::Validation);
        }
        Ok(())
    }
}
/// Parse and validate only; referenced files and network endpoints are not accessed.
pub fn load_config(path: &std::path::Path) -> Result<AppConfig, BridgeError> {
    let bytes = crate::secrets::read_private(path, 1024 * 1024)?;
    let config: AppConfig = serde_json::from_slice(&bytes).map_err(|_| BridgeError::Validation)?;
    config.validate()?;
    Ok(config)
}
/// Write private gateway selections and file references, never credential contents.
pub fn save_config_atomic(path: &std::path::Path, config: &AppConfig) -> Result<(), BridgeError> {
    config.validate()?;
    use serde_json::json;
    let tahoma = match &config.tahoma {
        TahomaConfig::LocalToken {
            integration_id,
            tls_dns_name,
            port,
            expected_pin,
            resolved_ip,
            token_file,
            ca_file,
        } => json!({
            "mode": "local_token", "integration_id": integration_id, "tls_dns_name": tls_dns_name,
            "port": port, "expected_pin": expected_pin, "resolved_ip": resolved_ip,
            "token_file": token_file, "ca_file": ca_file,
        }),
        TahomaConfig::SomfyAccount {
            integration_id,
            provider,
            gateway_pin,
            credentials_file,
        } => json!({
            "mode": "somfy_account", "integration_id": integration_id, "provider": provider,
            "gateway_pin": gateway_pin, "credentials_file": credentials_file,
        }),
    };
    let bytes = serde_json::to_vec_pretty(&json!({
        "schema_version": config.schema_version, "listen": config.listen,
        "data_dir": config.data_dir, "tahoma": tahoma, "homekit": config.homekit,
    }))
    .map_err(|_| BridgeError::Validation)?;
    if bytes.len() > 1024 * 1024 {
        return Err(BridgeError::Validation);
    }
    crate::secrets::write_private_atomic(path, &bytes)
}
