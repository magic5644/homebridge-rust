# Homebridge Rust

Proposal dated October 17, 2026, for `magic5644`: a resource-efficient home automation bridge with Web administration, NAS or Docker installation, and Somfy/TaHoma integration as the priority.

**Status: P02 CLI/workspace baseline; no home automation integration or migration performed.** Numerical targets remain criteria to measure, not achieved gains.

## Read the project documents

1. [Architecture and decisions](docs/architecture.md): scope, alternatives, technology, HomeKit, Somfy, installation, performance, and migration.
2. [Shared contracts](docs/contracts.md): data model, Rust interfaces, Web API, and shared rules.
3. [Implementation plan and tickets](docs/superpowers/plans/2026-10-17-homebridge-rust.md): execution order, files, tests, and completion criteria.
4. [Luna execution guide](docs/execution-luna.md): minimum context, autonomy, limits, and report format.
5. [Inventory template](docs/inventory-template.md): hardware, plugins, capabilities, and device coverage.
6. [Research and verification](docs/research.md): sources, Context7 documentation, and local observations without household identifiers.

## One session per ticket

Start with [current state](docs/project-state.md), then use the [startup prompt](docs/prompts/start-ticket.md) for the chosen ticket after verifying its dependencies. [Agent instructions](AGENTS.md) and [GitHub Actions memory](.github/instructions/github-actions-memory.instructions.md) are versioned for future sessions.

Each session implements its ticket, tests it, prepares a PR, and leaves a report; it does not automatically start the next ticket. The P02 executable baseline is described in [its report](docs/sessions/P02.md).

All project documentation and PR descriptions must be in English. The planned French product UI is a separate requirement.

## Recommended direction

Build the server, integrations and HomeKit transport in Rust, with minimum server memory and CPU overhead. JavaScript/TypeScript may be used in the browser UI; its build runs off-NAS and production serving uses Rust. Evaluate existing Rust HAP libraries first; if none is suitable, implement the transport in Rust using established cryptographic libraries. Keep the existing Homebridge only during migration for functions not yet replaced. No new JavaScript/TypeScript server or HAP-NodeJS fallback is planned.

The owner deferred the comparative resource baseline to final validation (P31/P38); it is not a P03 gate. Resource gains will be measured on comparable functions before legacy removal.

Compatibility with the entire household must be verified per device. Recompiling a JavaScript plugin does not make it a Rust plugin. An all-Rust edition will be delivered only after HomeKit validation and replacement of every required integration.

## Repository and confidentiality

Public repository: [magic5644/homebridge-rust](https://github.com/magic5644/homebridge-rust), as requested by the owner. The initial publication contained documents only. The account, visibility, and remote files were verified.

Never commit tokens, real Homebridge backups, TaHoma identifiers, device addresses, or HomeKit pairing data. Examples and tests use fictional values.

## Develop the P02 baseline

Install Rust **1.99.1** with rustfmt and clippy (rustup reads `rust-toolchain.toml`), then run from the repository root:

```sh
sh scripts/check.sh
cargo run -p bridge-server --locked -- --version
cargo run -p bridge-server --locked -- check --config /path/to/config.toml
```

`check` reads a UTF-8 file without modifying it and fails for a missing or inaccessible path or a directory. It does not yet validate a configuration schema, start a server, or contact equipment. The three integration/core/transport libraries are empty baselines for later tickets.

Versions and licenses: [dependencies](docs/dependencies.md). Project code is licensed under [MIT](LICENSE).
