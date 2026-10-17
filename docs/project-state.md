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
| Ticket session result | [P02 report](sessions/P02.md); future `docs/sessions/PXX.md` |

Pxx values are Markdown plan IDs, not GitHub issue numbers. Add an issue/PR link only after creating or verifying it.

## Verified delivery state

| Item | State |
|---|---|
| P01 — repository/design | `done`; design published on `main`, initial reference `dec76c0` |
| Instructions/session memory | Versioned; English-language policy and translation included in P02 at owner request |
| P02 — Rust workspace/CI | `ready_for_review`; [PR #1](https://github.com/magic5644/homebridge-rust/pull/1), branch `feat/p02-workspace-ci`, tested implementation `99e6e43500e791140a233ed6c56a04e8a352b170`; not merged |
| P03 — detailed inventory | Not started; waits for P02 merge; previous visual inspection exists, authenticated TaHoma inventory still needed |
| P04 — HAP decision | Not started; waits for P02 merge; no native backend approved |
| P05 and later | Not started; follow plan dependencies |
| Application/tests/CI | Minimal CLI and six automated tests delivered in P02 PR; no native compatibility or memory gain demonstrated |

P02 provides four packages (three empty core/integration/transport baselines plus CLI), pinned Rust 1.99.1 and clap 4.6.7/lockfile, MIT license, `scripts/check.sh`, and Linux Actions. `check --config PATH` only verifies UTF-8 readability without writing; schema validation belongs to P07. No device/network implementation exists.

Local `sh scripts/check.sh`, shell syntax, and actionlint passed. [Implementation CI](https://github.com/magic5644/homebridge-rust/actions/runs/39028568989) passed `Rust checks` on Ubuntu 24.04 for `99e6e43`. The final documentation commit must also pass; consult [current PR checks](https://github.com/magic5644/homebridge-rust/pull/1/checks), rather than treating the implementation run as proof of a later commit.

## GitHub observations

Read-only API observations from October 17, 2026:

- Verified owner/name: `magic5644/homebridge-rust`; public; default branch `main`.
- Issues/Projects enabled; no issue breakdown or board configured by these sessions.
- Actions enabled; previously returned policy `allowed_actions=all`, `sha_pinning_required=false`. Repository instructions enforce pinning independently and do not change remote policy.
- No ruleset returned by the earlier API inspection. Classic branch protections were not audited.
- `.github/workflows/check.yml` now exists in P02 PR: workflow `CI`, stable job `Rust checks`, PR/push main/manual triggers. No workflow exists on `main` until merge.
- Suggested required check after merge: **Rust checks**. No remote protection, permission, or runner setting changed.

Authentication: existing GitHub CLI session works as `magic5644`. A previous system credential-helper push stalled; the per-command GitHub CLI helper allowed synchronization. P02 also used that per-command helper without exposing a token or changing global configuration.

## Established technical facts

- Previous UI observation: Homebridge 2.4.0, UI 5.29.0, Node 24.21.0, homebridge-tahoma 2.2.64 child bridge; Ubuntu 22.04.5 x64 container.
- 23 visible elements: 11 coverings, 2 thermostats, 1 garage, 9 actions displayed as Switch; qualify actual capabilities.
- TaHoma developer API detected; HTTPS with validated CA/DNS; public `apiVersion` read succeeded, protocol `2026.4.5-7`.
- No TaHoma token used for that test, no physical commands, no Homebridge/NAS modifications. P02 performed no hardware tests.

Internal addresses, PINs, tokens, and household names are excluded from this public memory.

## Next session

1. After the owner merges P02, verify its commit and successful CI on `main`, then reconcile P02 from `ready_for_review` to `done`.
2. Select **P03** (inventory/baseline) or **P04** (HAP feasibility); both become eligible only after verified P02 integration. Neither started in this session.
3. P03 needs explicitly authorized local read access and a locally supplied token; report missing access without inventing captures/measurements. No physical commands.
4. Update only the selected ticket's proven results, report, and plan checkboxes. Simulated tests are not hardware proof; a ready PR is not a merge; a written workflow is not a successful run.
