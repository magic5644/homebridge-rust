mod support;
use bridge_core::{config::TahomaConfig, error::BridgeError};
use integration_tahoma::{TahomaClient, TahomaCredentials};
use support::*;
fn credentials() -> TahomaCredentials {
    TahomaCredentials::LocalToken {
        token: "fictional-token".into(),
    }
}
#[tokio::test]
async fn wrong_ca_and_wrong_certificate_name_fail_before_bearer_transmission() {
    let server = Server::start(vec![response(200, SETUP)]).await;
    let mut ca = server.config();
    if let TahomaConfig::LocalToken { ca_file, .. } = &mut ca {
        *ca_file =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("certs/overkiz-root-ca-2048.pem");
    }
    assert_eq!(
        TahomaClient::connect(ca, credentials()).await.err(),
        Some(BridgeError::Tls)
    );
    let mut name = server.config();
    if let TahomaConfig::LocalToken {
        tls_dns_name,
        expected_pin,
        ..
    } = &mut name
    {
        *tls_dns_name = "gateway-3333-4444-5555.local".into();
        *expected_pin = "3333-4444-5555".into();
    }
    assert_eq!(
        TahomaClient::connect(name, credentials()).await.err(),
        Some(BridgeError::Tls)
    );
    assert_eq!(server.count(), 0);
}
#[tokio::test]
async fn configured_pin_must_equal_canonical_dns_identity() {
    let server = Server::start(vec![response(200, SETUP)]).await;
    let mut config = server.config();
    if let TahomaConfig::LocalToken { tls_dns_name, .. } = &mut config {
        *tls_dns_name = "api.test".into()
    }
    assert_eq!(
        TahomaClient::connect(config, credentials()).await.err(),
        Some(BridgeError::Validation)
    );
    assert_eq!(server.count(), 0);
}
#[tokio::test]
async fn redirects_do_not_send_the_token_to_the_destination() {
    let target = Server::start(vec![]).await;
    let redirect=format!("HTTP/1.1 302 Found\r\nLocation: https://gateway-0000-1111-2222.local:{}/stolen\r\nContent-Length: 0\r\n\r\n",target.port).into_bytes();
    let source = Server::start(vec![redirect]).await;
    assert_eq!(
        TahomaClient::connect(source.config(), credentials())
            .await
            .err(),
        Some(BridgeError::Upstream)
    );
    assert_eq!(source.count(), 1);
    assert_eq!(target.count(), 0);
}
#[test]
fn proxy_settings_are_ignored_in_an_isolated_process() {
    if std::env::var_os("P11_PROXY_CHILD").is_some() {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let server = Server::start(vec![response(200, SETUP)]).await;
                assert!(
                    TahomaClient::connect(server.config(), credentials())
                        .await
                        .is_ok()
                );
                assert_eq!(server.count(), 1);
            });
        return;
    }
    let result = std::process::Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "proxy_settings_are_ignored_in_an_isolated_process",
        ])
        .env("P11_PROXY_CHILD", "1")
        .env("HTTPS_PROXY", "http://127.0.0.1:1")
        .env("https_proxy", "http://127.0.0.1:1")
        .env("ALL_PROXY", "http://127.0.0.1:1")
        .env("all_proxy", "http://127.0.0.1:1")
        .env("NO_PROXY", "")
        .env("no_proxy", "")
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
}

#[tokio::test]
async fn refused_connection_is_unavailable_rather_than_a_certificate_error() {
    let server = Server::start(vec![]).await;
    let config = server.config();
    drop(server);
    tokio::task::yield_now().await;
    assert_eq!(
        TahomaClient::connect(config, credentials()).await.err(),
        Some(BridgeError::Unavailable)
    );
}
