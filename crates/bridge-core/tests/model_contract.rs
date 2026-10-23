use bridge_core::{config::*, error::BridgeError, event::*, model::*};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, json};
use std::collections::BTreeMap;

fn device(capabilities: Vec<Capability>) -> Device {
    Device {
        id: DeviceId::from_u128(1),
        integration_id: "fictional".into(),
        external_id: "fictional-device".into(),
        name: "Example".into(),
        capabilities,
        limits: BTreeMap::new(),
        ownership: Ownership::Native,
        available: true,
        states: BTreeMap::new(),
    }
}

#[test]
fn position_bounds() {
    let d = device(vec![Capability::Position]);
    for position_pct in [0, 100] {
        assert_eq!(
            Command::SetPosition { position_pct }.validate_for(&d),
            Ok(())
        );
    }
    assert_eq!(
        Command::SetPosition { position_pct: 101 }.validate_for(&d),
        Err(BridgeError::Validation)
    );
    assert_eq!(
        Command::SetBrightness { percent: 101 }.validate(),
        Err(BridgeError::Validation)
    );
    for degrees in [-91, 91, i16::MIN, i16::MAX] {
        assert_eq!(
            Command::SetTilt { degrees }.validate(),
            Err(BridgeError::Validation)
        );
    }
    for degrees in [-90, 0, 90] {
        assert!(Command::SetTilt { degrees }.validate().is_ok());
    }
}

#[test]
fn unsupported_command() {
    let pairs = [
        (Command::Open, Capability::OpenCloseStop),
        (Command::Close, Capability::OpenCloseStop),
        (Command::Stop, Capability::OpenCloseStop),
        (
            Command::SetPosition { position_pct: 50 },
            Capability::Position,
        ),
        (Command::SetTilt { degrees: -30 }, Capability::Tilt),
        (Command::SetPower { on: true }, Capability::Power),
        (
            Command::SetBrightness { percent: 50 },
            Capability::Brightness,
        ),
        (
            Command::SetSetpoint {
                centi_celsius: 2100,
            },
            Capability::Setpoint,
        ),
        (Command::TriggerGarage, Capability::MomentaryGarageAction),
    ];
    for (command, capability) in pairs {
        assert!(command.validate_for(&device(vec![capability])).is_ok());
        let err = command.validate_for(&device(vec![])).unwrap_err();
        assert_eq!(err.code(), "validation_error");
        for other in [
            Capability::Temperature,
            Capability::Contact,
            Capability::Motion,
            Capability::SecurityReadOnly,
            Capability::GarageDoor,
            Capability::Obstruction,
        ] {
            assert_eq!(
                command.validate_for(&device(vec![other])),
                Err(BridgeError::Validation)
            );
        }
    }
    assert!(
        Command::Open
            .validate_for(&device(vec![Capability::MomentaryGarageAction]))
            .is_err()
    );
    assert!(
        Command::TriggerGarage
            .validate_for(&device(vec![Capability::OpenCloseStop]))
            .is_err()
    );
}

#[test]
fn numeric_limits_and_steps() {
    let mut d = device(vec![Capability::Position, Capability::Setpoint]);
    d.limits.insert(
        StateKey::PositionPct,
        NumericLimits {
            min: 10,
            max: 90,
            step: 5,
        },
    );
    for position_pct in [10, 15, 90] {
        assert!(
            Command::SetPosition { position_pct }
                .validate_for(&d)
                .is_ok()
        );
    }
    for position_pct in [0, 11, 100] {
        assert_eq!(
            Command::SetPosition { position_pct }.validate_for(&d),
            Err(BridgeError::Validation)
        );
    }
    d.limits.insert(
        StateKey::TargetTemperatureCentiCelsius,
        NumericLimits {
            min: 1600,
            max: 2600,
            step: 50,
        },
    );
    assert!(
        Command::SetSetpoint {
            centi_celsius: 2150
        }
        .validate_for(&d)
        .is_ok()
    );
    assert!(
        Command::SetSetpoint {
            centi_celsius: 2125
        }
        .validate_for(&d)
        .is_err()
    );
    for limits in [
        NumericLimits {
            min: 2,
            max: 1,
            step: 1,
        },
        NumericLimits {
            min: 0,
            max: 100,
            step: 0,
        },
        NumericLimits {
            min: 0,
            max: 100,
            step: -1,
        },
    ] {
        assert!(limits.validate().is_err());
        assert!(limits.validate_value(1).is_err());
    }
    let extreme = NumericLimits {
        min: i64::MIN,
        max: i64::MAX,
        step: 1,
    };
    assert!(extreme.validate_value(i64::MAX).is_ok());
    assert!(extreme.validate_value(i64::MIN).is_ok());
}

