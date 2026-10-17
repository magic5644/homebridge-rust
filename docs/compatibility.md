# P03 — Inventory and compatibility evidence

Date: October 17, 2026. Status: **inventory and usage observed; blocked on raw setup declarations**; fictional fixtures are available for review. No native compatibility has been demonstrated.

## Evidence and access boundary

The owner authorized the existing Homebridge, QNAP and Apple Home sessions as read-only inventory sources, without transferring a TaHoma token. All 23 Homebridge information dialogs, system/plugin pages, relevant plugin form fields, existing plugin logs, five Apple scene editors and the automation list were inspected. Editors were canceled; no accessory, scene, configuration, pairing, service or plugin was changed. No physical command was sent.

Homebridge reports 2.4.0, UI 5.29.0, Node 24.21.0, homebridge-tahoma 2.2.64 in a child bridge, Ubuntu 22.04.5 x64, Docker and container user `root`. Plugin fields show Somfy Europe/cloud service, scenario exposure enabled, refresh period 600 minutes; the polling field is blank with a 60-second placeholder, not evidence of an explicitly configured value. No password/token field or raw configuration was copied. Existing logs were downloaded to ignored `private/` (directory 0700, file 0600); only reviewed technical summaries appear here.

The logs identify 14 physical-device projections by `uiClass`/`widgetName` and all nine switches as mapped manufacturer scenarios. These are stronger evidence than generic HomeKit model labels, but contain no raw `controllableName`, command/state declarations or manufacturer model definitions. No authenticated raw setup was obtained. All eleven covering serial projections carry an `io` prefix; the owner reported both io and RTS. The actual radio split remains unresolved. Names, serials, addresses and gateway/controller identities are excluded.

The QNAP session confirms TS-251+, Celeron J1900, 2 GB RAM, QTS 5.2.10.3577 Build 20260731 and Container Station 3.1.2.1742. Reviewed container topology, image/digest, storage and limits are in the [baseline](performance-baseline.md). Kernel and Apple controller versions were not established. Actual Matter use was not established. Resource comparison is deferred to P31/P38 by owner decision and does not block P03.

## Coverage of all visible elements

References below follow the order within each family in the observed UI. They are anonymous observation references, not manufacturer IDs or durable device mappings. Every row remains **legacy / physically unvalidated**. The existing Homebridge child bridge is the current fallback; actual operation was not tested. The displayed models and characteristics describe the plugin's HAP projection, not verified physical hardware or commands used by the household.

