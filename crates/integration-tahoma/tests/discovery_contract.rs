// Compile the private discovery seam without exposing network test hooks as public API.
#[path = "../src/discovery.rs"]
mod discovery;

use bridge_core::{config::TahomaConfig, error::BridgeError};
use mdns_sd::{ServiceEvent, ServiceInfo};
use std::{cell::Cell, collections::VecDeque, future::pending, time::Duration};

const PIN: &str = "0000-1111-2222";

fn announcement(service: &str, name: &str, pin: Option<&str>, ip: &str) -> ServiceEvent {
    let properties: Vec<_> = pin.into_iter().map(|pin| ("gateway_pin", pin)).collect();
    ServiceEvent::ServiceResolved(Box::new(
        ServiceInfo::new(
            service,
            name,
            "gateway-0000-1111-2222.local.",
            ip,
            8443,
            properties.as_slice(),
        )
        .unwrap()
        .as_resolved_service(),
    ))
}

async fn simulate(events: Vec<ServiceEvent>) -> Vec<bridge_core::config::DiscoveredGateway> {
    let mut events = VecDeque::from(events);
    let stopped = Cell::new(false);
    let result = discovery::collect(
        Duration::from_secs(1),
        async || match events.pop_front() {
            Some(event) => Ok(event),
            None => pending().await,
        },
        || {
            stopped.set(true);
            Ok(())
        },
    )
    .await
    .unwrap();
    assert!(stopped.get());
    result
}

#[tokio::test(start_paused = true)]
async fn txt_identifies_gateway_and_names_do_not() {
    let gateways = simulate(vec![announcement(
        discovery::SERVICE_TYPE,
        "arbitrary",
        Some(PIN),
        "192.0.2.1",
    )])
    .await;
    assert_eq!(gateways.len(), 1);
    let gateway = &gateways[0];
    assert_eq!(gateway.expected_pin, PIN);
    assert_eq!(gateway.tls_dns_name, "gateway-0000-1111-2222.local");
    assert_eq!(gateway.port, 8443);
    assert_eq!(gateway.resolved_ip.unwrap().to_string(), "192.0.2.1");
    assert_eq!(gateway.integration_id, "tahoma-main");
    gateway.validate().unwrap();

    let impostor = simulate(vec![announcement(
        discovery::SERVICE_TYPE,
        "gateway-0000-1111-2222",
        Some("0000-3333-4444"),
        "192.0.2.2",
    )])
    .await;
    assert!(
        !impostor
            .iter()
            .any(|candidate| candidate.expected_pin == PIN)
    );
    assert_eq!(impostor[0].tls_dns_name, "gateway-0000-3333-4444.local");
}

#[tokio::test(start_paused = true)]
async fn ignores_other_services_missing_and_invalid_txt() {
    let gateways = simulate(vec![
        announcement("_http._tcp.local.", "name", Some(PIN), "192.0.2.1"),
        announcement(discovery::SERVICE_TYPE, "name", None, "192.0.2.1"),
        announcement(
            discovery::SERVICE_TYPE,
            "name",
            Some("attacker.example"),
            "192.0.2.1",
        ),
        announcement(
            discovery::SERVICE_TYPE,
            "name",
            Some("0000-1111-2222 "),
            "192.0.2.1",
        ),
    ])
    .await;
    assert!(gateways.is_empty());
}

#[tokio::test(start_paused = true)]
async fn deduplicates_interfaces_by_txt_pin_with_deterministic_address() {
    let gateways = simulate(vec![
        announcement(discovery::SERVICE_TYPE, "ethernet", Some(PIN), "192.0.2.2"),
        announcement(discovery::SERVICE_TYPE, "wifi", Some(PIN), "192.0.2.1"),
    ])
    .await;
    assert_eq!(gateways.len(), 1);
    assert_eq!(gateways[0].resolved_ip.unwrap().to_string(), "192.0.2.1");
}

#[tokio::test(start_paused = true)]
async fn timeout_returns_empty_and_stops_source() {
    assert!(simulate(vec![]).await.is_empty());
    assert!(
        discovery::discover(Duration::ZERO)
            .await
            .unwrap()
            .is_empty()
    );
    assert!(
        integration_tahoma::discover(Duration::ZERO)
            .await
            .unwrap()
            .is_empty()
    );
}

