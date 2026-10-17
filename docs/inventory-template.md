# Inventory and coverage matrix

Fill in a copy in `private/inventory.md` or another private location. The public repository contains only anonymized technical profiles, element counts, and coverage levels. This document requests no passwords, tokens, gateway PINs, or pairing PINs.

## 1. Confirmed hardware

| Item | Known value | To collect |
|---|---|---|
| NAS | QNAP TS-251+; QTS 5.2.10.3577; Container Station 3.1.2.1742 | Kernel and binary compatibility |
| CPU/RAM | Intel Celeron J1900, 2 GB reported by the owner | Kernel, instruction set, and host binary compatibility |
| Current installation | Docker; UI reports Ubuntu 22.04.5, x64 | Reviewed image/topology/storage/limits in performance-baseline.md |
| Homebridge | 2.4.0 | Backup and relevant redacted configuration |
| UI | 5.29.0 | Access/proxy settings without secrets |
| Node | 24.21.0 | Required processes and each process's PSS/RSS |
| Plugin | homebridge-tahoma 2.2.64 child bridge; Somfy Europe/cloud, scenarios exposed | Raw profile declarations and individual overrides |
| Gateway | TaHoma V2 | Motor models and exact setup capabilities |
| Local API | Developer API announced; protocol 2026.4.5-7; TLS and GET apiVersion validated | Owner-run authenticated setup captured; declarations in tahoma-profiles.md |
| Controllers | Owner confirms Apple Home/Siri; five scenes/seven automation entries inspected | Versions, hub and actual Matter usage |

## 2. Visible elements — read-only inspection on October 17, 2026

Household names are omitted. The following numbers are inventory references, not real system UUIDs.

| Group | Count | Confirmed | Still to qualify | Ticket |
|---|---:|---|---|---|
| Coverings | 11 | Seven generic/four Velux io profiles captured; no RTS entry | Physical part numbers, bounds, position feedback/calibration; no tilt command/state declared | P12/P13 |
| Thermostats | 2 | Two io:AtlanticElectricalHeaterIOComponent declarations | Actual measurement, possible pilot-wire heating, setpoints/modes | P34 |
| Garage | 1 | io:GarageOpenerIOComponent; separate open/close/stop, no cycle | Physical model, feedback reliability and obstruction (no explicit obstruction state declared) | P33 |
| Switch | 9 | Logs confirm stored scenarios; owner reports covering/radiator uses through Maison/Siri | Exact per-scenario targets, effects/cancellation | P35 |
| Total | 23 | UI elements, not necessarily 23 physical devices | Other invisible uses and scene dependencies | P36 |

Do not infer a verified physical measurement from a displayed value. No controls were activated during this inventory.

## 3. One row per device or action, in the private copy

| Anonymous reference | Family/model | io/RTS/other | Commands used | Actually available states | Limits/calibration | Dependent automations | Current owner | Target | Validation |
|---|---|---|---|---|---|---|---|---|---|
| covering-01 | To collect | To collect | Open/close/stop/position as used | Collect from setup, then compare physically | To collect | To collect | legacy | observe then native | unvalidated |
| heating-01 | To collect | To collect | To collect | Actual temperature or absent | Bounds/modes | To collect | legacy | native after P34 | unvalidated |
| garage-01 | To collect | To collect | Deterministic or pulse | Position/closed/obstruction, with evidence | Manufacturer procedure | To collect | legacy | P33 validated mode | unvalidated |
| action-01 | Stored manufacturer scenario; exact targets to qualify | To collect | Exact effects | Result/partial failure if available | Dependencies | To collect | legacy | P35 | unvalidated |

Extend to every element; do not collapse different commands into a single “Somfy supported” row.

## 4. Garage information

- Motor reference/radio type from documentation, invoice, or setup without dismantling.
- Exposed commands and effects: open, close, stop, or cyclic pulse.
- Feedback: end-stop measurement, position, plugin-calculated state, or no measurement.
- Obstruction detection: real sensor, accessible feedback, manufacturer behavior.
- Behavior for lost/duplicated commands, remote-control actions, and movement already in progress.
- Current HomeKit mode, critical automations, and test procedure with the owner present.

## 5. Heating and action information

- Each thermostat: actual sensor, model, units, setpoint, modes, schedule, override duration, and feedback.
- Each Switch: momentary trigger or maintained state; manufacturer scenario or direct command; every affected device, especially garage/heating.
- Schedules/conditions stored in TaHoma remain there until explicit equivalence is established.

## 6. Resource comparison — final validation (P31/P38)

Owner decision: collect comparable legacy and Rust measurements at final validation before removing the legacy installation. This measurement is deferred from P03 and does not block development.

Record date, duration, version, device count, connection state, and active functions. For every required process: RSS, PSS if available, CPU, and child processes; for Docker: current/peak cgroup memory separately. Keep raw data private and publish only a technical summary.

The Homebridge memory widget's total/available values do not measure Homebridge consumption alone. A single CPU-load screenshot is not a baseline.

## 7. Status definitions

| Status | Meaning |
|---|---|
| legacy | Function retained in Homebridge; native integration not delivered |
| fixture-tested | Mapping tested on anonymized data; hardware unvalidated |
| observe | Rust reading active, Rust commands/publication blocked |
| native-pilot | Control validated in a pilot; stability period in progress |
| native-validated | Hardware, UI, HomeKit, failures, and rollback validated |
| gap | Uncovered function; retaining legacy or a decision is required |

Removing Homebridge requires zero `legacy`/`gap` use cases; removing Node.js also requires validated native HAP transport.
