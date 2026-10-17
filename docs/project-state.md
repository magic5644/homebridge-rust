# Project memory and current state

Last updated: October 17, 2026. This file is the session resumption point; code, commits, and CI take precedence when state has not yet been reconciled after a merge.

## Owner decisions

- **Public** GitHub repository under `magic5644`: [magic5644/homebridge-rust](https://github.com/magic5644/homebridge-rust).
- **One ticket per session**, owner-selected model; Luna is intended for bounded work. No automatic ticket chaining or delegation.
- Each session reads repository instructions, checks dependencies, and leaves a versioned report. GitHub Actions details live in [CI instructions](../.github/instructions/github-actions-memory.instructions.md).
- **All project documentation must be in English**, including instructions, plans, prompts, session reports, comments, and PR descriptions. Existing documents were translated in the P02 PR at the owner's explicit request. The planned French product UI remains a separate requirement.
- Final deployment: QNAP TS-251+, Celeron J1900, 2 GB; Docker and native QPKG installation.
- TaHoma V2, io-homecontrol/RTS; known uses include coverings, garage, heating, and virtual actions.
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
| Ticket session result | [P02 report](sessions/P02.md), [P03 report](sessions/P03.md); future `docs/sessions/PXX.md` |

Pxx values are Markdown plan IDs, not GitHub issue numbers. Add an issue/PR link only after creating or verifying it.

## Verified delivery state

| Item | State |
|---|---|
| P01 — repository/design | `done`; design published on `main`, initial reference `dec76c0` |
| Instructions/session memory | Versioned; English-language policy and translation included in P02 at owner request |
| P02 — Rust workspace/CI | `done`; [PR #1](https://github.com/magic5644/homebridge-rust/pull/1) merged; remote `main` `2c0350c73f2411dabbe1a0d195928593ae566791`, exact-commit [CI passed](https://github.com/magic5644/homebridge-rust/actions/runs/39041606358) |
| P03 — detailed inventory | `blocked` on remaining capability/action/resource evidence; [draft PR #2](https://github.com/magic5644/homebridge-rust/pull/2), branch `feat/p03-inventory-baseline`, delivers fictional fixtures, 23 individually inspected Homebridge projections and baseline protocol; see [P03 report](sessions/P03.md). Raw TaHoma capabilities, action classification and real measurements remain missing |
| P04 — HAP decision | Not started; P02 dependency available; no native backend approved |
| P05 and later | Not started; follow plan dependencies |
| Application/tests/CI | P02 minimal CLI and six tests integrated; P03 adds offline fictional-fixture validation. No native compatibility or memory gain demonstrated |

P02 provides four packages (three empty core/integration/transport baselines plus CLI), pinned Rust 1.99.1 and clap 4.6.7/lockfile, MIT license, `scripts/check.sh`, and Linux Actions. `check --config PATH` only verifies UTF-8 readability without writing; schema validation belongs to P07. No device/network implementation exists. P03 changes no runtime interfaces or dependencies.

P02 integration was verified at P03 startup: [main CI](https://github.com/magic5644/homebridge-rust/actions/runs/39041606358) passed for `2c0350c73f2411dabbe1a0d195928593ae566791`. P03 validation and PR CI are recorded in [its report](sessions/P03.md); a green fixture check does not complete the missing installation work.

P03 public artifacts: [coverage matrix](compatibility.md), [baseline status/protocol](performance-baseline.md), and [fixture manifest](../tests/fixtures_manifest.json). Ten examples are entirely fictional; detailed event payloads and profile combinations are not gateway-confirmed. Garage/heating/action profiles remain prerequisites, not declared support.

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
- All 23 Homebridge accessory dialogs inspected read-only: 11 `RollerShutter` projections, 2 `HeatingSystem`, 1 `GarageDoor`, 9 Switch projections with default model/manufacturer. All identify homebridge-tahoma; these are plugin model labels, not physical-model proof. Actual capabilities, action effects and radio split remain unqualified. No credentials or raw configuration inspected.
- TaHoma developer API detected; HTTPS with validated CA/DNS; public `apiVersion` read succeeded, protocol `2026.4.5-7`.
- No TaHoma token used for that test, no physical commands, no Homebridge/NAS modifications. P02 performed no hardware tests.

Internal addresses, PINs, tokens, and household names are excluded from this public memory.

## Resumption point

1. Review the P03 partial-delivery PR and [session evidence](sessions/P03.md). P02 is `done`; do not repeat its pending-merge gate.
2. Resume **P03 only** using the owner-authorized existing Homebridge UI and privately collected evidence for remaining raw capabilities, action/Apple Home usage, and a complete production resource report or read-only measurement access. The owner keeps the TaHoma token; token transfer is not required. No credentials in chat, Git, CI, or documentation queries.
3. Collect missing QTS/Container Station/topology/plugin facts, qualify the remaining physical capabilities and nine Switch effects, and measure the unchanged required process set after warmup for 30 minutes. No physical commands or production changes.
4. P03 remains incomplete even if this preparatory PR is merged. Do not mark P05's P03 dependency available until the missing evidence is reviewed. P04 is independently eligible but was not started.
5. Preserve the distinction between fictional tests, historical UI observations, actual hardware evidence, CI success and integration on `main`.
