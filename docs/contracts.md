# Shared contracts — version 1 (P05 core types)

This document fixes shared decisions. P05 implements the data types in `bridge-core`; storage, adapters, routing and public API implementations remain with their later tickets. The public names below are normative; changes require updating the contract, fixtures, and every consumer in the same work package.

## 1. Identity and ownership

- `DeviceId`, `CommandId`, and `RequestId` alias `uuid::Uuid`, serialized as UUID strings.
- `IntegrationId` is a configured stable string, such as `tahoma-main`.
- The source key is `(integration_id, external_id)`; `external_id` is the manufacturer's opaque identifier, never the device name.
- Initial inventory allocates/stores a UUID. Renaming/reordering devices or restarting preserves it. Switching cloud/local APIs requires explicit mapping; do not merge by name.
- `Ownership`: `legacy`, `observe`, `native`. Commands/publication are prohibited outside `native`.
- A disappeared device becomes unavailable. Its row and identifiers are not implicitly deleted.

## 2. Shared model in `bridge-core::model`

Implemented in `bridge-core::model`, with configuration in `bridge-core::config`, errors in `bridge-core::error`, and events in `bridge-core::event`.

```rust
pub type DeviceId = uuid::Uuid;
pub type CommandId = uuid::Uuid;
pub type RequestId = uuid::Uuid;
pub type IntegrationId = String;

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

pub enum Capability {
    OpenCloseStop, Position, Tilt, Power, Brightness,
    Temperature, Contact, Motion, Setpoint, SecurityReadOnly,
    GarageDoor, Obstruction, MomentaryGarageAction,
}
pub enum StateKey {
    PositionPct, TiltDegrees, Power, BrightnessPct,
    TemperatureCentiCelsius, ContactOpen, MotionDetected,
    TargetTemperatureCentiCelsius, SecurityState,
    DoorState, ObstructionDetected,
}
pub struct NumericLimits { pub min: i64, pub max: i64, pub step: i64 }
pub enum Scalar { Bool(bool), Integer(i64), Text(String) }
pub enum StateQuality { Confirmed, Estimated, Stale, Unknown }
pub struct StateSample {
    pub value: Option<Scalar>,
    pub quality: StateQuality,
    pub observed_at_ms: Option<u64>,
}
pub struct Snapshot { pub revision: u64, pub devices: Vec<Device> }

pub enum Command {
    Open, Close, Stop,
    SetPosition { position_pct: u8 },
    SetTilt { degrees: i16 },
    SetPower { on: bool },
    SetBrightness { percent: u8 },
    SetSetpoint { centi_celsius: i32 },
    TriggerGarage,
}
pub enum CommandOrigin { Web, HomeKit, Test }
pub struct CommandRequest {
    pub request_id: RequestId,
    pub device_id: DeviceId,
    pub command: Command,
    pub origin: CommandOrigin,
}
pub enum CommandStatus {
    Accepted, Sent, Running, Succeeded, Failed, Unknown, Cancelled,
}
pub struct CommandReceipt {
    pub command_id: CommandId,
    pub request_id: RequestId,
    pub status: CommandStatus,
}
pub struct CommandAdmission {
    pub receipt: CommandReceipt,
    pub is_new: bool,
}
pub struct CommandEnvelope {
    pub request: CommandRequest,
    pub reply: tokio::sync::oneshot::Sender<Result<CommandAdmission, BridgeError>>,
}
```

Shared imports: `BTreeMap = std::collections::BTreeMap`, `Path/PathBuf = std::path::{Path, PathBuf}`, `Duration = std::time::Duration`, `CancellationToken = tokio_util::sync::CancellationToken`; `BridgeError` is defined in `bridge-core::error`. Serialized enums use `snake_case`. `Scalar` is tagged by `kind`/`value`, `Command` by `type`. Data-free `Command` variants become, for example, `{"type":"open"}`.

Rules: position 0 = fully closed, 100 = fully open; tilt −90° to +90° when supported; integer temperatures in hundredths of °C. `SetPosition` and `SetBrightness` accept 0..100 inclusive; manufacturer limits may be stricter. Validate ranges/steps before sending. `Unknown` requires `value = None` and no confirmed observation timestamp. Non-unknown samples require a value; confirmed samples also require an observation timestamp. An estimate never becomes `Confirmed` without physical feedback. Validation does not promote quality or update physical state; future adapters must supply actual feedback provenance.