#[tokio::test(start_paused = true)]
async fn error_and_cancellation_stop_source() {
    let stopped = Cell::new(0);
    let result = discovery::collect(
        Duration::from_secs(1),
        async || Err(BridgeError::Unavailable),
        || {
            stopped.set(stopped.get() + 1);
            Ok(())
        },
    )
    .await;
    assert!(matches!(result, Err(BridgeError::Unavailable)));
    assert_eq!(stopped.get(), 1);
    let future = discovery::collect(
        Duration::from_secs(1),
        async || pending().await,
        || {
            stopped.set(stopped.get() + 1);
            Ok(())
        },
    );
    let mut future = Box::pin(future);
    assert!(
        std::future::poll_fn(|cx| match future.as_mut().poll(cx) {
            std::task::Poll::Pending => std::task::Poll::Ready(true),
            std::task::Poll::Ready(_) => std::task::Poll::Ready(false),
        })
        .await
    );
    drop(future);
    assert_eq!(stopped.get(), 2);
}

#[test]
fn manual_address_preserves_configured_tls_dns_identity() {
    let config = TahomaConfig::LocalToken {
        integration_id: "tahoma-main".into(),
        tls_dns_name: "gateway-0000-1111-2222.local".into(),
        port: 8443,
        expected_pin: PIN.into(),
        resolved_ip: Some("192.0.2.99".parse().unwrap()),
        token_file: "token".into(),
        ca_file: "ca.pem".into(),
    };
    config.validate().unwrap();
    let TahomaConfig::LocalToken {
        tls_dns_name,
        resolved_ip,
        ..
    } = config
    else {
        panic!()
    };
    assert_eq!(tls_dns_name, "gateway-0000-1111-2222.local");
    assert_eq!(resolved_ip.unwrap().to_string(), "192.0.2.99");
}

#[tokio::test(start_paused = true)]
async fn rejects_unbounded_duration_without_network_and_reports_shutdown_failure() {
    assert!(matches!(
        discovery::discover(Duration::MAX).await,
        Err(BridgeError::Validation)
    ));
    let result = discovery::collect(
        Duration::ZERO,
        async || pending().await,
        || Err(BridgeError::Unavailable),
    )
    .await;
    assert!(matches!(result, Err(BridgeError::Unavailable)));
}

#[tokio::test(start_paused = true)]
async fn bounds_results_and_rejects_conflicting_ports() {
    let mut events: VecDeque<_> = (0..65)
        .map(|index| {
            announcement(
                discovery::SERVICE_TYPE,
                &format!("fictional-{index}"),
                Some(&format!("0000-1111-{index:04}")),
                "192.0.2.1",
            )
        })
        .collect();
    let stopped = Cell::new(false);
    let result = discovery::collect(
        Duration::from_secs(1),
        async || Ok(events.pop_front().unwrap()),
        || {
            stopped.set(true);
            Ok(())
        },
    )
    .await;
    assert!(matches!(result, Err(BridgeError::Busy)));
    assert!(stopped.get());

    let first = announcement(discovery::SERVICE_TYPE, "first", Some(PIN), "192.0.2.1");
    let mut second = announcement(discovery::SERVICE_TYPE, "second", Some(PIN), "192.0.2.2");
    if let ServiceEvent::ServiceResolved(service) = &mut second {
        service.port = 8444;
    }
    let mut events = VecDeque::from([first, second]);
    let result = discovery::collect(
        Duration::from_secs(1),
        async || Ok(events.pop_front().unwrap()),
        || Ok(()),
    )
    .await;
    assert!(matches!(result, Err(BridgeError::Conflict)));
}

#[tokio::test(start_paused = true)]
async fn rejects_zero_port_and_preserves_dns_for_scoped_ipv6() {
    let mut zero_port = announcement(discovery::SERVICE_TYPE, "name", Some(PIN), "192.0.2.1");
    if let ServiceEvent::ServiceResolved(service) = &mut zero_port {
        service.port = 0;
    }
    assert!(simulate(vec![zero_port]).await.is_empty());
    let gateways = simulate(vec![announcement(
        discovery::SERVICE_TYPE,
        "ipv6",
        Some(PIN),
        "fe80::1234",
    )])
    .await;
    assert_eq!(gateways.len(), 1);
    assert!(gateways[0].resolved_ip.is_none());
    assert_eq!(gateways[0].tls_dns_name, "gateway-0000-1111-2222.local");
}
