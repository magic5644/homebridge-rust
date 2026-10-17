# Versions and licenses — P02

Versions verified on October 17, 2026. Project code is under the [MIT license](../LICENSE).
No Homebridge, HAP, or manufacturer code is reused in this baseline.

## Minimal choices

- Rust **1.99.1**, edition **2024**, resolver **3**: stable version verified in the
  [official manifest](https://static.rust-lang.org/dist/channel-rust-1.99.1.toml).
  `rust-toolchain.toml` pins the channel, minimal profile, rustfmt, and clippy.
  The Rust compiler is distributed under MIT OR Apache-2.0.
- clap **4.6.7**, `derive` feature, MIT OR Apache-2.0: the only direct dependency,
  limited to the CLI binary. Exact version in `Cargo.toml`; transitives/checksums
  in `Cargo.lock`. No async runtime, HTTP, storage, or HAP backend at P02.
- All four local packages are version **0.1.0**, not publishable to crates.io.
- CI: **ubuntu-24.04** runner, `actions/checkout` **v7.0.1** (MIT), SHA
  `3d3c42e5aac5ba805825da76410c181273ba90b1`, verified in the
  [official repository](https://github.com/actions/checkout/releases/tag/v7.0.1).
  Node 24 action runtime; compatible GitHub-hosted runner.
  Preinstalled rustup selects the toolchain from the versioned file;
  argument-free `rustup toolchain install` installs it if missing.

Documentation consulted through Context7 (`resolve-library-id`, then `query-docs`):
[clap derive](https://docs.rs/clap/4.6.7/clap/_derive/_tutorial/index.html),
[Cargo workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html),
[checkout](https://github.com/actions/checkout),
[rustup](https://rust-lang.github.io/rustup/overrides.html),
[Actions concurrency](https://docs.github.com/en/actions/how-tos/deploy/configure-and-manage-deployments/control-deployments).
Published versions were also checked against crates.io, static.rust-lang.org, and
GitHub API; Context7 examples alone do not prove the latest version.

## Locked dependencies

Table derived from `cargo metadata --format-version 1 --locked`, including all targets.
License values are each component's declared SPDX expression, not a replacement
for its license text. crates.io archives retain original licenses/notices;
no third-party source is copied or vendored into this repository.
P30 must include those texts/notices with redistributed artifacts and recheck
this list after dependencies from later tickets are added.

| Component | Exact version | Declared license |
|---|---|---|
| anstream | 1.0.0 | MIT OR Apache-2.0 |
| anstyle | 1.0.14 | MIT OR Apache-2.0 |
| anstyle-parse | 1.0.0 | MIT OR Apache-2.0 |
| anstyle-query | 1.1.5 | MIT OR Apache-2.0 |
| anstyle-wincon | 3.0.11 | MIT OR Apache-2.0 |
| clap | 4.6.7 | MIT OR Apache-2.0 |
| clap_builder | 4.6.7 | MIT OR Apache-2.0 |
| clap_derive | 4.6.7 | MIT OR Apache-2.0 |
| clap_lex | 1.1.1 | MIT OR Apache-2.0 |
| colorchoice | 1.0.5 | MIT OR Apache-2.0 |
| heck | 0.5.0 | MIT OR Apache-2.0 |
| is_terminal_polyfill | 1.70.2 | MIT OR Apache-2.0 |
| once_cell_polyfill | 1.70.2 | MIT OR Apache-2.0 |
| proc-macro2 | 1.0.107 | MIT OR Apache-2.0 |
| quote | 1.0.47 | MIT OR Apache-2.0 |
| strsim | 0.11.1 | MIT |
| syn | 3.0.9 | MIT OR Apache-2.0 |
| unicode-ident | 1.0.27 | (MIT OR Apache-2.0) AND Unicode-3.0 |
| utf8parse | 0.2.2 | Apache-2.0 OR MIT |
| windows-link | 0.2.1 | MIT OR Apache-2.0 |
| windows-sys | 0.61.2 | MIT OR Apache-2.0 |
