# Project memory and current state

Last updated: October 23, 2026. This file is the session resumption point; code, commits, and CI take precedence when state has not yet been reconciled after a merge.

## Owner decisions

- **Public** GitHub repository under `magic5644`: [magic5644/homebridge-rust](https://github.com/magic5644/homebridge-rust).
- **Autonomous first-version delivery authorized October 23, 2026:** chain bounded sessions, parallelize independent work in isolated worktrees, review/merge green PRs and verify main CI. Use `magic5644` commit identity. See [delivery policy/checkpoint](autonomous-development.md). The owner-selected model is retained.
- Each session reads repository instructions, checks dependencies, and leaves a versioned report. GitHub Actions details live in [CI instructions](../.github/instructions/github-actions-memory.instructions.md).
- **All project documentation must be in English**, including instructions, plans, prompts, session reports, comments, and PR descriptions. Existing documents were translated in the P02 PR at the owner's explicit request. The planned French product UI remains a separate requirement.
- Final deployment: QNAP TS-251+, Celeron J1900, 2 GB; Docker and native QPKG installation.
- TaHoma V2, io-homecontrol/RTS; known uses include coverings, garage, heating, and virtual actions.
- Owner update: minimize server memory and CPU; the replacement server, integrations and HomeKit transport must be Rust-only. No JavaScript/TypeScript server, Node runtime or HAP-NodeJS fallback. Prefer one process, bounded work/memory and event-driven updates.
- Browser JavaScript/TypeScript is allowed; build assets off-NAS and serve them with Rust. A static browser frontend remains acceptable.
- Evaluate Rust HAP libraries at P04. If none fits, implement native HAP at P22 using established cryptographic libraries; retain interoperability/security validation before production use.
- Defer the legacy-versus-Rust resource comparison to final validation (P31/P38). The 30-minute initial measurement is not a P03 completion gate or prerequisite for development. Preserve comparable legacy operation until the final comparison.
- TaHoma must offer recommended local-token mode and explicit Somfy-account mode, implemented entirely in Rust. No silent cloud fallback. Token acquisition and secret handling: [access guide](integrations/tahoma-access.md). No token transfer to the agent is required.
- **Limited P05 exception (October 23, 2026):** the owner explicitly authorizes P05 core types/tests despite P04 remaining `stop-and-review`. This does not complete P04, approve any HomeKit backend or waive later security, interoperability or hardware requirements. No P04 experiment launch is authorized.
- Gradual migration preserving existing operation; no physical control or production changes in a standard coding session.

## Where to find information

| Need | Source |
|---|---|
| Startup instructions | [AGENTS.md](../AGENTS.md) |
| Scope and architectural choices | [Architecture](architecture.md) |
| Types, APIs, invariants | [Contracts](contracts.md) |
| Tickets/dependencies | [P01–P38 plan](superpowers/plans/2026-10-17-homebridge-rust.md) |
| Session startup command | [Reusable prompt](prompts/start-ticket.md) |
| Luna method/limits | [Execution guide](execution-luna.md) |
| Hardware/missing information | [Inventory](inventory-template.md) |
| Sources/research evidence | [Research](research.md) |
| Versions/licenses | [Dependencies](dependencies.md) |
| CI/publication rules | [GitHub Actions memory](../.github/instructions/github-actions-memory.instructions.md) |
| Ticket session result | [P02 report](sessions/P02.md), [P03 report](sessions/P03.md), [P04 report](sessions/P04.md), [P05 report](sessions/P05.md); future `docs/sessions/PXX.md` |

Pxx values are Markdown plan IDs, not GitHub issue numbers. Add an issue/PR link only after creating or verifying it.

## Verified delivery state

| Item | State |
|---|---|
| P01 — repository/design | `done`; design published on `main`, initial reference `dec76c0` |
| Instructions/session memory | Versioned; English-language policy and translation included in P02 at owner request |
| P02 — Rust workspace/CI | `done`; [PR #1](https://github.com/magic5644/homebridge-rust/pull/1) merged; remote `main` `2c0350c73f2411dabbe1a0d195928593ae566791`, exact-commit [CI passed](https://github.com/magic5644/homebridge-rust/actions/runs/39041606358) |
| P03 — detailed inventory | `done`; [PR #2](https://github.com/magic5644/homebridge-rust/pull/2) merged as `a9021b4ff71341e75f11939b535777bd634ea98b`; exact-commit [main CI passed](https://github.com/magic5644/homebridge-rust/actions/runs/39052106863), verified at P04 startup. Declared profiles, coverage and fictional fixtures integrated; physical qualification remains with later tickets; resource comparison deferred |
| P04 — HAP decision | Evaluation integrated via [PR #3](https://github.com/magic5644/homebridge-rust/pull/3), merged October 23 as `3b770b4a8b4af8163223524eb923a6f32b7809dc`; exact-commit push-to-main [CI passed](https://github.com/magic5644/homebridge-rust/actions/runs/39991787981), verified at P05 startup. **P04 acceptance remains incomplete**, development direction `native-build-required`; the inspected P04 backend is rejected and no production backend is approved. [ADR](decisions/0001-homekit-backend.md), [proof ledger](hardware/hap-proof.md) and [AI security review](reviews/P04-hap-security.md) remain authoritative. Apple Home interoperability, independent encrypted protocol evidence and mandatory competent human HAP/security review are missing. Earlier pre-merge reports are historical |
| P05 — shared types/invariants | `done`; [PR #4](https://github.com/magic5644/homebridge-rust/pull/4) merged as `082bf62c8e63ea55fb8c0b4be0bb51301292fcd3`; exact main [CI passed](https://github.com/magic5644/homebridge-rust/actions/runs/39994576717), four local workspace checks passed on merged result. [Report](sessions/P05.md) reconciled |
| P07 — configuration/secrets | `done`; [PR #6](https://github.com/magic5644/homebridge-rust/pull/6) merged as `22807985d781be3758135812a79f5f4c34a6ce1c`; exact main [CI passed](https://github.com/magic5644/homebridge-rust/actions/runs/39996760608). Private config/secret loading, atomic replacement and offline CLI check; [report](sessions/P07.md) |
| P10 — discovery | Integrated via [PR #7](https://github.com/magic5644/homebridge-rust/pull/7) as `ec62b2d2644c19730abb8ff66914b10783f5cfa6`; [head CI passed](https://github.com/magic5644/homebridge-rust/actions/runs/39997022605), exact main [CI passed](https://github.com/magic5644/homebridge-rust/actions/runs/39997461671), P10 is `done`. No actual mDNS scan or TLS/hardware proof; [report](sessions/P10.md) |
| P06 — SQLite Store | Integrated via [PR #8](https://github.com/magic5644/homebridge-rust/pull/8) as `40fcd11ff4ee0461e810a36ea89326f6d56a3f67`; head CI39998439204 passed; exact main CI39999065419 pending. Independent review and combined workspace checks passed. [Report](sessions/P06.md) |
| P11 — TaHoma HTTP/TLS | `implemented`; 21 new fictional HTTP/TLS/account tests and coordinator review passed; exact integration checks and PR/main CI tracked below. No real gateway/account compatibility proof. [Report](sessions/P11.md) |
| Autonomous delivery | `in_progress`; policy and P05 reconciliation prepared, foundations integrated, P08 auth and P22 native cryptographic stages under development. First delivery is a test build, not a claim of complete hardware qualification |
| Application/tests/CI | P02 minimal CLI and six tests integrated; P03 adds offline fictional-fixture validation. P05 branch adds 10 offline core contract tests. No native compatibility or memory gain demonstrated |

P02 provides four packages (initially empty core/integration/transport baselines plus CLI), pinned Rust 1.99.1 and clap 4.6.7/lockfile, MIT license, `scripts/check.sh`, and Linux Actions. P07 now makes `check --config PATH` validate schema offline without opening secrets or connecting. P06/P07/P10 and P11 provide library foundations; daemon orchestration is still pending. P03 changes no implemented runtime interface or dependency. Owner-requested future connection contracts now distinguish local-token/account modes; P05/P07/P11/P23 definitions were updated together, without starting them.

P02 integration was verified at P03 startup: [main CI](https://github.com/magic5644/homebridge-rust/actions/runs/39041606358) passed for `2c0350c73f2411dabbe1a0d195928593ae566791`. P03 validation and PR CI are recorded in [its report](sessions/P03.md); the owner subsequently ran the capture helper and the raw setup criterion is fulfilled. At P04 startup, PR #2 integration and exact-commit main CI were verified; its pre-merge report is historical.

P03 public artifacts: [coverage matrix](compatibility.md), [baseline status/protocol](performance-baseline.md), and [fixture manifest](../tests/fixtures_manifest.json). Ten examples are entirely fictional; detailed event payloads and profile combinations are not gateway-confirmed. Actual garage/heating/covering declarations are now documented separately; no native compatibility is declared.

## GitHub observations

Read-only API observations from October 17, 2026:

- Verified owner/name: `magic5644/homebridge-rust`; public; default branch `main`.
- Issues/Projects enabled; no issue breakdown or board configured by these sessions.
- Actions enabled; previously returned policy `allowed_actions=all`, `sha_pinning_required=false`. Repository instructions enforce pinning independently and do not change remote policy.
- No ruleset returned by the earlier API inspection. Classic branch protections were not audited.
- `.github/workflows/check.yml` is integrated on `main`: workflow `CI`, stable job `Rust checks`, PR/push main/manual triggers. P03 adds a Python standard-library fixture check to the same job.
- Suggested required check after merge: **Rust checks**. No remote protection, permission, or runner setting changed.

Authentication: existing GitHub CLI session works as `magic5644`. A previous system credential-helper push stalled; the per-command GitHub CLI helper allowed synchronization. P02 also used that per-command helper without exposing a token or changing global configuration.

## Established technical facts

- UI versions rechecked during the owner-authorized P03 follow-up: Homebridge 2.4.0, UI 5.29.0, Node 24.21.0, homebridge-tahoma 2.2.64 child bridge; Ubuntu 22.04.5 x64 container.
- All 23 Homebridge accessory dialogs inspected read-only: 11 `RollerShutter` projections, 2 `HeatingSystem`, 1 `GarageDoor`, 9 Switch projections with default model/manufacturer. All identify homebridge-tahoma; these are plugin model labels, not physical-model proof. Existing logs refine these to seven `PositionableRollerShutter`, four `PositionableTiltedRollerShutter`, two `AtlanticElectricalHeater`, one `PositionableGarageDoor`, and nine mapped stored scenarios. The owner-run local capture now confirms four technical profiles for all 14 known equipment projections; all are io and no RTS entry appears. Seven additional entries are infrastructure. No credential was inspected, stored by the helper or transferred to the agent. Actual product part numbers and numeric bounds are not supplied by the definitions; see the reviewed profile evidence.
- Plugin form: Somfy Europe/cloud, expose scenarios enabled, refresh 600 minutes; polling blank with a 60-second placeholder. Existing plugin log retained only in ignored private storage (0600); public summaries omit labels/identifiers. Source review of installed 2.2.64 warns that heating temperatures and garage states may be synthesized; scenario Off can cancel an in-progress execution.
- Owner confirms manufacturer scenarios mainly control coverings, others radiators, used via Apple Home/Siri. Read-only Apple Home inspection found five scenes (covering, garage and media dependencies) and seven automation entries (five disabled, two without targets). No scene/automation was executed, saved or enabled. Controller versions, actual Matter use and precise manufacturer scenario targets remain unknown.
- Owner-authorized QNAP UI inspection confirms TS-251+, J1900 (4 cores/4 threads), 2 GB RAM, QTS 5.2.10.3577 Build 20260731, and Container Station 3.1.2.1742. Single running Homebridge container: image/digest and reviewed runtime/network/storage facts in [baseline](performance-baseline.md). No settings or services changed. Instantaneous host metrics are observations only; the comparative process/cgroup measurement is deferred to final validation by owner decision.
- TaHoma developer API detected; HTTPS with validated CA/DNS; public `apiVersion` read succeeded, protocol `2026.4.5-7`.
- No TaHoma token used for that initial apiVersion test. Later, the owner entered a token privately into the read-only capture helper and produced `private/tahoma-setup.json` (0600). Its 21 entries were reviewed locally without publishing raw payload or current values. No physical commands or Homebridge/NAS modifications. P02 performed no hardware tests.

Internal addresses, PINs, tokens, and household names are excluded from this public memory.

## Resumption point

1. P02/P03 and P04 evaluation artifacts are integrated on `main`. PR #3 merge and exact main-commit CI were verified at P05 startup; P04 integration does **not** satisfy its acceptance gates.
2. Preserve P04's rejection evidence and updated `native-build-required` development [ADR](decisions/0001-homekit-backend.md), [proof ledger](hardware/hap-proof.md) and [security review](reviews/P04-hap-security.md). Green compilation/CI is not HomeKit approval. Do not launch the bridge.
3. P05 is integrated with exact main CI verified. Continue P08/P11 and P22 under the autonomous goal; see [checkpoint](autonomous-development.md). Do not start dependent runtime work until its prerequisites are integrated.
4. The owner supersedes the procedural stop before software development/review. P22 is the selected Rust-native development direction; the rejected P04 library is not approved. Independent encrypted protocol, Apple Home and human/security/hardware evidence remains absent and must not be invented. No production commands/pairings or legacy removal are authorized.
5. Future consumers must call the shared semantic validators, recheck ownership/availability before transmission, preserve unknown/estimated quality and redact internal/private fields. P07 owns configuration file loading and private permissions; P11 owns actual endpoint/authentication validation; P18/P20 own admission/publication enforcement. P07 CLI validation and P11 metadata/client use these types; command admission and publication are still pending.
6. Remaining device qualification stays with P11–P14/P33–P35: actual bounds/units, event/error bodies, physical feedback/safety and scenario membership. Preserve legacy paths until individually validated. Final legacy/Rust resource comparison remains P31/P38; no gains are measured here.
7. Keep `private/`, test identities and credentials local/excluded; preserve `.serena/`. No hardware, production pairing, Homebridge or NAS change is authorized by this report.