#[test]
fn quality_is_honest() {
    let unknown = StateSample {
        value: None,
        quality: StateQuality::Unknown,
        observed_at_ms: None,
    };
    assert!(unknown.validate().is_ok());
    assert!(
        StateSample {
            value: Some(Scalar::Integer(0)),
            ..unknown.clone()
        }
        .validate()
        .is_err()
    );
    assert!(
        StateSample {
            observed_at_ms: Some(1),
            ..unknown.clone()
        }
        .validate()
        .is_err()
    );
    for quality in [
        StateQuality::Confirmed,
        StateQuality::Estimated,
        StateQuality::Stale,
    ] {
        assert!(
            StateSample {
                quality,
                ..unknown.clone()
            }
            .validate()
            .is_err()
        );
    }
    assert!(
        StateSample {
            value: Some(Scalar::Integer(1)),
            quality: StateQuality::Confirmed,
            observed_at_ms: None
        }
        .validate()
        .is_err()
    );
    let estimate = StateSample {
        value: Some(Scalar::Integer(42)),
        quality: StateQuality::Estimated,
        observed_at_ms: Some(1000),
    };
    assert!(estimate.validate_for(StateKey::PositionPct).is_ok());
    let decoded: StateSample =
        serde_json::from_value(serde_json::to_value(&estimate).unwrap()).unwrap();
    assert_eq!(decoded, estimate);
    assert_ne!(decoded.quality, StateQuality::Confirmed);
    let mut d = device(vec![Capability::Position]);
    d.states.insert(StateKey::PositionPct, estimate);
    let before = d.clone();
    Command::SetPosition { position_pct: 100 }
        .validate_for(&d)
        .unwrap();
    assert_eq!(d, before);
    assert!(unknown.validate_for(StateKey::DoorState).is_ok());
    assert!(unknown.validate_for(StateKey::ObstructionDetected).is_ok());
}

#[test]
fn state_values_match_keys() {
    let sample = |value| StateSample {
        value: Some(value),
        quality: StateQuality::Confirmed,
        observed_at_ms: Some(1),
    };
    for (key, value) in [
        (StateKey::PositionPct, Scalar::Integer(101)),
        (StateKey::TiltDegrees, Scalar::Integer(-91)),
        (StateKey::BrightnessPct, Scalar::Integer(-1)),
        (StateKey::Power, Scalar::Integer(1)),
        (StateKey::DoorState, Scalar::Text("unknown".into())),
        (StateKey::TemperatureCentiCelsius, Scalar::Text("20".into())),
        (StateKey::ObstructionDetected, Scalar::Text("false".into())),
    ] {
        assert!(sample(value).validate_for(key).is_err());
    }
    for state in ["open", "closed", "opening", "closing", "stopped"] {
        assert!(
            sample(Scalar::Text(state.into()))
                .validate_for(StateKey::DoorState)
                .is_ok()
        );
    }
    for key in [
        StateKey::Power,
        StateKey::ContactOpen,
        StateKey::MotionDetected,
        StateKey::ObstructionDetected,
    ] {
        assert!(sample(Scalar::Bool(false)).validate_for(key).is_ok());
    }
    assert!(
        sample(Scalar::Integer(-100))
            .validate_for(StateKey::TemperatureCentiCelsius)
            .is_ok()
    );
    assert!(
        sample(Scalar::Integer(2100))
            .validate_for(StateKey::TargetTemperatureCentiCelsius)
            .is_ok()
    );
    assert!(
        sample(Scalar::Text("armed".into()))
            .validate_for(StateKey::SecurityState)
            .is_ok()
    );
}

#[test]
fn ownership_blocks_writes() {
    let mut d = device(vec![Capability::OpenCloseStop]);
    for ownership in [Ownership::Legacy, Ownership::Observe] {
        d.ownership = ownership;
        assert!(!ownership.allows_commands());
        assert!(!ownership.allows_publication());
        assert_eq!(Command::Open.validate_for(&d), Err(BridgeError::NotNative));
    }
    d.ownership = Ownership::Native;
    assert!(d.ownership.allows_commands());
    assert!(d.ownership.allows_publication());
    d.available = false;
    assert_eq!(
        Command::Open.validate_for(&d),
        Err(BridgeError::Unavailable)
    );
}

fn configs() -> Value {
    serde_json::from_str(include_str!("../../../fixtures/core/config.json")).unwrap()
}

#[test]
fn mutually_exclusive_tahoma_modes() {
    let examples = configs();
    for name in ["local_token", "somfy_account"] {
        let v = examples[name].clone();
        let config: TahomaConfig = serde_json::from_value(v.clone()).unwrap();
        assert!(config.validate().is_ok());
        for (field, value) in [
            ("token", json!("fictional-secret")),
            ("password", json!("fictional-secret")),
            ("endpoint", json!("https://invalid.example")),
            ("connection", json!({"mode":"local_token"})),
        ] {
            let mut mixed = v.clone();
            mixed[field] = value;
            assert!(serde_json::from_value::<TahomaConfig>(mixed).is_err());
        }
    }
    let mut local = examples["local_token"].clone();
    local["credentials_file"] = json!("example-account.json");
    assert!(serde_json::from_value::<TahomaConfig>(local).is_err());
    let mut account = examples["somfy_account"].clone();
    account["token_file"] = json!("example-token");
    assert!(serde_json::from_value::<TahomaConfig>(account).is_err());
    let mut account = examples["somfy_account"].clone();
    account["provider"] = json!("https://invalid.example");
    assert!(serde_json::from_value::<TahomaConfig>(account).is_err());
    assert!(serde_json::from_value::<TahomaConfig>(json!({"mode":"automatic"})).is_err());
}

