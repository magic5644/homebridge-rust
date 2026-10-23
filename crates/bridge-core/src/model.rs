use crate::error::BridgeError;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub type DeviceId = uuid::Uuid;
pub type CommandId = uuid::Uuid;
pub type RequestId = uuid::Uuid;
pub type IntegrationId = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Ownership {
    Legacy,
    Observe,
    Native,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Capability {
    OpenCloseStop,
    Position,
    Tilt,
    Power,
    Brightness,
    Temperature,
    Contact,
    Motion,
    Setpoint,
    SecurityReadOnly,
    GarageDoor,
    Obstruction,
    MomentaryGarageAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateKey {
    PositionPct,
    TiltDegrees,
    Power,
    BrightnessPct,
    TemperatureCentiCelsius,
    ContactOpen,
    MotionDetected,
    TargetTemperatureCentiCelsius,
    SecurityState,
    DoorState,
    ObstructionDetected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind", content = "value")]
pub enum Scalar {
    Bool(bool),
    Integer(i64),
    Text(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateQuality {
    Confirmed,
    Estimated,
    Stale,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type", deny_unknown_fields)]
pub enum Command {
    Open,
    Close,
    Stop,
    SetPosition { position_pct: u8 },
    SetTilt { degrees: i16 },
    SetPower { on: bool },
    SetBrightness { percent: u8 },
    SetSetpoint { centi_celsius: i32 },
    TriggerGarage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandOrigin {
    Web,
    HomeKit,
    Test,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandStatus {
    Accepted,
    Sent,
    Running,
    Succeeded,
    Failed,
    Unknown,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Device {
    pub id: DeviceId,
    pub integration_id: IntegrationId,
    pub external_id: String,
    pub name: String,
    pub capabilities: Vec<Capability>,
    pub limits: BTreeMap<StateKey, NumericLimits>,
    pub ownership: Ownership,
    pub available: bool,
    pub states: BTreeMap<StateKey, StateSample>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NumericLimits {
    pub min: i64,
    pub max: i64,
    pub step: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateSample {
    pub value: Option<Scalar>,
    pub quality: StateQuality,
    pub observed_at_ms: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Snapshot {
    pub revision: u64,
    pub devices: Vec<Device>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandRequest {
    pub request_id: RequestId,
    pub device_id: DeviceId,
    pub command: Command,
    pub origin: CommandOrigin,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandReceipt {
    pub command_id: CommandId,
    pub request_id: RequestId,
    pub status: CommandStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandAdmission {
    pub receipt: CommandReceipt,
    pub is_new: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiscoveredDevice {
    pub external_id: String,
    pub name: String,
    pub capabilities: Vec<Capability>,
    pub limits: BTreeMap<StateKey, NumericLimits>,
    pub available: bool,
    pub states: BTreeMap<StateKey, StateSample>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceUpdate {
    pub device_id: DeviceId,
    pub available: Option<bool>,
    pub states: BTreeMap<StateKey, StateSample>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandRecord {
    pub request: CommandRequest,
    pub receipt: CommandReceipt,
    pub created_at_ms: u64,
    pub updated_at_ms: u64,
    pub upstream_execution_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HapIdentity {
    pub aid: u64,
    pub iids: BTreeMap<String, u64>,
}

#[derive(Debug)]
pub struct CommandEnvelope {
    pub request: CommandRequest,
    pub reply: tokio::sync::oneshot::Sender<Result<CommandAdmission, BridgeError>>,
}

impl Ownership {
    pub fn allows_commands(self) -> bool {
        self == Self::Native
    }
    pub fn allows_publication(self) -> bool {
        self == Self::Native
    }
}

impl NumericLimits {
    pub fn validate(&self) -> Result<(), BridgeError> {
        if self.min > self.max || self.step <= 0 {
            return Err(BridgeError::Validation);
        }
        Ok(())
    }

    pub fn validate_value(&self, value: i64) -> Result<(), BridgeError> {
        self.validate()?;
        // Widen before subtracting so even the full i64 range is safe.
        if value < self.min
            || value > self.max
            || (i128::from(value) - i128::from(self.min)) % i128::from(self.step) != 0
        {
            return Err(BridgeError::Validation);
        }
        Ok(())
    }
}

impl StateSample {
    pub fn validate(&self) -> Result<(), BridgeError> {
        match self.quality {
            StateQuality::Unknown if self.value.is_some() || self.observed_at_ms.is_some() => {
                Err(BridgeError::Validation)
            }
            StateQuality::Unknown => Ok(()),
            _ if self.value.is_none() => Err(BridgeError::Validation),
            StateQuality::Confirmed if self.observed_at_ms.is_none() => {
                Err(BridgeError::Validation)
            }
            _ => Ok(()),
        }
    }

    pub fn validate_for(&self, key: StateKey) -> Result<(), BridgeError> {
        self.validate()?;
        let Some(value) = &self.value else {
            return Ok(());
        };
        let valid = match (key, value) {
            (StateKey::PositionPct | StateKey::BrightnessPct, Scalar::Integer(v)) => {
                (0..=100).contains(v)
            }
            (StateKey::TiltDegrees, Scalar::Integer(v)) => (-90..=90).contains(v),
            (
                StateKey::TemperatureCentiCelsius | StateKey::TargetTemperatureCentiCelsius,
                Scalar::Integer(_),
            ) => true,
            (
                StateKey::Power
                | StateKey::ContactOpen
                | StateKey::MotionDetected
                | StateKey::ObstructionDetected,
                Scalar::Bool(_),
            ) => true,
            (StateKey::SecurityState, Scalar::Text(_)) => true,
            (StateKey::DoorState, Scalar::Text(v)) => matches!(
                v.as_str(),
                "open" | "closed" | "opening" | "closing" | "stopped"
            ),
            _ => false,
        };
        if !valid {
            return Err(BridgeError::Validation);
        }
        Ok(())
    }
}

impl Command {
    pub fn validate(&self) -> Result<(), BridgeError> {
        match self {
            Self::SetPosition { position_pct } if *position_pct > 100 => {
                Err(BridgeError::Validation)
            }
            Self::SetBrightness { percent } if *percent > 100 => Err(BridgeError::Validation),
            Self::SetTilt { degrees } if !(-90..=90).contains(degrees) => {
                Err(BridgeError::Validation)
            }
            _ => Ok(()),
        }
    }

    pub fn validate_for(&self, device: &Device) -> Result<(), BridgeError> {
        if !device.ownership.allows_commands() {
            return Err(BridgeError::NotNative);
        }
        if !device.available {
            return Err(BridgeError::Unavailable);
        }
        self.validate()?;
        let (capability, numeric) = match *self {
            Self::Open | Self::Close | Self::Stop => (Capability::OpenCloseStop, None),
            Self::SetPosition { position_pct } => (
                Capability::Position,
                Some((StateKey::PositionPct, i64::from(position_pct))),
            ),
            Self::SetTilt { degrees } => (
                Capability::Tilt,
                Some((StateKey::TiltDegrees, i64::from(degrees))),
            ),
            Self::SetPower { .. } => (Capability::Power, None),
            Self::SetBrightness { percent } => (
                Capability::Brightness,
                Some((StateKey::BrightnessPct, i64::from(percent))),
            ),
            Self::SetSetpoint { centi_celsius } => (
                Capability::Setpoint,
                Some((
                    StateKey::TargetTemperatureCentiCelsius,
                    i64::from(centi_celsius),
                )),
            ),
            Self::TriggerGarage => (Capability::MomentaryGarageAction, None),
        };
        if !device.capabilities.contains(&capability) {
            return Err(BridgeError::Validation);
        }
        if let Some((key, value)) = numeric
            && let Some(limits) = device.limits.get(&key)
        {
            limits.validate_value(value)?;
        }
        Ok(())
    }
}