The public fields preserve the contract shapes; deserialization checks representation, not semantic validity. Consumers must call `StateSample::validate_for(key)`, `NumericLimits::validate()` and the relevant config `validate()` before accepting input. Validate numeric state samples against declared device limits with `NumericLimits::validate_value`. `Command::validate()` checks generic bounds; `Command::validate_for(&Device)` additionally checks current ownership, availability, declared capability and applicable manufacturer limits. Recheck immediately before transmission. `Ownership::allows_commands()` and `allows_publication()` are false for both `legacy` and `observe`. These predicates do not grant HomeKit backend approval. No runtime caller exists yet; P06/P07/P11/P18/P20 must enforce these boundaries.

Unrecognized hardware receives no command capabilities. Unknown Somfy fields are tolerated and retained in anonymized fixtures; they do not automatically become generic API commands.

Garage: `DoorState` uses `Scalar::Text` with `open`, `closed`, `opening`, `closing`, or `stopped`; no measurement = `Unknown`. `ObstructionDetected` is confirmed only when actually measured. Enable `GarageDoor` for HomeKit only when mandatory characteristics can be populated honestly. `MomentaryGarageAction` represents a pulse, never door state. A cyclic command does not automatically receive `OpenCloseStop`.

`TriggerGarage` is reserved for pulse mode explicitly qualified at P33. Never use it to automatically fulfill a HomeKit open/close request. P34/P35 may add heating modes or an action-group model only after precise inventory, updating this contract and every consumer.

## 3. Errors

`BridgeError` exposes `code() -> &'static str` and a secret-free public message. Technical detail belongs in a redacted log event. Error codes are stable across API, CLI, and tests.

| Variant / code | HTTP | Meaning |
|---|---|---|
| `Validation` / `validation_error` | 422 | Invalid value or capability |
| `NotFound` / `not_found` | 404 | Missing device/command |
| `NotNative` / `not_native` | 409 | Device in observe or legacy mode |
| `Conflict` / `conflict` | 409 | Same request_id with different content or outdated revision |
| `Busy` / `busy` | 429 | Queue or per-device operation saturated |
| `Unavailable` / `unavailable` | 503 | Gateway/device unavailable |
| `Authentication` / `authentication_required` | 401 | Missing/invalid session |
| `Forbidden` / `forbidden` | 403 | Invalid CSRF/origin/permission |
| `UpstreamAuthentication` / `upstream_authentication_failed` | 502 | Manufacturer credentials/session rejected; no secret in response |
| `Tls` / `tls_validation_failed` | 502 | Invalid CA or hostname |
| `Upstream` / `upstream_error` | 502 | Unusable manufacturer response |
| `Storage` / `storage_error` | 500 | Transaction/storage failure; operation not reported successful |

Variants may carry redacted private context without changing codes. Error JSON: `{"error":{"code":"not_native","message":"This device is in observation mode.","request_id":"<UUID>"}}`.

## 4. Storage in `bridge-core::store`

```rust
impl Store {
    pub async fn open(data_dir: &Path) -> Result<Self, BridgeError>;
    pub async fn snapshot(&self) -> Result<Snapshot, BridgeError>;
    pub fn subscribe(&self) -> tokio::sync::broadcast::Receiver<EventEnvelope>;
    pub async fn upsert_discovered(&self, integration_id: &str,
        devices: Vec<DiscoveredDevice>) -> Result<Snapshot, BridgeError>;
    pub async fn set_ownership(&self, id: DeviceId, ownership: Ownership,
        expected_revision: u64) -> Result<Snapshot, BridgeError>;
    pub async fn apply_updates(&self, updates: Vec<DeviceUpdate>)
        -> Result<Snapshot, BridgeError>;
    pub async fn set_integration_availability(&self, integration_id: &str,
        available: bool) -> Result<(), BridgeError>;
    pub async fn accept_command(&self, request: &CommandRequest)
        -> Result<CommandAdmission, BridgeError>;
    pub async fn mark_sent(&self, id: CommandId, execution_id: &str)
        -> Result<(), BridgeError>;
    pub async fn set_command_status(&self, id: CommandId,
        status: CommandStatus) -> Result<(), BridgeError>;
    pub async fn command(&self, id: CommandId)
        -> Result<CommandRecord, BridgeError>;
    pub async fn allocate_hap_ids(&self, device: DeviceId,
        service_keys: Vec<String>) -> Result<HapIdentity, BridgeError>;
    pub async fn backup_to(&self, destination: &Path) -> Result<(), BridgeError>;
}
```

