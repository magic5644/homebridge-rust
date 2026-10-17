# P03 — Inventory and compatibility evidence

Date: October 17, 2026. Status: **partial inventory observed; blocked on remaining evidence**; fictional fixtures are available for review. No native compatibility has been demonstrated.

## Evidence and access boundary

The owner explicitly authorized using the existing Homebridge application instead of supplying a TaHoma token. On October 17, 2026, the authenticated UI, all 23 accessory information dialogs, status page, and plugin page were inspected read-only. No token, raw JSON configuration, logs, credential store, private backup, or pairing screen was opened. Only information dialogs were opened/closed; no accessory value, UI setting, cache, service or plugin was changed.

Current UI observations confirm Homebridge 2.4.0, UI 5.29.0, Node 24.21.0, homebridge-tahoma 2.2.64 in a child bridge, Ubuntu Jammy 22.04.5, x64, Docker enabled, and container user `root`. These are values reported by Homebridge, not a host/QTS audit. All 23 accessory dialogs identify `homebridge-tahoma` as their plugin.

The owner reported both io and RTS coverings. All eleven covering dialogs display an `io` prefix in the plugin-provided serial field, as do the heating and garage dialogs. This is evidence about the Homebridge projection only; it does not resolve the actual radio split or manufacturer model. Full serials, names, addresses and bridge identities are omitted. No raw TaHoma setup/event evidence was collected.

The owner subsequently supplied the authenticated QNAP session in Safari. System status confirms TS-251+, Celeron J1900 and 2 GB RAM, QTS 5.2.10.3577 Build 20260731. Container Station reports 3.1.2.1742 and one running Homebridge container. Its image/digest, Bridge/Qnet topology, writable persistent bind mount, resource limits and restart policy are recorded in the [baseline evidence](performance-baseline.md). Only reviewed technical facts are public; host/container IDs, addresses, hardware serials and host mount path are omitted.

Kernel, redacted plugin options, controller versions, Apple Home automations, actual Matter usage and the full process/cgroup baseline remain uncollected. Homebridge/QNAP UI access is now available; missing raw capabilities and process metrics may be supplied as privately collected evidence without handing the agent a token.

## Coverage of all visible elements

References below follow the order within each family in the observed UI. They are anonymous observation references, not manufacturer IDs or durable device mappings. Every row remains **legacy / physically unvalidated**. The existing Homebridge child bridge is the current fallback; actual operation was not tested. The displayed models and characteristics describe the plugin's HAP projection, not verified physical hardware or commands used by the household.