#[test]
fn config_validation() {
    let examples = configs();
    for name in ["local_token", "somfy_account"] {
        for field in if name == "local_token" {
            vec![
                "integration_id",
                "tls_dns_name",
                "expected_pin",
                "token_file",
                "ca_file",
            ]
        } else {
            vec!["integration_id", "gateway_pin", "credentials_file"]
        } {
            let mut v = examples[name].clone();
            v[field] = json!("");
            assert!(
                serde_json::from_value::<TahomaConfig>(v)
                    .unwrap()
                    .validate()
                    .is_err(),
                "{field}"
            );
        }
    }
    for (field, value) in [
        ("port", json!(0)),
        ("tls_dns_name", json!("https://invalid.example/path")),
        ("tls_dns_name", json!("127.0.0.1")),
        ("tls_dns_name", json!("bad..example")),
    ] {
        let mut v = examples["local_token"].clone();
        v[field] = value;
        assert!(
            serde_json::from_value::<TahomaConfig>(v)
                .unwrap()
                .validate()
                .is_err()
        );
    }
    let mut homekit: HomeKitConfig = serde_json::from_value(examples["homekit"].clone()).unwrap();
    assert!(homekit.validate().is_ok());
    homekit.port = 0;
    assert!(homekit.validate().is_err());
    homekit.port = 51827;
    homekit.name.clear();
    assert!(homekit.validate().is_err());
    let mut v = examples["homekit"].clone();
    v["backend"] = json!("native_approved");
    assert!(serde_json::from_value::<HomeKitConfig>(v).is_err());
    let mut gateway: DiscoveredGateway =
        serde_json::from_value(examples["gateway"].clone()).unwrap();
    assert!(gateway.validate().is_ok());
    gateway.expected_pin.clear();
    assert!(gateway.validate().is_err());
}

fn roundtrip<T: DeserializeOwned + Serialize>(value: &Value) {
    let decoded: T = serde_json::from_value(value.clone()).unwrap();
    assert_eq!(serde_json::to_value(decoded).unwrap(), *value);
}

#[test]
fn shared_json_contract() {
    let examples: Value =
        serde_json::from_str(include_str!("../../../fixtures/core/contracts.json")).unwrap();
    roundtrip::<Snapshot>(&examples["snapshot"]);
    roundtrip::<DiscoveredDevice>(&examples["discovered_device"]);
    roundtrip::<DeviceUpdate>(&examples["device_update"]);
    roundtrip::<CommandRecord>(&examples["command_record"]);
    roundtrip::<CommandAdmission>(&examples["command_admission"]);
    roundtrip::<HapIdentity>(&examples["hap_identity"]);
    for command in examples["commands"].as_array().unwrap() {
        roundtrip::<Command>(command);
    }
    for event in examples["events"].as_array().unwrap() {
        roundtrip::<EventEnvelope>(event);
    }
    assert!(
        serde_json::from_value::<Command>(json!({"type":"set_position", "position_pct":-1}))
            .is_err()
    );
    assert!(
        serde_json::from_value::<Command>(
            json!({"type":"set_position", "position_pct":50, "url":"https://invalid.example"})
        )
        .is_err()
    );
    assert!(serde_json::from_value::<CommandRequest>(json!({"request_id":"invalid"})).is_err());
    let record: CommandRecord = serde_json::from_value(examples["command_record"].clone()).unwrap();
    let (reply, mut receiver) = tokio::sync::oneshot::channel();
    let envelope = CommandEnvelope {
        request: record.request,
        reply,
    };
    let admission = CommandAdmission {
        receipt: record.receipt,
        is_new: false,
    };
    envelope.reply.send(Ok(admission.clone())).unwrap();
    assert_eq!(receiver.try_recv().unwrap().unwrap(), admission);
}

#[test]
fn error_codes_are_stable_and_public() {
    let errors = [
        (BridgeError::Validation, "validation_error"),
        (BridgeError::NotFound, "not_found"),
        (BridgeError::NotNative, "not_native"),
        (BridgeError::Conflict, "conflict"),
        (BridgeError::Busy, "busy"),
        (BridgeError::Unavailable, "unavailable"),
        (BridgeError::Authentication, "authentication_required"),
        (BridgeError::Forbidden, "forbidden"),
        (
            BridgeError::UpstreamAuthentication,
            "upstream_authentication_failed",
        ),
        (BridgeError::Tls, "tls_validation_failed"),
        (BridgeError::Upstream, "upstream_error"),
        (BridgeError::Storage, "storage_error"),
    ];
    for (error, code) in errors {
        assert_eq!(error.code(), code);
        assert_eq!(error.to_string(), error.message());
        assert!(!error.message().is_empty());
    }
}