P05 also defines: `DiscoveredDevice` = `external_id`, `name`, `capabilities`, `limits`, `available`, `states`; `DeviceUpdate` = `device_id`, `available: Option<bool>`, `states`; `CommandRecord` = request, receipt, `created_at_ms`, `updated_at_ms`, `upstream_execution_id: Option<String>`; `HapIdentity` = `aid: u64`, `iids: BTreeMap<String,u64>`.

P06 implements `schema_migrations`, `devices`, `hap_ids`, `commands`, `sessions`, and `settings` tables. Unique constraints cover the source key and request_id; transactions cover identity/revision/command. Service→IID mappings survive capability removal. Reject a second process opening the same directory for writing through a process lock.

The revision is a persistent increasing integer for every snapshot change. Commands have their own state and do not simulate physical measurement changes. State events are emitted only after the transaction commits.

Create newly discovered devices in `observe`. A new inventory never changes previously chosen ownership. Integration availability is persisted in `settings` and emits `IntegrationChanged` only after commit.

Idempotency: same request_id and normalized command → existing receipt, no new send; same ID with different content → conflict. Retain at least 24 h, maximum 10,000 commands; when saturated within this window, reject new commands instead of deleting a still-valid key. Nonterminal commands become `Unknown` on restart, without retransmission.

Decide `CommandAdmission.is_new` transactionally. Only `true` permits router transmission; a duplicate whose receipt is still `Accepted` must not cause a second send. `is_new` remains internal: the API returns the receipt with HTTP 202 or 200 as appropriate. `mark_sent` persists the manufacturer execution ID and `Sent` status together.

## 5. Events

```rust
pub struct EventEnvelope { pub sequence: u64, pub event: BridgeEvent }
pub enum BridgeEvent {
    SnapshotChanged { revision: u64, device_ids: Vec<DeviceId> },
    CommandChanged { command_id: CommandId, status: CommandStatus },
    IntegrationChanged { integration_id: IntegrationId, available: bool },
    ResyncRequired,
}
```

`BridgeEvent` uses the internal JSON tag `type` with snake_case variants; see `fixtures/core/contracts.json`.

`sequence` belongs to the current process run, with no durable-resume guarantee. Web events contain no tokens, manufacturer URLs, or pairing keys. Clients always fetch a snapshot after reconnecting. Broadcast loss causes `ResyncRequired`, then a fresh snapshot. HomeKit consumers perform the same reconciliation rather than ignoring lag.

## 6. Adapter interfaces

```rust
// integration-tahoma
pub async fn discover(timeout: Duration)
    -> Result<Vec<DiscoveredGateway>, BridgeError>;
impl TahomaClient {
    pub async fn connect(config: TahomaConfig, credentials: TahomaCredentials)
        -> Result<Self, BridgeError>;
    pub async fn setup(&self) -> Result<Vec<DiscoveredDevice>, BridgeError>;
    pub async fn register_listener(&self) -> Result<String, BridgeError>;
    pub async fn fetch_events(&self, listener: &str)
        -> Result<Vec<TahomaEvent>, BridgeError>;
    pub async fn unregister_listener(&self, listener: &str)
        -> Result<(), BridgeError>;
    pub async fn execute(&self, external_id: &str, command: &Command)
        -> Result<String, BridgeError>; // manufacturer execution identifier
    pub async fn execution(&self, execution_id: &str)
        -> Result<TahomaExecution, BridgeError>;
}
pub fn map_device(raw: &RawTahomaDevice) -> Result<DiscoveredDevice, BridgeError>;
pub fn encode_command(raw: &RawTahomaDevice, command: &Command)
    -> Result<serde_json::Value, BridgeError>;

// transport-homekit: same signature for both backends
pub async fn run(config: HomeKitConfig, store: Store,
    commands: tokio::sync::mpsc::Sender<CommandEnvelope>,
    cancel: tokio_util::sync::CancellationToken) -> Result<(), BridgeError>;
```

`RawTahomaDevice`, `TahomaEvent`, `TahomaExecution`, and execution DTOs remain connector-private except types explicitly used by public functions. P11 defines their fields from fixtures/OpenAPI; do not generate the entire Somfy API. `TahomaExecution` exposes at least `id: String` and `status: CommandStatus`.