| Reference | Observed UI family | Plugin-reported model | Displayed characteristics; use/dependencies | Current path | Target and qualification |
|---|---|---|---|---|---|
| covering-01 | Window Covering | Somfy `RollerShutter` | Current/target position, position state, obstruction; actual feedback and use unqualified | Homebridge child bridge | P12/P13: qualify physical model, io/RTS, tilt and calibration; retain legacy until validation |
| covering-02 | Window Covering | Somfy `RollerShutter` | Current/target position, position state, obstruction; actual feedback and use unqualified | Homebridge child bridge | P12/P13: qualify physical model, io/RTS, tilt and calibration; retain legacy until validation |
| covering-03 | Window Covering | Somfy `RollerShutter` | Current/target position, position state, obstruction; actual feedback and use unqualified | Homebridge child bridge | P12/P13: qualify physical model, io/RTS, tilt and calibration; retain legacy until validation |
| covering-04 | Window Covering | Somfy `RollerShutter` | Current/target position, position state, obstruction; actual feedback and use unqualified | Homebridge child bridge | P12/P13: qualify physical model, io/RTS, tilt and calibration; retain legacy until validation |
| covering-05 | Window Covering | Somfy `RollerShutter` | Current/target position, position state, obstruction; actual feedback and use unqualified | Homebridge child bridge | P12/P13: qualify physical model, io/RTS, tilt and calibration; retain legacy until validation |
| covering-06 | Window Covering | Somfy `RollerShutter` | Current/target position, position state, obstruction; actual feedback and use unqualified | Homebridge child bridge | P12/P13: qualify physical model, io/RTS, tilt and calibration; retain legacy until validation |
| covering-07 | Window Covering | Somfy `RollerShutter` | Current/target position, position state, obstruction; actual feedback and use unqualified | Homebridge child bridge | P12/P13: qualify physical model, io/RTS, tilt and calibration; retain legacy until validation |
| covering-08 | Window Covering | Somfy `RollerShutter` | Current/target position, position state, obstruction; actual feedback and use unqualified | Homebridge child bridge | P12/P13: qualify physical model, io/RTS, tilt and calibration; retain legacy until validation |
| covering-09 | Window Covering | Somfy `RollerShutter` | Current/target position, position state, obstruction; actual feedback and use unqualified | Homebridge child bridge | P12/P13: qualify physical model, io/RTS, tilt and calibration; retain legacy until validation |
| covering-10 | Window Covering | Somfy `RollerShutter` | Current/target position, position state, obstruction; actual feedback and use unqualified | Homebridge child bridge | P12/P13: qualify physical model, io/RTS, tilt and calibration; retain legacy until validation |
| covering-11 | Window Covering | Somfy `RollerShutter` | Current/target position, position state, obstruction; actual feedback and use unqualified | Homebridge child bridge | P12/P13: qualify physical model, io/RTS, tilt and calibration; retain legacy until validation |
| heating-01 | Thermostat | Somfy `HeatingSystem` | Current/target heating-cooling state, current/target temperature, Celsius; actual sensor/modes unqualified | Homebridge child bridge | P34: qualify physical model, sensor, setpoint and pilot-wire behavior; retain legacy until validation |
| heating-02 | Thermostat | Somfy `HeatingSystem` | Current/target heating-cooling state, current/target temperature, Celsius; actual sensor/modes unqualified | Homebridge child bridge | P34: qualify physical model, sensor, setpoint and pilot-wire behavior; retain legacy until validation |
| garage-01 | Garage Door Opener | Somfy `GarageDoor` | Current/target door state and obstruction; physical feedback and command type unqualified | Homebridge child bridge | P33: qualify pulse/separate commands, feedback and obstruction; retain legacy until validation |
| action-01 | Switch | `Default-Manufacturer` / `Default-Model` | Switch projection only; effects, source type and Apple Home use unqualified | Homebridge child bridge | P35: classify scenario/group/device and dependencies; retain legacy until validation |
| action-02 | Switch | `Default-Manufacturer` / `Default-Model` | Switch projection only; effects, source type and Apple Home use unqualified | Homebridge child bridge | P35: classify scenario/group/device and dependencies; retain legacy until validation |
| action-03 | Switch | `Default-Manufacturer` / `Default-Model` | Switch projection only; effects, source type and Apple Home use unqualified | Homebridge child bridge | P35: classify scenario/group/device and dependencies; retain legacy until validation |
| action-04 | Switch | `Default-Manufacturer` / `Default-Model` | Switch projection only; effects, source type and Apple Home use unqualified | Homebridge child bridge | P35: classify scenario/group/device and dependencies; retain legacy until validation |
| action-05 | Switch | `Default-Manufacturer` / `Default-Model` | Switch projection only; effects, source type and Apple Home use unqualified | Homebridge child bridge | P35: classify scenario/group/device and dependencies; retain legacy until validation |
| action-06 | Switch | `Default-Manufacturer` / `Default-Model` | Switch projection only; effects, source type and Apple Home use unqualified | Homebridge child bridge | P35: classify scenario/group/device and dependencies; retain legacy until validation |
| action-07 | Switch | `Default-Manufacturer` / `Default-Model` | Switch projection only; effects, source type and Apple Home use unqualified | Homebridge child bridge | P35: classify scenario/group/device and dependencies; retain legacy until validation |
| action-08 | Switch | `Default-Manufacturer` / `Default-Model` | Switch projection only; effects, source type and Apple Home use unqualified | Homebridge child bridge | P35: classify scenario/group/device and dependencies; retain legacy until validation |
| action-09 | Switch | `Default-Manufacturer` / `Default-Model` | Switch projection only; effects, source type and Apple Home use unqualified | Homebridge child bridge | P35: classify scenario/group/device and dependencies; retain legacy until validation |

No row authorizes Rust control or HomeKit publication. An unknown garage state stays unknown; RTS estimates are not measurements. The nine Switch rows cannot yet be classified as physical devices or virtual actions. A fictional binary-switch fixture does not resolve them. Both thermostat projections display 0 °C, which does not establish a real temperature sensor. Door/obstruction and position characteristics do not establish physical feedback merely because the plugin populates them.

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

- P03: remaining kernel/plugin facts, raw capabilities/setup evidence, each action's effects/usage, and the [resource baseline](performance-baseline.md). Homebridge UI inventory is now observed; no token transfer is required to review privately collected evidence.
- P11–P14: confirmed raw models, commands, states, bounds/units and detailed event/error shapes before claiming real compatibility. Current fixtures support only synthetic tests.
- P33/P34/P35: garage safety/feedback, both heating profiles, and all nine action classifications. Hardware validation remains separate and requires specific authorization.
- P05: P03 is not complete; P04 is independently still required. This partial delivery does not clear either dependency.
