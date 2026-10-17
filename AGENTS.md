# Project instructions

## Documentation language

All project documentation must be written in **English**. This includes the README,
architecture, contracts, plans, prompts, agent instructions, session reports, code
comments, and pull request descriptions. Keep identifiers, commands, paths, and API
names unchanged when translating. The product UI may remain French as specified
in the architecture; the documentation language does not change that requirement.

## Start every session

1. Read [current state](docs/project-state.md), the [session guide](docs/prompts/start-ticket.md), and the requested ticket in the [plan](docs/superpowers/plans/2026-10-17-homebridge-rust.md).
2. Read the relevant parts of the [architecture](docs/architecture.md), [contracts](docs/contracts.md), and [Luna guide](docs/execution-luna.md), then the affected files and their callers.
3. Check Git status, the branch, existing changes, PRs, and ticket dependencies. A dependency that is only documented or in an unmerged PR is not available on `main`.
4. Execute **one ticket per session**. Do not start the next ticket, spawn subagents, or create other chats without an explicit request. The owner selects the session model; no automatic model selection.

The owner's explicit session instructions take precedence over repository conventions. Do not mechanically request approval that has already been given.

## Project and decisions to preserve

- Public repository: `https://github.com/magic5644/homebridge-rust`, owner `magic5644`, reference branch `main`.
- Target: QNAP TS-251+, Celeron J1900, 2 GB; TaHoma V2; io-homecontrol and RTS coverings, garage, two thermostats, and nine actions to qualify.
- Four planned Rust packages: `bridge-core`, `integration-tahoma`, `transport-homekit`, `bridge-server`. Static Svelte/TypeScript UI at P17.
- Docker amd64 and QNAP `.qpkg` distribution; no compilation on the NAS. HomeKit transport must be Rust: evaluate libraries at P04, or implement native HAP at P22 if needed. No JavaScript/TypeScript server or HAP-NodeJS fallback. Browser JavaScript/TypeScript is allowed; build it off-NAS and serve it with Rust. Minimize server memory/CPU with one process and bounded resources.
- Owner decision: defer legacy-versus-Rust resource measurements to final validation (P31/P38); they do not block P03. Do not report unmeasured gains.
- Compatibility is proven per device and use case. Keep Homebridge for functions not replaced; retaining it does not constitute a delivered Rust integration.
- Durable memory belongs in these versioned files, not only in chat or personal agent memory. Verify actual implementation state in Git and CI.

## Execute a ticket

- Use a dedicated `feat/pXX-topic` branch; inspect any existing ticket branch/PR before creating another. Preserve the owner's local changes.
- Reuse contract interfaces and existing conventions. Implement the complete minimum, without future features or extra abstractions.
- For nontrivial logic, write a behavior test, observe its relevant failure, then implement and verify. A fix also updates every affected caller and fixture.
- A shared contract change includes its consumers, documentation, and tests in the same ticket. Explain the impact; do not defer the correction to another session.
- Proceed without confirmation for ordinary authorized work. If an essential prerequisite is missing, identify it and complete independent work without inventing evidence.
- Before concluding, review the diff and required checks; list checks not run and explain why.

## Technical documentation and commands

- For libraries, frameworks, SDKs, APIs, CLIs, and cloud services, use Context7: `resolve-library-id` first unless an exact ID is provided, then `query-docs` with the full question. Select relevant official sources and explicit versions when known. If documentation is inaccessible, report the limit instead of inventing an API.
- On the owner's machine, read the environment's RTK file if present and prefix shell commands with `rtk` (`rtk proxy` for unsupported commands). Use native tools in CI; RTK is not a product dependency.
- Use `rg` for searches. Do not change global Git/Codex settings to solve a repository-specific issue.
- The Rust commands below become executable with P02; do not report success before creating the workspace and actually running them:

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo build -p bridge-server --release --locked
```

P02 creates `scripts/check.sh`, the entry point for these four checks, used locally and in CI. Frontend scripts are required only from the ticket that creates `web/package.json`.

## GitHub, CI, and deliveries

Read [.github/instructions/github-actions-memory.instructions.md](.github/instructions/github-actions-memory.instructions.md) for changes to workflows, build dependencies, packaging, or publication.

- Verify the repository and account before remote writes; use the existing GitHub session without copying a token into a file or command.
- A code ticket produces a branch, a focused commit, and a PR to `main`, with tests, limits, and a link to the plan ticket. Do not equate Pxx IDs with GitHub issue numbers.
- Startup prompts authorize committing, pushing the branch, and creating/updating its PR. No automatic merge, release publication, or production installation.
- Wait for CI results for the pushed commit; inspect and fix failures within the ticket's scope. If remote execution is inaccessible or still pending, say so explicitly and do not declare the ticket complete.
- No workflow deploys to the NAS or uses its home network. Hardware tests remain separate from public PRs.

## Data and hardware

- Fictional fixtures by default. Private data belongs in `private/`, secrets and runtime state outside Git; do not add `.gitignore` exceptions to publish them.
- `observe` and `legacy` modes prohibit commands and HomeKit publication from Rust.
- No automatic tests on real equipment, production restoration, pairing resets, or plugin changes without specific authorization.
- An uncertain physical command is not replayed. An estimated RTS position or unknown garage state is not a confirmed measurement.
- Secrets, PINs, cookies, gateway IDs, household names, and private captures must never appear in GitHub, CI logs, public reports, or documentation queries.

## End-of-session memory

- Write `docs/sessions/PXX.md`: goal, branch/PR, tested implementation commit, changes, commands/results, CI state, limits, and dependencies now available. Avoid self-referential hashes: cite the tested commit before adding the report or the last implementation commit.
- Update `docs/project-state.md` with the result and resumption point. Check off ticket items only with evidence.
- Distinguish `in_progress`, `implemented`, `ci_pending`, `ready_for_review`, `blocked`, and `done`. `done` requires the ticket criteria and verified integration on `main`; hardware/human tests remain mandatory where required.
- After the owner merges, the next session verifies the commit and CI, then reconciles state if the previous report still says `ready_for_review`.
- Conclude with the result, PR/commit, checks, and limits. Stop after this ticket; never start the next automatically.