| Reference | Observed UI family | Plugin-reported model | Displayed characteristics; use/dependencies | Current path | Target and qualification |
|---|---|---|---|---|---|
| covering-01 | Window Covering | `RollerShutter` / `PositionableRollerShutter` | Current/target position, position state, obstruction; Apple Home covering scenes; actual feedback unqualified | Homebridge child bridge | P12/P13: qualify physical model, io/RTS, tilt and calibration; retain legacy until validation |
| covering-02 | Window Covering | `RollerShutter` / `PositionableRollerShutter` | Current/target position, position state, obstruction; Apple Home covering scenes; actual feedback unqualified | Homebridge child bridge | P12/P13: qualify physical model, io/RTS, tilt and calibration; retain legacy until validation |
| covering-03 | Window Covering | `RollerShutter` / `PositionableRollerShutter` | Current/target position, position state, obstruction; Apple Home covering scenes; actual feedback unqualified | Homebridge child bridge | P12/P13: qualify physical model, io/RTS, tilt and calibration; retain legacy until validation |
| covering-04 | Window Covering | `RollerShutter` / `PositionableRollerShutter` | Current/target position, position state, obstruction; Apple Home covering scenes; actual feedback unqualified | Homebridge child bridge | P12/P13: qualify physical model, io/RTS, tilt and calibration; retain legacy until validation |
| covering-05 | Window Covering | `RollerShutter` / `PositionableTiltedRollerShutter` | Current/target position, position state, obstruction; Apple Home covering scenes; actual feedback unqualified | Homebridge child bridge | P12/P13: qualify physical model, io/RTS, tilt and calibration; retain legacy until validation |
| covering-06 | Window Covering | `RollerShutter` / `PositionableRollerShutter` | Current/target position, position state, obstruction; Apple Home covering scenes; actual feedback unqualified | Homebridge child bridge | P12/P13: qualify physical model, io/RTS, tilt and calibration; retain legacy until validation |
| covering-07 | Window Covering | `RollerShutter` / `PositionableRollerShutter` | Current/target position, position state, obstruction; Apple Home covering scenes; actual feedback unqualified | Homebridge child bridge | P12/P13: qualify physical model, io/RTS, tilt and calibration; retain legacy until validation |
| covering-08 | Window Covering | `RollerShutter` / `PositionableTiltedRollerShutter` | Current/target position, position state, obstruction; Apple Home covering scenes; actual feedback unqualified | Homebridge child bridge | P12/P13: qualify physical model, io/RTS, tilt and calibration; retain legacy until validation |
| covering-09 | Window Covering | `RollerShutter` / `PositionableTiltedRollerShutter` | Current/target position, position state, obstruction; Apple Home covering scenes; actual feedback unqualified | Homebridge child bridge | P12/P13: qualify physical model, io/RTS, tilt and calibration; retain legacy until validation |
| covering-10 | Window Covering | `RollerShutter` / `PositionableTiltedRollerShutter` | Current/target position, position state, obstruction; Apple Home covering scenes; actual feedback unqualified | Homebridge child bridge | P12/P13: qualify physical model, io/RTS, tilt and calibration; retain legacy until validation |
| covering-11 | Window Covering | `RollerShutter` / `PositionableRollerShutter` | Current/target position, position state, obstruction; Apple Home covering scenes; actual feedback unqualified | Homebridge child bridge | P12/P13: qualify physical model, io/RTS, tilt and calibration; retain legacy until validation |
| heating-01 | Thermostat | `HeatingSystem` / `AtlanticElectricalHeater` | Current/target heating-cooling state, current/target temperature, Celsius; owner reports radiator scenarios; sensor/modes need qualification | Homebridge child bridge | P34: qualify physical model, sensor, setpoint and pilot-wire behavior; retain legacy until validation |
| heating-02 | Thermostat | `HeatingSystem` / `AtlanticElectricalHeater` | Current/target heating-cooling state, current/target temperature, Celsius; owner reports radiator scenarios; sensor/modes need qualification | Homebridge child bridge | P34: qualify physical model, sensor, setpoint and pilot-wire behavior; retain legacy until validation |
| garage-01 | Garage Door Opener | `GarageDoor` / `PositionableGarageDoor` | Current/target door state and obstruction; included in an Apple closure scene; feedback/command type unqualified | Homebridge child bridge | P33: qualify pulse/separate commands, feedback and obstruction; retain legacy until validation |
| action-01 | Switch | TaHoma action group / plugin `SceneMapper` | Stored scenario; owner reports covering/radiator uses via Maison/Siri; exact membership uncollected | Homebridge child bridge | P35: qualify individual scenario membership, effects and cancellation; retain legacy until validation |
| action-02 | Switch | TaHoma action group / plugin `SceneMapper` | Stored scenario; owner reports covering/radiator uses via Maison/Siri; exact membership uncollected | Homebridge child bridge | P35: qualify individual scenario membership, effects and cancellation; retain legacy until validation |
| action-03 | Switch | TaHoma action group / plugin `SceneMapper` | Stored scenario; owner reports covering/radiator uses via Maison/Siri; exact membership uncollected | Homebridge child bridge | P35: qualify individual scenario membership, effects and cancellation; retain legacy until validation |
| action-04 | Switch | TaHoma action group / plugin `SceneMapper` | Stored scenario; owner reports covering/radiator uses via Maison/Siri; exact membership uncollected | Homebridge child bridge | P35: qualify individual scenario membership, effects and cancellation; retain legacy until validation |
| action-05 | Switch | TaHoma action group / plugin `SceneMapper` | Stored scenario; owner reports covering/radiator uses via Maison/Siri; exact membership uncollected | Homebridge child bridge | P35: qualify individual scenario membership, effects and cancellation; retain legacy until validation |
| action-06 | Switch | TaHoma action group / plugin `SceneMapper` | Stored scenario; owner reports covering/radiator uses via Maison/Siri; exact membership uncollected | Homebridge child bridge | P35: qualify individual scenario membership, effects and cancellation; retain legacy until validation |
| action-07 | Switch | TaHoma action group / plugin `SceneMapper` | Stored scenario; owner reports covering/radiator uses via Maison/Siri; exact membership uncollected | Homebridge child bridge | P35: qualify individual scenario membership, effects and cancellation; retain legacy until validation |
| action-08 | Switch | TaHoma action group / plugin `SceneMapper` | Stored scenario; owner reports covering/radiator uses via Maison/Siri; exact membership uncollected | Homebridge child bridge | P35: qualify individual scenario membership, effects and cancellation; retain legacy until validation |
| action-09 | Switch | TaHoma action group / plugin `SceneMapper` | Stored scenario; owner reports covering/radiator uses via Maison/Siri; exact membership uncollected | Homebridge child bridge | P35: qualify individual scenario membership, effects and cancellation; retain legacy until validation |

