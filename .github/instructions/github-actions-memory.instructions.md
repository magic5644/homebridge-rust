---
description: Project memory for GitHub Actions checks and artifact publication.
applyTo: ".github/workflows/**/*.yml,.github/workflows/**/*.yaml,scripts/**,packaging/**"
---

# GitHub Actions Memory

The repository is public. Code validation and artifact publication have distinct scopes. These instructions are project decisions; their presence alone does not mean a workflow exists.

## Initial CI delivered by P02

- File `.github/workflows/check.yml`, workflow name `CI`, job `rust`, stable display name `Rust checks`.
- Triggers: `pull_request`, `push` to `main`, `workflow_dispatch`. Run checks for documentation-only PRs too while this job is intended as a required check; avoid path filters that leave it pending.
- GitHub-hosted Linux runner, explicit version compatible with selected actions. No NAS or home-network runner for public PRs.
- Validation permission: `contents: read`; no deployment, manufacturer-account, or production secrets.
- Pin third-party actions to full SHAs verified in their official repositories; add a readable version comment. Choose versions/runtimes from current documentation, not an old example.
- Checkout must not persist credentials for build steps. No write access is needed to compile/test.
- Toolchain from `rust-toolchain.toml`, dependencies from versioned `Cargo.lock`; `--locked` for clippy, tests, and build.
- Execute `sh scripts/check.sh`, also used locally; the four exact checks are defined in `AGENTS.md`.
- Initial job timeout: 20 minutes. Cancel earlier CI runs for the same workflow and branch/PR when superseded by a new commit; do not cancel an ongoing publication this way.
- P02 also checks the script with `sh -n`, exercises failure behavior, and proves actual Linux execution in Actions. YAML linting alone is not a green CI run.

## Evolving checks

- P17 adds frontend checks when the frontend exists: lockfile-based installation, type checks, tests, and static build. E2E tests use the simulated server.
- Contract and packaging tests join CI with their components. Initial tests do not claim nonexistent Somfy/HAP coverage.
- Caching is optional: add it for measured benefit with keys covering OS, toolchain, and lockfile. Never cache secrets, household databases, or HomeKit state.
- Keep check names stable; changing a required check requires inspecting the GitHub rules that expect it.
- A successful job on an old commit does not prove success on the latest PR commit.

## Untrusted PR code

- Use `pull_request` for builds/tests; do not execute PR-head code under `pull_request_target` with privileged permissions or secrets.
- Treat titles, bodies, branches, and other PR metadata as data. Never interpolate them directly into shell scripts.
- Fork PRs receive no production secrets. Do not change permissions to bypass a missing test step.
- Inspect logs/artifacts: no real configuration, tokens, PINs, household names, or backups.

These rules follow [GitHub secure-use documentation](https://docs.github.com/en/actions/reference/security/secure-use) and [pull_request_target risks](https://docs.github.com/en/actions/reference/security/securely-using-pull_request_target), consulted through Context7 on October 17, 2026.

## Publication at P30

- Release publication uses a separate workflow. It belongs to P30; P02 publishes no GHCR image, QPKG, or release.
- Planned destination: `ghcr.io/magic5644/homebridge-rust`. Docker amd64 is the QNAP priority; arm64 is secondary with its testing level identified.
- Artifacts: versioned/digest image, native archive, `.qpkg`, checksums, and licenses. Build for generic x86_64, without the build machine's `target-cpu=native`.
- Grant publication permissions only to jobs that need them, such as `packages: write` for GHCR or `contents: write` for a release; never globally for convenience.
- Verify anonymous package access when an image must be public; repository visibility and GHCR visibility are distinct.
- The product server and HomeKit backend are Rust-only; no Node sidecar/hybrid profile is planned. Browser assets may use JavaScript/TypeScript and are compiled off-NAS. A stable release requires planned acceptance evidence; no NAS deployment on a simple push/tag.

## GitHub settings and memory

- Date remote observations in `docs/project-state.md`. Revalidate before relying on them; remote settings can change independently of the repository.
- File instructions do not create mandatory checks or branch protections.
- After the first green CI, propose check names to require on `main`. Changing rules/permissions requires an explicit request; recording memory does not modify them.
- Issues/Projects availability does not imply Pxx issues exist; keep only verified links.
