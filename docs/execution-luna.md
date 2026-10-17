# Autonomous execution guide — Luna agents

## 1. Purpose

Implement one bounded task from the [plan](superpowers/plans/2026-10-17-homebridge-rust.md), without rediscovering the architecture or expanding scope. The target server is a QNAP TS-251+ with 2 GB, the gateway is a TaHoma V2, and use cases include io-homecontrol/RTS, garage, heating, and nine actions to qualify.

Luna suits fixture-based mappings, specified API routes, screens, tests, and isolated packaging tasks. HAP decisions, cryptography, physical garage tests, and production migration require competent human review. An agent must not remove a safeguard merely to pass a test.

## 2. Context to recover for each ticket

Do not copy the entire chat history. The agent finds the following in the repository:

1. The ticket ID and full text in the plan; verify the starting commit in Git.
2. `docs/architecture.md`, `docs/contracts.md`, this guide, and relevant sections of the anonymized inventory.
3. Files owned by the task and exports of already merged dependencies.
4. Component fixtures/tests and consumers of any modified interface.
5. Verification commands and expected evidence, including any outstanding hardware tests.

Run tasks from the repository root. Check applicable `AGENTS.md` and environment rules. On the owner's machine, prefix shell commands with `rtk` as required by the RTK file. In CI, use native commands without adding RTK as a product dependency.

For library/API/CLI questions: start with Context7 `resolve-library-id`, choose relevant official documentation, then `query-docs` with the full question. If no results are available, use official sources and report the limit. Never include household identifiers in external queries.

All documentation, comments, prompts, reports, and PR descriptions must be in English.

## 3. Startup prompt and session rule

The [startup prompt](prompts/start-ticket.md) is the single source to copy. Replace only `PXX` with the chosen ID; the first development ticket is P02. Infer files, dependencies, and fixtures from the ticket and verify what actually exists.

The owner requires **one ticket per session**. The session works autonomously on that ticket, creates/updates its PR, verifies CI, and stops. Do not move to the next ticket, delegate, or create another chat without an explicit request.

Read [AGENTS.md](../AGENTS.md), [current state](project-state.md), and [GitHub Actions instructions](../.github/instructions/github-actions-memory.instructions.md). The owner selects Luna from models actually available in the environment; the repository does not impose an outdated model reference.

## 4. Autonomy and concrete limits

### The agent may proceed without interruption

- Read/edit ticket files, write tests, and complete fictional examples.
- Consult official documentation/source and reuse the selected stack.
- Build locally or in CI and run simulated servers on loopback.
- Create its branch and commits in an isolated workspace when the environment permits.
- Fix callers and tests affected by authorized changes; explain any additional files.
- Measure the simulator, detect a leak, propose the minimum fix, and verify it.

### The agent prepares but does not perform on its own

- Read private backups/configuration outside explicitly provided context.
- Enable developer mode, create/revoke a token, or modify existing pairing.
- Send a command to a covering, heating system, scenario, or garage door.
- Install a package on the production NAS, restart an existing service, or disable a plugin.
- Move HomeKit identities, reset a bridge, or restore real data.
- Publish captures, room/person names, internal addresses, PINs, keys, raw logs, or detailed inventories on GitHub.

These limits apply to standard implementation work. An explicit owner instruction for a specific operation provides authorization and must not be requested again mechanically. Physical garage operations follow manufacturer procedures and require appropriate human presence.

## 5. Contracts and shared files

- Exact contract types are the shared source of truth. Check all callers before modifying them.
- An API change includes OpenAPI, Web client, fixtures, HAP projection, and affected tests in the same PR.
- `Cargo.lock`, `lib.rs` exports, the mapping dispatcher, and contracts must not be edited concurrently by multiple agents.
- Propose cross-cutting changes to the integration lead, who reserves files or sequences tickets.
- Integrate one PR at a time and run CI on the combined changes.

Do not generate universal support based on product-name families. Add a capability only with declared commands, test data, defined UI/HomeKit output, and a real validation procedure.

## 6. Task completion criteria

- Exact ticket behavior demonstrated by reproducible evidence.
- Relevant negative test passed: errors, unavailability, limits, or missing capability as appropriate.
- No known caller regressions; required formatting, compilation, and tests passed.
- No secrets or real data added; no automatic test commands a real device.
- Documentation and coverage matrix updated; simulated, hardware-validated, and legacy states remain distinct.
- Focused diff, no unused code, no dependency for a simple standard-library operation.
- Review completed before merge, with particular attention to hardware assumptions.

## 7. Expected report

Save the report in `docs/sessions/PXX.md` and summarize it in `docs/project-state.md`. It is part of the ticket PR. A ready PR remains `ready_for_review` until integration is verified; it is not a completed ticket on `main`.

```text
Ticket: Pxx
Result: achieved behavior, in one sentence.
Commit: identifier.
Changed files: short list.
Produced/modified interfaces: names and impact on consumers.
Checks: command, result, and duration if useful.
Hardware actually tested: none / authorized model and scenario.
Compatibility: native simulated / native validated / legacy / blocked.
Limits: remaining assumption, missing evidence, and owning ticket.
Next available dependency: ticket/API now usable.
```

Compilation alone is not evidence of HomeKit or Somfy compatibility. A Homebridge screenshot does not prove that a covering moved. Command acceptance does not prove that the garage is closed.
