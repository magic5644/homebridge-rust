use super::*;
#[path = "mod.rs"]
mod fixture;
use fixture::{PIN, SETUP, Server, response};
async fn account(server: &Server) -> Result<TahomaClient, BridgeError> {
    let http = builder()
        .tls_certs_only([
            reqwest::Certificate::from_pem(include_bytes!("../fixtures/tls/ca.pem")).unwrap(),
        ])
        .resolve("account.test", ([127, 0, 0, 1], server.port).into())
        .resolve("api.test", ([127, 0, 0, 1], server.port).into())
        .build()
        .unwrap();
    TahomaClient::account(
        http,
        Url::parse(&format!("https://api.test:{}/enduserAPI/", server.port)).unwrap(),
        Url::parse(&format!(
            "https://account.test:{}/oauth/oauth/v2/token/jwt",
            server.port
        ))
        .unwrap(),
        PIN.into(),
        "fixture-user".into(),
        "fixture-password".into(),
    )
    .await
}
const TOKEN: &str = r#"{"access_token":"fixture-access","refresh_token":"fixture-refresh","expires_in":3600,"token_type":"Bearer","future":true}"#;
async fn expire(client: &TahomaClient) {
    if let Auth::Account { session, .. } = &*client.auth {
        session.lock().await.expires_at = Instant::now();
    }
}
#[tokio::test]
async fn password_grant_selected_gateway_and_serialized_rotating_refresh() {
    let setup=SETUP.replace("\"devices\":[", "\"devices\":[{\"deviceURL\":\"io://9999-9999-9999/0\",\"available\":true,\"enabled\":true,\"label\":\"Other\",\"controllableName\":\"io:Other\",\"states\":[],\"definition\":{\"commands\":[],\"states\":[]}},");
    let server = Server::start(vec![
        response(200, TOKEN),
        response(200, &setup),
        response(
            200,
            r#"{"access_token":"new-access","refresh_token":"rotated-refresh","expires_in":3600}"#,
        ),
        response(200, &setup),
        response(200, &setup),
        response(200, r#"{"access_token":"last-access","expires_in":3600}"#),
        response(200, SETUP),
    ])
    .await;
    let client = account(&server).await.unwrap();
    expire(&client).await;
    let (a, b) = tokio::join!(client.raw_setup(), client.raw_setup());
    assert_eq!(a.unwrap().len(), 1);
    assert_eq!(b.unwrap().len(), 1);
    expire(&client).await;
    client.raw_setup().await.unwrap();
    let requests = server.requests.lock().unwrap();
    assert_eq!(requests.len(), 7);
    assert!(requests[0].starts_with("POST /oauth/oauth/v2/token/jwt "));
    assert!(requests[0].contains("grant_type=password"));
    assert!(requests[0].contains("username=fixture-user"));
    assert!(requests[0].contains("password=fixture-password"));
    assert!(requests[0].contains("application/x-www-form-urlencoded"));
    assert!(!requests[0].contains("authorization:"));
    assert!(requests[1].contains("authorization: Bearer fixture-access"));
    assert!(requests[2].contains("grant_type=refresh_token"));
    assert!(requests[2].contains("refresh_token=fixture-refresh"));
    assert!(!requests[2].contains("password="));
    assert!(requests[3].contains("authorization: Bearer new-access"));
    assert!(requests[5].contains("refresh_token=rotated-refresh"));
    assert!(
        !requests
            .iter()
            .any(|r| r.contains("/login") || r.contains("Cookie:"))
    );
}
#[tokio::test]
async fn bad_credentials_expiry_and_malformed_tokens_are_redacted_without_fallback() {
    for (status, body, expected) in [
        (
            400,
            r#"{"message":"error.invalid.grant"}"#,
            BridgeError::UpstreamAuthentication,
        ),
        (401, "fixture-password", BridgeError::UpstreamAuthentication),
        (503, "<html>service</html>", BridgeError::Upstream),
        (
            200,
            r#"{"access_token":"","expires_in":3600}"#,
            BridgeError::Upstream,
        ),
        (
            200,
            r#"{"access_token":"x","expires_in":0}"#,
            BridgeError::Upstream,
        ),
        (
            200,
            r#"{"access_token":"x","expires_in":3600,"token_type":"MAC"}"#,
            BridgeError::Upstream,
        ),
    ] {
        let server = Server::start(vec![response(status, body)]).await;
        let error = account(&server).await.err().unwrap();
        assert_eq!(error, expected);
        assert!(!format!("{error:?} {error}").contains("fixture-password"));
        assert_eq!(server.count(), 1);
    }
    let server = Server::start(vec![
        response(200, r#"{"access_token":"access","expires_in":3600}"#),
        response(200, SETUP),
    ])
    .await;
    let client = account(&server).await.unwrap();
    expire(&client).await;
    assert_eq!(
        client.raw_setup().await.err(),
        Some(BridgeError::UpstreamAuthentication)
    );
    assert_eq!(server.count(), 2);
}
#[tokio::test]
async fn failed_refresh_or_unauthorized_consuming_request_never_replays() {
    let server = Server::start(vec![
        response(200, TOKEN),
        response(200, SETUP),
        response(401, "secret"),
    ])
    .await;
    let client = account(&server).await.unwrap();
    expire(&client).await;
    assert_eq!(
        client.fetch_events("listener").await.err(),
        Some(BridgeError::UpstreamAuthentication)
    );
    assert_eq!(server.count(), 3);
    assert!(server.requests.lock().unwrap()[2].contains("grant_type=refresh_token"));
    let server = Server::start(vec![
        response(200, TOKEN),
        response(200, SETUP),
        response(401, "secret"),
    ])
    .await;
    let client = account(&server).await.unwrap();
    assert_eq!(
        client.fetch_events("listener").await.err(),
        Some(BridgeError::UpstreamAuthentication)
    );
    assert_eq!(server.count(), 3);
    assert!(
        server.requests.lock().unwrap()[2].starts_with("POST /enduserAPI/events/listener/fetch ")
    );
}
#[tokio::test]
async fn account_requires_the_selected_gateway() {
    let server = Server::start(vec![
        response(200, TOKEN),
        response(200, &SETUP.replace(PIN, "9999-9999-9999")),
    ])
    .await;
    assert_eq!(account(&server).await.err(), Some(BridgeError::Validation));
    assert_eq!(server.count(), 2);
}

#[tokio::test]
async fn near_expiry_access_is_renewed_before_any_api_request() {
    let server = Server::start(vec![
        response(200, &TOKEN.replace("3600", "1")),
        response(200, TOKEN),
        response(200, SETUP),
    ])
    .await;
    account(&server).await.unwrap();
    let requests = server.requests.lock().unwrap();
    assert_eq!(requests.len(), 3);
    assert!(requests[0].contains("grant_type=password"));
    assert!(requests[1].contains("grant_type=refresh_token"));
    assert!(requests[2].starts_with("GET /enduserAPI/setup "));
}