P05 defines `DiscoveredGateway` (`tls_dns_name: String`, `port: u16`, private `expected_pin: String`, `resolved_ip: Option<IpAddr>`, `integration_id`) and mode-specific `TahomaConfig`, internally tagged by `mode`. `LocalToken` contains `integration_id`, the same four gateway fields, `token_file: PathBuf` and `ca_file: PathBuf`. `SomfyAccount` contains `integration_id`, `provider: SomfyProvider`, private `gateway_pin: String`, and `credentials_file: PathBuf`. `SomfyProvider::SomfyEurope` serializes as `somfy_europe` and identifies the only initially declared provider/region; P11 must validate the actual authentication flow before supporting it. Reject mixed-mode fields and arbitrary account endpoints; local mode is recommended. P07 defines private secret loading. No credentials are serialized in public config. `TahomaConfig` and `DiscoveredGateway` deliberately implement neither `Debug` nor `Serialize`, since they contain private gateway selections. Future public views must use explicit redacted DTOs; future config writing must handle private fields separately. Parser diagnostics may echo input and must be mapped to secret-free `BridgeError` messages at future boundaries. DNS validation is syntactic and rejects URLs/IP literals; an optional resolved IP never replaces the validated TLS DNS identity. P07 owns file access/permissions; P11 owns CA/DNS identity and endpoint verification.

P11 defines connector-private `TahomaCredentials` variants for a loaded local token or Somfy account credentials/session; it must not derive secret-revealing Debug/serialization. `connect` authenticates the selected mode using bounded requests; a mismatch fails without trying the other mode. Account session renewal must preserve the no-replay command rule. No current runtime caller exists: P05/P07/P11/P23 are the affected future consumers. See the [access guide](integrations/tahoma-access.md) for storage and current-source limits.

`HomeKitConfig`: `name: String`, `port: u16`, `interface: Option<String>`, `state_dir: PathBuf`, `backend: HomeKitBackend`. While P04 is `stop-and-review`, the only backend value is `HomeKitBackend::StopAndReview` (`stop_and_review`). It is a blocked decision marker, never permission to run a transport. The owner exception authorizes P05 types/tests only; it approves no HomeKit backend. An approved future decision must update this enum and every consumer together. Generate/store pairing PINs in private state, never in example files.

Both HomeKit implementation paths are native Rust: a suitable existing stack (P21) or a project HAP implementation (P22), using established cryptographic libraries. No Node/JavaScript backend is allowed. Their shared `run` interface and all safety/persistence rules remain unchanged.

HomeKit transport uses Store to allocate AID/IID, subscribe to events, and reread a snapshot after lost events. It must not keep only an initial copy that cannot resynchronize.

P14 provides async `run_events(client: TahomaClient, store: Store, cancel: CancellationToken) -> Result<(), BridgeError>`; updates flow through Store, which publishes events after commit. `Store` is clonable and shares the same worker and 256-capacity broadcast. Each consumer calls `subscribe`, then reads a snapshot and reconciles on lag.

P18 provides async `CommandRouter::submit(&self, CommandRequest) -> Result<CommandAdmission, BridgeError>`, shared by Web and HomeKit. It uses Store and the concrete connector; do not invent a dynamic-plugin interface.

## 7. Web API version 1

Prefix `/api/v1`. OpenAPI produced at P08 transposes these contracts; it is not a second independent specification. Authentication is required except for health routes and first-admin setup.

| Method / path | Input / output and rule |
|---|---|
| `GET /health/live` | 200 if process responds; no private data |
| `GET /health/ready` | 200 if storage/API ready, otherwise 503; gateway failures appear as degradation in authenticated API |
| `GET /api/v1/setup/status` | Only `initialized: bool` |
| `POST /api/v1/setup/admin` | Ephemeral code + username + password; 201 once only |
| `POST /api/v1/auth/login` | Creates session; 204 + cookie; generic error |
| `POST /api/v1/auth/logout` | Destroys session; 204 |
| `GET /api/v1/auth/session` | Current user and session CSRF token |
| `GET /api/v1/system` | Version, uptime, RSS if available, integration states, HAP backend |
| `GET /api/v1/devices` | Public snapshot: revision + devices; external_id omitted |
| `POST /api/v1/devices/{id}/commands` | `{request_id, command}`; 202 for new receipt, 200 for idempotent receipt |
| `GET /api/v1/commands/{id}` | Command state, timestamps, public error |
| `PUT /api/v1/devices/{id}/ownership` | `{ownership, expected_revision}`; observe by default; no remote Homebridge disabling |
| `GET /api/v1/events` | Session SSE; 15 s heartbeat; resynchronize on every reconnect |
| `GET /api/v1/integrations/tahoma` | Secret-free mode/configuration, connection state, latest redacted error |
| `PUT /api/v1/integrations/tahoma` | Explicit mode and parameters/write-only token or account credentials; validation, no secret echo |
| `POST /api/v1/integrations/tahoma/test` | TLS/auth/read test; no commands |
| `GET /api/v1/homekit` | Backend, status, published devices; no PIN |
| `GET /api/v1/homekit/pairing` | QR/PIN for administrator session; `Cache-Control: no-store` |
| `GET /api/v1/logs?limit=...` | Maximum 500 redacted lines |
| `POST /api/v1/backups` | Downloadable redacted backup; secrets explicitly excluded |
| `POST /api/v1/restore/validate` | Validates redacted archive and returns report; applies nothing |

