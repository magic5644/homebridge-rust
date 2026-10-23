use crate::model::{CommandId, CommandStatus, DeviceId, IntegrationId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventEnvelope {
    pub sequence: u64,
    pub event: BridgeEvent,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BridgeEvent {
    SnapshotChanged {
        revision: u64,
        device_ids: Vec<DeviceId>,
    },
    CommandChanged {
        command_id: CommandId,
        status: CommandStatus,
    },
    IntegrationChanged {
        integration_id: IntegrationId,
        available: bool,
    },
    ResyncRequired,
}
