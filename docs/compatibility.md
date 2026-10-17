# P03 — Inventory and compatibility evidence

Date: October 17, 2026. Status: **blocked on installation access**; fictional fixtures are available for review. No native compatibility has been demonstrated.

## Evidence and access boundary

The [earlier read-only UI inventory](inventory-template.md) reported 23 visible elements and Homebridge 2.4.0, UI 5.29.0, Node 24.21.0, and homebridge-tahoma 2.2.64 in a child bridge. These are historical observations, not fresh authenticated setup evidence. The owner reported io and RTS coverings; the split remains unknown.

This session received no TaHoma endpoint/token file, NAS access method, private captures, or connected inventory browser session. The repository has no `private/` directory and no explicitly named TaHoma/QNAP/Homebridge access environment variables. No credential stores, private backups, or unrelated configuration were searched. No installation requests, event listeners, device commands, pairing, service changes, or plugin changes were made.

QTS/Container Station versions, image/digest, topology, ports, volumes, network/resource settings, redacted plugin options, controller versions, Apple Home automations, and actual Matter usage remain uncollected. A named read-only session or locally supplied capture/access path is needed; secrets must stay outside Git and documentation queries.

## Coverage of all visible elements

References below are placeholders for the historical counts, not household names, manufacturer IDs, or stable mappings to individual devices. Every row remains **legacy / unvalidated**. The existing Homebridge child bridge is the current fallback; its continued operation was not retested. Features shown by the UI are not physical validation.

| Reference | Observed UI family | Physical device or action | Used commands/states and dependencies | Current path | Target and qualification |
|---|---|---|---|---|---|
| covering-01 | Covering | Unqualified | Not collected | Homebridge child bridge | P12/P13: qualify model, io/RTS, position/tilt and calibration; retain legacy until validation |
| covering-02 | Covering | Unqualified | Not collected | Homebridge child bridge | P12/P13: qualify model, io/RTS, position/tilt and calibration; retain legacy until validation |
| covering-03 | Covering | Unqualified | Not collected | Homebridge child bridge | P12/P13: qualify model, io/RTS, position/tilt and calibration; retain legacy until validation |
| covering-04 | Covering | Unqualified | Not collected | Homebridge child bridge | P12/P13: qualify model, io/RTS, position/tilt and calibration; retain legacy until validation |
| covering-05 | Covering | Unqualified | Not collected | Homebridge child bridge | P12/P13: qualify model, io/RTS, position/tilt and calibration; retain legacy until validation |
| covering-06 | Covering | Unqualified | Not collected | Homebridge child bridge | P12/P13: qualify model, io/RTS, position/tilt and calibration; retain legacy until validation |
| covering-07 | Covering | Unqualified | Not collected | Homebridge child bridge | P12/P13: qualify model, io/RTS, position/tilt and calibration; retain legacy until validation |
| covering-08 | Covering | Unqualified | Not collected | Homebridge child bridge | P12/P13: qualify model, io/RTS, position/tilt and calibration; retain legacy until validation |
| covering-09 | Covering | Unqualified | Not collected | Homebridge child bridge | P12/P13: qualify model, io/RTS, position/tilt and calibration; retain legacy until validation |
| covering-10 | Covering | Unqualified | Not collected | Homebridge child bridge | P12/P13: qualify model, io/RTS, position/tilt and calibration; retain legacy until validation |
| covering-11 | Covering | Unqualified | Not collected | Homebridge child bridge | P12/P13: qualify model, io/RTS, position/tilt and calibration; retain legacy until validation |
| heating-01 | Thermostat | Unqualified | Not collected | Homebridge child bridge | P34: qualify sensor, units, setpoint, modes and pilot-wire behavior; retain legacy until validation |
| heating-02 | Thermostat | Unqualified | Not collected | Homebridge child bridge | P34: qualify sensor, units, setpoint, modes and pilot-wire behavior; retain legacy until validation |
| garage-01 | Garage Door Opener | Unqualified | Not collected | Homebridge child bridge | P33: qualify pulse/separate commands, feedback and obstruction; retain legacy until validation |
| action-01 | Switch | Unqualified | Not collected | Homebridge child bridge | P35: classify scenario/group/device, effects and Apple Home usage; retain legacy until validation |
| action-02 | Switch | Unqualified | Not collected | Homebridge child bridge | P35: classify scenario/group/device, effects and Apple Home usage; retain legacy until validation |
| action-03 | Switch | Unqualified | Not collected | Homebridge child bridge | P35: classify scenario/group/device, effects and Apple Home usage; retain legacy until validation |
| action-04 | Switch | Unqualified | Not collected | Homebridge child bridge | P35: classify scenario/group/device, effects and Apple Home usage; retain legacy until validation |
| action-05 | Switch | Unqualified | Not collected | Homebridge child bridge | P35: classify scenario/group/device, effects and Apple Home usage; retain legacy until validation |
| action-06 | Switch | Unqualified | Not collected | Homebridge child bridge | P35: classify scenario/group/device, effects and Apple Home usage; retain legacy until validation |
| action-07 | Switch | Unqualified | Not collected | Homebridge child bridge | P35: classify scenario/group/device, effects and Apple Home usage; retain legacy until validation |
| action-08 | Switch | Unqualified | Not collected | Homebridge child bridge | P35: classify scenario/group/device, effects and Apple Home usage; retain legacy until validation |
| action-09 | Switch | Unqualified | Not collected | Homebridge child bridge | P35: classify scenario/group/device, effects and Apple Home usage; retain legacy until validation |

No row authorizes Rust control or HomeKit publication. An unknown garage state stays unknown; RTS estimates are not measurements. The nine Switch rows cannot yet be classified as physical devices or virtual actions. A fictional binary-switch fixture does not resolve them.

## Fictional fixtures and provenance

The [manifest](../tests/fixtures_manifest.json) lists all ten files required by [contracts §9](contracts.md#9-shared-fixtures), their purpose, fictional origin, and reviewed SHA-256 content. All data were authored from scratch; no real capture was anonymized. `0000-0000-0000` is a fictional gateway reference and all device URLs use it. No working credentials are included.

The [official Somfy OpenAPI](https://github.com/Somfy-Developer/Somfy-TaHoma-Developer-Mode/blob/main/docs/openapi.yaml), inspected October 17, 2026, defines the setup/device/state envelopes, command names with parameter counts, event names, and HTTP 400 listener-error envelope. Context7 resolve/query selected PyOverKiz after no relevant Somfy-specific result on the first query; it supplied secondary event-model context. Detailed event payloads are **illustrative, unverified against this gateway**: OpenAPI does not specify their full fields. Command/profile combinations, sensor units, and listener error strings are invented examples, not established manufacturer compatibility or production error-matching rules.

| Fixture | Intended use and boundary |
|---|---|
| `setup_io.json` | Example open/close/stop/closure profile; raw closure is not normalized openness |
| `setup_rts.json` | Example open/close/stop with no position measurement |
| `setup_switch.json` | Binary device example; does not classify any observed Switch |
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

- P03: authorized read-only installation evidence, authenticated setup, each action's effects/usage, and the [resource baseline](performance-baseline.md).
- P11–P14: confirmed raw models, commands, states, bounds/units and detailed event/error shapes before claiming real compatibility. Current fixtures support only synthetic tests.
- P33/P34/P35: garage safety/feedback, both heating profiles, and all nine action classifications. Hardware validation remains separate and requires specific authorization.
- P05: P03 is not complete; P04 is independently still required. This partial delivery does not clear either dependency.