Restoration that applies data is a local CLI command in the initial delivery. No route restarts the NAS, changes Docker, or executes a shell. Updates use Docker/native installation with a prior backup.

Command JSON:

```json
{
  "request_id": "00000000-0000-4000-8000-000000000010",
  "command": { "type": "set_position", "position_pct": 80 }
}
```

This route never directly accepts a Somfy command name or arbitrary URL. The connector selects operations allowed by inventoried capabilities.

## 8. Initial limits and shutdown

| Resource | Proposed limit |
|---|---|
| Ordinary API JSON | 256 KiB; 413 above limit |
| TaHoma setup response | 4 MiB decompressed; explicit error above limit |
| Admitted command queue | 32 globally, one ordinary active command per device; priority stop admitted during movement |
| Wait before transmission | 5 s; expire before sending when exceeded |
| Manufacturer HTTP request | 5 s timeout; one write in flight per gateway |
| Movement tracking | 120 s timeout; `Unknown` if outcome unproven |
| Event broadcast | 256 events; resync for slow reader |
| SSE clients | 16 concurrent; 429 above limit |
| SQLite worker requests | 64 queued; bounded rejection |
| In-memory log history | 500 lines, maximum 2 KiB per line |
| Restore archive | 50 MiB compressed, 200 MiB extracted, 1,000 entries |
| SIGTERM shutdown | Stop admission, cancel streams, close storage, exit ≤ 10 s |

Change thresholds only with a measurement and corresponding test. A full queue never causes unbounded `spawn`. `Stop` may replace a not-yet-sent position; it does not repeat an already uncertain physical operation.

A sent command becomes `Cancelled` only with manufacturer evidence; otherwise it remains tracked or becomes `Unknown`. A movement's completion does not prohibit receiving its Stop command. The router reserves priority processing for Stop by replacing an unsent position if needed; it does not raise the total bound of 32.

Recheck ownership/availability immediately before transmission. Switching to `observe` cancels unsent commands and prevents later transmission; a sent command stays tracked or becomes uncertain, without a promise to stop it physically.

Open handlers and SSE must be cancellable so graceful shutdown cannot hang. Use monotonic time for deadlines and Unix time for persistence/display.

## 9. Shared fixtures

P03 creates only invented examples under `fixtures/tahoma/`: `setup_io.json`, `setup_rts.json`, `setup_switch.json`, `setup_sensor.json`, `setup_unknown.json`, `events_state.json`, `events_execution.json`, `events_listener_expired.json`, `setup_extra_fields.json`, and `setup_invalid.json`.

Examples use a fictional PIN and consistent but unreal URLs. Converting real captures must retain useful commands, types, and bounds while replacing identifiers, room names, locations, accounts, and keys. Anonymized real event shapes supplement OpenAPI if it lacks detail; do not invent a structure and present it as confirmed.

P09 adds `fixtures/api/`: snapshot, command, errors, resync. P20 adds `fixtures/homekit/`: projection and unavailable states. All three layers use the same closed=0/open=100 convention and fictional UUIDs.

P05 adds entirely fictional shared type examples under `fixtures/core/`: `contracts.json` (snapshot, discovery/update, commands, admissions/records, identities and events) and `config.json` (both mutually exclusive TaHoma modes, gateway discovery and blocked HomeKit configuration). These internal examples are not public API payloads: internal devices retain `external_id`; P08/P09 must implement the redacted API boundary. All paths/identities/PINs are invented; no file is loaded and no bridge is started by these fixtures.
