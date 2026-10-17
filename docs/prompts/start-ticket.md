# Start a ticket session

Open a new session in the `Homebridge-rust` project, select the desired model, and paste the prompt below. Replace only `PXX` with the desired ID. The first development ticket is **P02**; after its merge is verified, P03 or P04 becomes eligible.

```text
Implement only ticket PXX in this repository.

Read AGENTS.md, docs/project-state.md, and the PXX section of
docs/superpowers/plans/2026-10-17-homebridge-rust.md.
Then read the contracts, architecture, and files cited by the ticket.
For CI and GitHub, also apply
.github/instructions/github-actions-memory.instructions.md.

Verify Git state, PRs, and that prerequisites are actually available on main.
Work on a dedicated ticket branch, preserving my changes.
This session handles one ticket only, without automatic delegation.
Proceed autonomously with all ordinary authorized steps.

Implement the complete minimum with relevant tests and real checks.
Use only fictional test data and do not command real equipment.
Do not perform a production installation or publish a release.
Write all project documentation, comments, reports, and PR descriptions in English.

Once local checks pass, commit ticket files, push the branch, and create or
update a PR to main without merging it. Verify CI for the latest pushed
commit and fix failures related to the ticket. If essential evidence or a
prerequisite is inaccessible, explain precisely; never report an unrun check
as successful.

Update docs/project-state.md, proven plan checkboxes, and
docs/sessions/PXX.md; include them in the PR and verify the final commit too.
Cite the tested implementation commit in the report without trying to make
the report self-referential.

Conclude with the result, PR link, checks, and remaining limits.
Stop after this ticket. Do not start the next one.
```

## P02 specifics

P02 builds the first executable baseline: four minimal Rust packages, pinned toolchain and lockfile, minimal CLI, CLI tests, `scripts/check.sh`, and `.github/workflows/check.yml` executed on GitHub.

Instructions and memory already exist: extend them where necessary without rebuilding them or implementing Somfy/HAP features belonging to later tickets. The full frontend belongs to P17.

Selecting P02 authorizes its minimum implementation within the proposed architecture; do not ask again for workspace approval. HAP decisions, hardware tests, and migrations remain with their respective tickets.

For later tickets, verify merged dependencies. P03 requires local data/access; P04 requires distinct HomeKit evidence. Missing access must not be replaced with fixtures claimed to be captured or with invented hardware validation.
