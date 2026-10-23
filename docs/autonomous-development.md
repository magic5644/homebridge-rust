# Autonomous first-version delivery

Owner authorization: October 23, 2026. This supersedes earlier ticket stopping,
no-delegation and no-automatic-merge conventions for this delivery goal.

## Objective and boundaries

Deliver the implemented features of the P01–P38 plan as a first functional version
the owner can install in an isolated test environment and evaluate. Continue
bounded software work without asking for routine approvals. The plan remains the
source of feature scope and contracts; do not call a simulator a delivered hardware
integration. Hardware qualification, endurance and production migration remain
separate evidence, prepared as reproducible owner tests when equipment is needed.

The coordinator selects eligible tasks, reviews code, fixes CI, commits/pushes,
creates PRs, merges green reviewed changes and verifies main CI. Use Git author
and committer `magic5644`, `9008942+magic5644@users.noreply.github.com`, configured
only for this repository. No global Git/Codex settings are changed.

Parallel workers use isolated worktrees and short, task-specific contexts.
They own one ticket's files/tests and return tested commits; the coordinator owns
shared contracts, root manifests/lockfile, plan/state reconciliation and integration.
Workers must not merge, touch another worktree or operate household equipment.
Documentation is English; progress and owner instructions are simple French.

P04's inspected backend remains rejected. Native Rust HAP development may proceed
under the owner's delegation; use maintained crypto libraries and independent
protocol tests. Keep missing security/interoperability evidence explicit. Do not
launch the rejected P04 bridge or silently disable its qualification tests.

No production install, pairing reset/import, Homebridge change, equipment command,
secret/public capture disclosure or stable release is authorized by this software
work. The owner will test the first build and decide adjustments. Reads of private
household files are unnecessary for fixture-based implementation and are excluded.

## Execution sequence and review focus

1. Reconcile P05 integration and update delivery policy.
2. P06 storage, P07 config/secrets and P10 discovery independently; integrate each
   with head CI and main CI, preserving shared manifests/interfaces.
3. P08/P09 authenticated simulated server alongside P11 validated TaHoma client;
   then mappings/events/execution and daemon orchestration.
4. P17/P23–P25 UI, P18/P19 commands/events, P20 projection and bounded P22 native
   transport stages; no unauthenticated control, replay or invented physical state.
5. P26–P30 backup/build/package/CI, P32 migration safeguards and P33–P36 required
   profiles/actions. Real-only unknowns are disclosed and retain legacy support.
6. Produce off-NAS test artifacts and an isolated installation/test checklist.
   Prepare P31/P37/P38 validation without claiming unrun measurements or trials.

Every nontrivial behavior starts with a failing behavior test. Check disk-full and
failed atomic replacement, secret redaction, hostile inputs, bounded queues,
unknown outcomes/no replay, stable identities, shutdown and strict mode boundaries.
Run ticket checks and required workspace checks; review diff and scope, then
create/review/merge PR and verify exact main commit CI. A hardware-dependent
checkbox stays open when only simulation is available; continue independent work.

## Compact checkpoint

- Baseline: `082bf62c8e63ea55fb8c0b4be0bb51301292fcd3`, P05 integrated; main CI
  [39994576717](https://github.com/magic5644/homebridge-rust/actions/runs/39994576717) passed.
- Coordination branch: `feat/autonomous-first-version` (policy/reconciliation).
- Next workers: P06 storage, P07 configuration, P10 discovery; no active workers yet.
- Resume by reading this checkpoint, project-state active rows, the chosen ticket,
  relevant contract sections and owned files. Do not replay the complete chat.
- This checkpoint is updated at integration boundaries; per-ticket reports retain
  implementation hashes/checks/limits. Automatic context compaction is supported;
  do not assume a tool can erase chat history or a separate chat is required.
