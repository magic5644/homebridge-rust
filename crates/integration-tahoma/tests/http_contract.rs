mod support;
use bridge_core::error::BridgeError;
use integration_tahoma::{TahomaClient, TahomaCredentials};
use support::*;
fn credentials() -> TahomaCredentials {
    TahomaCredentials::LocalToken {
        token: "fictional-token".into(),
    }
}

#[tokio::test]
async fn authenticated_setup_tolerates_extra_fields_without_inventing_capabilities() {
    let server = Server::start(vec![response(200, SETUP), response(200, SETUP)]).await;
    let client = TahomaClient::connect(server.config(), credentials())
        .await
        .unwrap();
    let devices = client.setup().await.unwrap();
    assert_eq!(devices.len(), 1);
    assert_eq!(devices[0].external_id, "io://0000-1111-2222/1234");
    assert!(devices[0].capabilities.is_empty());
    assert!(devices[0].states.is_empty());
    for req in server.requests.lock().unwrap().iter() {
        assert!(req.starts_with("GET /enduser-mobile-web/1/enduserAPI/setup HTTP/1.1"));
        assert!(req.contains("authorization: Bearer fictional-token\r\n"));
    }
}
#[tokio::test]
async fn invalid_token_has_a_redacted_dedicated_error() {
    let server = Server::start(vec![response(401, "fictional-token")]).await;
    let err = TahomaClient::connect(server.config(), credentials())
        .await
        .err()
        .unwrap();
    assert_eq!(err, BridgeError::UpstreamAuthentication);
    assert!(!format!("{err:?} {err}").contains("fictional-token"));
    assert_eq!(server.count(), 1);
}
#[tokio::test]
async fn malformed_missing_identifiers_truncated_and_oversized_bodies_fail() {
    for bytes in [
        response(200, "{}"),
        response(200, "{\"gateways\":[],\"devices\":[{}]}"),
        b"HTTP/1.1 200 OK\r\nContent-Length: 999\r\n\r\n{}".to_vec(),
        response(200, &" ".repeat(4 * 1024 * 1024 + 1)),
        format!(
            "HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n{:x}\r\n{}\r\n0\r\n\r\n",
            4 * 1024 * 1024 + 1,
            " ".repeat(4 * 1024 * 1024 + 1)
        )
        .into_bytes(),
    ] {
        let server = Server::start(vec![bytes]).await;
        assert_eq!(
            TahomaClient::connect(server.config(), credentials())
                .await
                .err(),
            Some(BridgeError::Upstream)
        );
        assert_eq!(server.count(), 1);
    }
}
#[tokio::test]
async fn listener_ids_are_one_segment_and_consuming_fetch_is_never_replayed() {
    let server = Server::start(vec![
        response(200, SETUP),
        response(200, r#"{"id":"a/b?c"}"#),
        b"HTTP/1.1 200 OK\r\nContent-Length: 30\r\n\r\n[".to_vec(),
    ])
    .await;
    let client = TahomaClient::connect(server.config(), credentials())
        .await
        .unwrap();
    let id = client.register_listener().await.unwrap();
    assert_eq!(id, "a/b?c");
    assert_eq!(
        client.fetch_events(&id).await.err(),
        Some(BridgeError::Upstream)
    );
    let requests = server.requests.lock().unwrap();
    assert_eq!(requests.len(), 3);
    assert!(
        requests[2].starts_with("POST /enduser-mobile-web/1/enduserAPI/events/a%2Fb%3Fc/fetch ")
    );
}
#[tokio::test]
async fn foreign_gateway_is_rejected_not_substring_matched() {
    let setup = SETUP.replace("0000-1111-2222", "0000-1111-22220");
    let server = Server::start(vec![response(200, &setup)]).await;
    assert_eq!(
        TahomaClient::connect(server.config(), credentials())
            .await
            .err(),
        Some(BridgeError::Validation)
    );
}

#[tokio::test]
async fn raw_setup_retains_actual_definition_metadata() {
    let setup=SETUP.replace("\"commands\":[]", "\"commands\":[{\"commandName\":\"up\",\"nparams\":0}],\"uiClass\":\"RollerShutter\",\"widgetName\":\"PositionableRollerShutter\",\"type\":\"ACTUATOR\"");
    let server = Server::start(vec![response(200, &setup), response(200, &setup)]).await;
    let client = TahomaClient::connect(server.config(), credentials())
        .await
        .unwrap();
    let raw = client.raw_setup().await.unwrap().remove(0);
    assert_eq!(
        raw.definition.widget.as_deref(),
        Some("PositionableRollerShutter")
    );
    assert_eq!(raw.definition.type_name.as_deref(), Some("ACTUATOR"));
    assert_eq!(raw.definition.commands[0].command_name, "up");
}
#[tokio::test]
async fn events_unregister_execution_states_and_lost_commands_follow_documented_routes() {
    use bridge_core::model::CommandStatus;
    use integration_tahoma::dto::{TahomaAction, TahomaActionGroup, TahomaCommand};
    let server=Server::start(vec![response(200,SETUP),response(200,r#"[{"name":"DeviceStateChangedEvent","deviceURL":"io://0000-1111-2222/1234","deviceStates":[{"name":"core:ClosureState","type":1,"value":50}],"timestamp":10},{"name":"FutureEvent","extra":true},{"name":"DeviceUnavailableEvent","deviceURL":"io://9999-9999-9999/0"}]"#),response(204,""),
        response(200,r#"{"id":"x/y","state":"FUTURE_STATE"}"#),response(200,r#"{"id":"done","state":"COMPLETED"}"#),response(404,"{}"),
        b"HTTP/1.1 200 OK\r\nContent-Length: 99\r\n\r\n{".to_vec()]).await;
    let client = TahomaClient::connect(server.config(), credentials())
        .await
        .unwrap();
    let events = client.fetch_events("listener").await.unwrap();
    assert_eq!(events.len(), 2);
    assert_eq!(
        events[0].device_states.as_ref().unwrap()[0].value,
        Some(50.into())
    );
    assert_eq!(events[1].name, "FutureEvent");
    client.unregister_listener("listener").await.unwrap();
    assert_eq!(
        client.execution("x/y").await.unwrap().status,
        CommandStatus::Unknown
    );
    assert_eq!(
        client.execution("done").await.unwrap().status,
        CommandStatus::Succeeded
    );
    assert_eq!(
        client.execution("gone").await.err(),
        Some(BridgeError::Upstream)
    );
    let group = TahomaActionGroup {
        actions: vec![TahomaAction {
            device_url: "io://0000-1111-2222/1234".into(),
            commands: vec![TahomaCommand {
                name: "fixtureDeclaredCommand".into(),
                parameters: vec![],
            }],
        }],
        label: None,
    };
    assert_eq!(
        client.apply_raw(&group).await.err(),
        Some(BridgeError::Upstream)
    );
    let requests = server.requests.lock().unwrap();
    assert_eq!(requests.len(), 7);
    assert!(
        requests[2]
            .starts_with("POST /enduser-mobile-web/1/enduserAPI/events/listener/unregister ")
    );
    assert!(requests[3].starts_with("GET /enduser-mobile-web/1/enduserAPI/exec/current/x%2Fy "));
    assert!(requests[6].starts_with("POST /enduser-mobile-web/1/enduserAPI/exec/apply "));
    let body = requests[6].split("\r\n\r\n").nth(1).unwrap();
    let body: serde_json::Value = serde_json::from_str(body).unwrap();
    assert_eq!(body["actions"][0]["deviceURL"], "io://0000-1111-2222/1234");
    assert_eq!(
        body["actions"][0]["commands"][0]["name"],
        "fixtureDeclaredCommand"
    );
}
#[tokio::test]
async fn mixed_credentials_fail_before_network_or_secret_file_access() {
    let server = Server::start(vec![]).await;
    let mixed = TahomaCredentials::SomfyAccount {
        username: "fictional".into(),
        password: "fictional".into(),
    };
    assert_eq!(
        TahomaClient::connect(server.config(), mixed).await.err(),
        Some(BridgeError::Validation)
    );
    assert_eq!(server.count(), 0);
}

#[tokio::test]
async fn malformed_known_event_fields_are_errors() {
    for event in [
        r#"[{"name":"DeviceStateChangedEvent","deviceStates":[]}]"#,
        r#"[{"name":"ExecutionStateChangedEvent","execId":"x","newState":"COMPLETED"}]"#,
        r#"[{"name":"DeviceStateChangedEvent","deviceURL":"io://0000-1111-2222/1234","deviceStates":[{"name":""}]}]"#,
    ] {
        let server = Server::start(vec![response(200, SETUP), response(200, event)]).await;
        let client = TahomaClient::connect(server.config(), credentials())
            .await
            .unwrap();
        assert_eq!(
            client.fetch_events("listener").await.err(),
            Some(BridgeError::Upstream)
        );
    }
}
#[tokio::test]
async fn foreign_command_targets_fail_without_transmission() {
    use integration_tahoma::dto::{TahomaAction, TahomaActionGroup, TahomaCommand};
    let server = Server::start(vec![response(200, SETUP)]).await;
    let client = TahomaClient::connect(server.config(), credentials())
        .await
        .unwrap();
    let group = TahomaActionGroup {
        actions: vec![TahomaAction {
            device_url: "io://0000-1111-22220/0".into(),
            commands: vec![TahomaCommand {
                name: "fixture".into(),
                parameters: vec![],
            }],
        }],
        label: None,
    };
    assert_eq!(
        client.apply_raw(&group).await.err(),
        Some(BridgeError::Validation)
    );
    assert_eq!(server.count(), 1);
}

#[tokio::test]
async fn slow_setup_is_stopped_by_the_five_second_total_http_deadline() {
    use std::time::{Duration, Instant};
    let server = Server::delayed(vec![response(200, SETUP)], Duration::from_secs(30)).await;
    let started = Instant::now();
    let result = tokio::time::timeout(
        Duration::from_secs(10),
        TahomaClient::connect(server.config(), credentials()),
    )
    .await;
    assert_eq!(result.unwrap().err(), Some(BridgeError::Upstream));
    assert!(started.elapsed() >= Duration::from_secs(4));
    assert!(started.elapsed() < Duration::from_secs(9));
    assert_eq!(server.count(), 1);
}
