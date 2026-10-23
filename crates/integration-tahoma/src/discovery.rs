//! Discover untrusted candidates using the documented developer service only.
use bridge_core::{config::DiscoveredGateway, error::BridgeError};
use mdns_sd::{DaemonStatus, ResolvedService, ServiceDaemon, ServiceEvent};
use std::{collections::BTreeMap, net::IpAddr, time::Duration};
use tokio::time::{Instant, timeout_at};

pub const SERVICE_TYPE: &str = "_kizboxdev._tcp.local.";
const MAX_TIMEOUT: Duration = Duration::from_secs(60);
const MAX_GATEWAYS: usize = 64;
const SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(1);

/// Browse for at most 60 seconds; zero returns immediately without networking.
///
/// Candidates are not authenticated. Before use, match `expected_pin` against the
/// configured PIN, bind the configured integration ID, and verify TLS in P11.
/// `tahoma-main` is a neutral placeholder, never an identity derived from a PIN.
/// Cancellation and completion request daemon shutdown and wait at most one second
/// synchronously for its acknowledgement, including when the future is dropped.
pub async fn discover(timeout: Duration) -> Result<Vec<DiscoveredGateway>, BridgeError> {
    validate_timeout(timeout)?;
    if timeout.is_zero() {
        return Ok(vec![]);
    }
    let daemon = ServiceDaemon::new().map_err(|_| BridgeError::Unavailable)?;
    let mut cleanup = Cleanup(Some(|| shutdown(&daemon)));
    let events = daemon
        .browse(SERVICE_TYPE)
        .map_err(|_| BridgeError::Unavailable)?;
    collect(
        timeout,
        async || {
            events
                .recv_async()
                .await
                .map_err(|_| BridgeError::Unavailable)
        },
        || cleanup.finish(),
    )
    .await
}

fn shutdown(daemon: &ServiceDaemon) -> Result<(), BridgeError> {
    let status = daemon.shutdown().map_err(|_| BridgeError::Unavailable)?;
    match status.recv_timeout(SHUTDOWN_TIMEOUT) {
        Ok(DaemonStatus::Shutdown) => Ok(()),
        _ => Err(BridgeError::Unavailable),
    }
}

struct Cleanup<F: FnOnce() -> Result<(), BridgeError>>(Option<F>);
impl<F: FnOnce() -> Result<(), BridgeError>> Cleanup<F> {
    fn finish(&mut self) -> Result<(), BridgeError> {
        self.0.take().map_or(Ok(()), |shutdown| shutdown())
    }
}
impl<F: FnOnce() -> Result<(), BridgeError>> Drop for Cleanup<F> {
    fn drop(&mut self) {
        let _ = self.finish();
    }
}

fn validate_timeout(timeout: Duration) -> Result<(), BridgeError> {
    if timeout > MAX_TIMEOUT {
        Err(BridgeError::Validation)
    } else {
        Ok(())
    }
}

pub(crate) async fn collect(
    timeout: Duration,
    mut next: impl AsyncFnMut() -> Result<ServiceEvent, BridgeError>,
    shutdown: impl FnOnce() -> Result<(), BridgeError>,
) -> Result<Vec<DiscoveredGateway>, BridgeError> {
    let mut cleanup = Cleanup(Some(shutdown));
    validate_timeout(timeout)?;
    let deadline = Instant::now() + timeout;
    let result = async {
        let mut gateways: BTreeMap<String, DiscoveredGateway> = BTreeMap::new();
        while Instant::now() < deadline {
            let event = match timeout_at(deadline, next()).await {
                Ok(event) => event?,
                Err(_) => break,
            };
            if let ServiceEvent::ServiceResolved(service) = event
                && let Some(gateway) = candidate(&service)
            {
                if let Some(existing) = gateways.get_mut(&gateway.expected_pin) {
                    if existing.port != gateway.port {
                        return Err(BridgeError::Conflict);
                    }
                    existing.resolved_ip = existing
                        .resolved_ip
                        .into_iter()
                        .chain(gateway.resolved_ip)
                        .min();
                } else {
                    if gateways.len() == MAX_GATEWAYS {
                        return Err(BridgeError::Busy);
                    }
                    gateways.insert(gateway.expected_pin.clone(), gateway);
                }
            }
            tokio::task::yield_now().await;
        }
        Ok(gateways.into_values().collect())
    }
    .await;
    let stopped = cleanup.finish();
    result.and_then(|gateways| stopped.map(|()| gateways))
}

fn candidate(service: &ResolvedService) -> Option<DiscoveredGateway> {
    if !service.ty_domain.eq_ignore_ascii_case(SERVICE_TYPE) {
        return None;
    }
    let pin = service.get_property_val_str("gateway_pin")?;
    if pin.len() != 14
        || !pin.bytes().enumerate().all(|(index, byte)| {
            if index == 4 || index == 9 {
                byte == b'-'
            } else {
                byte.is_ascii_digit()
            }
        })
    {
        return None;
    }
    // Somfy OpenAPI specifies gateway-{pin}.local; advertised names do not prove identity.
    let gateway = DiscoveredGateway {
        tls_dns_name: format!("gateway-{pin}.local"),
        port: service.port,
        expected_pin: pin.to_owned(),
        resolved_ip: service
            .addresses
            .iter()
            .map(|ip| ip.to_ip_addr())
            .filter(usable_address)
            .min(),
        integration_id: "tahoma-main".into(),
    };
    gateway.validate().ok()?;
    Some(gateway)
}

fn usable_address(ip: &IpAddr) -> bool {
    !ip.is_unspecified() && !ip.is_multicast() && !ip.is_loopback()
        // IpAddr cannot retain an IPv6 link-local interface scope; let DNS resolve it.
        && !matches!(ip, IpAddr::V6(ip) if ip.is_unicast_link_local())
}
