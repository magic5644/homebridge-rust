//! Minimum manufacturer DTOs; unknown fields are intentionally tolerated.
use bridge_core::model::CommandStatus;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawTahomaDevice {
    #[serde(rename = "deviceURL")]
    pub device_url: String,
    pub available: bool,
    pub enabled: bool,
    pub label: String,
    pub controllable_name: String,
    pub states: Vec<RawTahomaState>,
    pub definition: RawTahomaDefinition,
}
#[derive(Clone, Deserialize)]
pub struct RawTahomaState {
    pub name: String,
    #[serde(rename = "type")]
    pub kind: Option<u32>,
    pub value: Option<Value>,
}
#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawTahomaDefinition {
    pub commands: Vec<RawTahomaCommandDefinition>,
    #[serde(default)]
    pub states: Vec<RawTahomaStateDefinition>,
    pub ui_class: Option<String>,
    #[serde(rename = "widgetName")]
    pub widget: Option<String>,
    #[serde(rename = "type")]
    pub type_name: Option<String>,
}
#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawTahomaCommandDefinition {
    pub command_name: String,
    pub nparams: u32,
}
#[derive(Clone, Deserialize)]
pub struct RawTahomaStateDefinition {
    pub name: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TahomaEvent {
    pub name: String,
    pub timestamp: Option<u64>,
    #[serde(rename = "deviceURL")]
    pub device_url: Option<String>,
    pub device_states: Option<Vec<RawTahomaState>>,
    pub exec_id: Option<String>,
    pub new_state: Option<String>,
    pub old_state: Option<String>,
}
/// Manufacturer execution outcome; never a measurement of physical device state.
pub struct TahomaExecution {
    pub id: String,
    pub status: CommandStatus,
}

/// Raw command bodies are encoded by the profile layer, never inferred here.
#[derive(Serialize)]
pub struct TahomaCommand {
    pub name: String,
    pub parameters: Vec<Value>,
}
#[derive(Serialize)]
pub struct TahomaAction {
    #[serde(rename = "deviceURL")]
    pub device_url: String,
    pub commands: Vec<TahomaCommand>,
}
#[derive(Serialize)]
pub struct TahomaActionGroup {
    pub actions: Vec<TahomaAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}
#[derive(Deserialize)]
pub(crate) struct Setup {
    pub gateways: Vec<Gateway>,
    pub devices: Vec<RawTahomaDevice>,
}
#[derive(Deserialize)]
pub(crate) struct Gateway {
    #[serde(rename = "gatewayId")]
    pub gateway_id: String,
}
#[derive(Deserialize)]
pub(crate) struct Listener {
    pub id: String,
}
#[derive(Deserialize)]
pub(crate) struct ExecutionId {
    #[serde(rename = "execId")]
    pub exec_id: String,
}
#[derive(Deserialize)]
pub(crate) struct Execution {
    pub id: String,
    pub state: String,
}
