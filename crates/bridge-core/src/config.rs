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