No row authorizes Rust control or HomeKit publication. The nine switches are manufacturer scenarios, distinct from the five Apple scenes below. The owner confirms that most scenarios operate coverings and others radiators, used through Apple Home and Siri. This is family-level use evidence; their exact per-scenario targets/commands were not exposed by Homebridge and must not be inferred from names. No garage involvement in those nine manufacturer scenarios was established.

## Existing behavior and migration dependencies

Source review uses the installed plugin's npm `gitHead`, [23ec9078f1111c0600933de1a775565cd7362e1e](https://github.com/dubocr/homebridge-tahoma/tree/23ec9078f1111c0600933de1a775565cd7362e1e). Source behavior describes possible projection logic, not this gateway's raw declarations:

- [Scenario mapper](https://github.com/dubocr/homebridge-tahoma/blob/23ec9078f1111c0600933de1a775565cd7362e1e/src/SceneMapper.ts): On executes a stored action group; completed/failed execution resets the switch. Off during an active execution cancels it. P35 must qualify trigger/reset/cancellation rather than model a maintained power state. The official local OpenAPI does not establish stored-scenario discovery/execution parity; retain the legacy scenario path until proven.
- [Atlantic heating mapper](https://github.com/dubocr/homebridge-tahoma/blob/23ec9078f1111c0600933de1a775565cd7362e1e/src/mappers/HeatingSystem/AtlanticElectricalHeater.ts): uses `setHeatingLevel` with comfort/eco/off/frost-protection modes and `io:TargetHeatingLevelState`. HomeKit COOL represents eco, not physical cooling. Reported current temperature may be synthesized from mode thresholds; 0 °C in the observed dialogs proves no sensor. P34 must verify actual states, mode support and bounds before exposing measured temperature or numeric setpoints.
- [Garage mapper](https://github.com/dubocr/homebridge-tahoma/blob/23ec9078f1111c0600933de1a775565cd7362e1e/src/mappers/GarageDoor.ts): may select `cycle` instead of separate open/close and synthesize state for stateless devices. P33 must establish the actual variant, physical feedback and obstruction. A displayed closed state is not safety evidence; an unknown state stays unknown and an uncertain command is never replayed.
- Covering logs distinguish seven positionable and four positionable/tilted profiles. P12/P13 must establish commands, units, radio and feedback from actual declarations before normalization/calibration; profile names alone do not prove tilt or measured position.

Apple Home scene editors were read without activating their tiles or using the test button. Anonymous references are observation references only:

| Apple scene | Configured targets | Migration dependency |
|---|---|---|
| apple-scene-01 | Four coverings, closure | Recreate/test targets only after native covering validation |
| apple-scene-02 | Same four coverings, opening | Same dependency |
| apple-scene-03 | Eleven coverings, opening | All eleven mappings required |
| apple-scene-04 | Four media targets, Pause | Preserve existing Apple media path; outside the 23 Somfy projections |
| apple-scene-05 | Eight coverings and garage, closure; four media targets, Pause | P33 safety qualification plus covering and media preservation; configured intent is not physical feedback |

The automation list contains seven entries: five explicitly disabled and two displaying no scene/accessory target. One disabled editor/target picker was additionally inspected and canceled; not every automation was audited internally. Do not reenable, delete or rewrite them automatically. Private schedules, presence conditions, labels and room names are omitted. Pairing the Rust bridge will not automatically preserve these references. No direct dependency of these five Apple scenes on the nine scenario switches was established; owner-confirmed Maison/Siri use remains separate evidence.

## Fictional fixtures and provenance

The [manifest](../tests/fixtures_manifest.json) lists all ten files required by [contracts §9](contracts.md#9-shared-fixtures), their purpose, fictional origin, and reviewed SHA-256 content. All data were authored from scratch; no real capture was anonymized. `0000-0000-0000` is a fictional gateway reference and all device URLs use it. No working credentials are included.

The [official Somfy OpenAPI](https://github.com/Somfy-Developer/Somfy-TaHoma-Developer-Mode/blob/main/docs/openapi.yaml), inspected October 17, 2026, defines the setup/device/state envelopes, command names with parameter counts, event names, and HTTP 400 listener-error envelope. Context7 resolve/query selected PyOverKiz after no relevant Somfy-specific result on the first query; it supplied secondary event-model context. Detailed event payloads are **illustrative, unverified against this gateway**: OpenAPI does not specify their full fields. Command/profile combinations, sensor units, and listener error strings are invented examples, not established manufacturer compatibility or production error-matching rules.

| Fixture | Intended use and boundary |
|---|---|
| `setup_io.json` | Example open/close/stop/closure profile; raw closure is not normalized openness |
| `setup_rts.json` | Example open/close/stop with no position measurement |
| `setup_switch.json` | Binary device example; not a fixture for the nine stored scenarios |
| `setup_sensor.json` | Temperature example; does not identify either thermostat |
| `setup_unknown.json` | Unrecognized profile with no command capabilities |
| `events_state.json` | Illustrative state change; detailed wire shape still needs evidence |
| `events_execution.json` | Illustrative execution change; completion alone is not physical feedback |
| `events_listener_expired.json` | Fictional HTTP 400 response; error code is explicitly invented |
| `setup_extra_fields.json` | Extra properties for future tolerant parsing |
| `setup_invalid.json` | Syntactically valid JSON with an invalid devices type |

Run `python3 -B -m unittest discover -s tests -v` from the repository root. Checks validate JSON, the exact ten-file set, content hashes, fictional gateway/device identifiers and labels, and prohibited private field names. Negative cases exercise missing, extra, malformed, changed, and private-identifier content. The hash gate requires review when content changes; it is not a general anonymizer or proof that an edited manifest is safe. Review new content manually before updating hashes. No product parser or mapper is implemented here.

Normalized shared position remains closed=0/open=100. The io fixture's raw `core:ClosureState=25` illustrates a different upstream convention; confirm mapping from real declarations before implementing P12. Missing RTS position must stay unknown unless explicitly estimated and labeled later.

## Prerequisites still missing

- P03 raw setup criterion remains unfulfilled: Homebridge logs do not expose `controllableName`, declared commands/states, physical model or bounds. Existing plugin options and action classification/use are now recorded; exact scenario membership is still missing. The owner retains credentials. A private owner-run read-only setup/action-group export or a suitable authenticated inventory view is required; no token transfer or physical test is needed.
- P11–P14: confirm raw models, commands, states, units and event/error shapes before claiming real compatibility. Fictional fixtures support synthetic tests only.
- P33/P34/P35: actual garage safety/feedback, heating modes/sensors and individual stored scenario effects/cancellation. Hardware validation requires specific authorization and remains outside P03.
- P05: P03 is still incomplete; P04 is independently required. Resource comparison is deferred, not a missing P03 gate. No merge is justified solely by green fixture CI.
