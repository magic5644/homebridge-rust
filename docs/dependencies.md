# Versions and licenses — P02 baseline and P05 contracts

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

## P05 core additions

Versions checked against crates.io on October 23, 2026, with exact manifest pins
and the workspace lockfile. No HTTP, storage, TaHoma or HAP dependency is added.

- Serde **1.0.229** with `derive`: shared contract representations.
- UUID **1.29.0**, defaults disabled, `serde` only: contract aliases/string encoding;
  no generator, randomness, JavaScript or WebAssembly dependency is needed.
- Tokio **1.54.1**, defaults disabled, `sync` only: the exact `CommandEnvelope`
  oneshot sender; no executor, networking, timers or tasks are started.
- serde_json **1.0.154** was introduced for P05 JSON tests and is a core runtime dependency from P07 for private config parsing/writing. P07 adds no external crate; bridge-server now depends on bridge-core for offline configuration validation.

Context7 `resolve-library-id` then `query-docs` supplied relevant primary guidance:
[Serde enum representations](https://serde.rs/enum-representations.html),
[container attributes](https://serde.rs/container-attrs.html),
[UUID Serde implementation](https://docs.rs/uuid/1.29.0/uuid/serde/index.html),
and [Tokio oneshot](https://docs.rs/tokio/1.54.1/tokio/sync/oneshot/index.html).
The Context7 MCP endpoint was called directly because its tools were not exposed
in this session's tool catalog; resolution and documentation queries succeeded.
Public queries contained no household data. Primary registry metadata supplied
published versions; Context7 snippets alone were not treated as version evidence.

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
| bitflags | 2.13.2 | MIT OR Apache-2.0 |
| bumpalo | 3.20.3 | MIT OR Apache-2.0 |
| cc | 1.10.0 | MIT OR Apache-2.0 |
| cfg-if | 1.0.5 | MIT OR Apache-2.0 |
| clap | 4.6.7 | MIT OR Apache-2.0 |
| clap_builder | 4.6.7 | MIT OR Apache-2.0 |
| clap_derive | 4.6.7 | MIT OR Apache-2.0 |
| clap_lex | 1.1.1 | MIT OR Apache-2.0 |
| colorchoice | 1.0.5 | MIT OR Apache-2.0 |
| fallible-iterator | 0.3.0 | MIT/Apache-2.0 |
| fallible-streaming-iterator | 0.1.9 | MIT/Apache-2.0 |
| fastrand | 2.5.0 | Apache-2.0 OR MIT |
| find-msvc-tools | 0.1.15 | MIT OR Apache-2.0 |
| flume | 0.12.0 | Apache-2.0/MIT |
| foldhash | 0.2.0 | Zlib |
| futures-core | 0.3.34 | MIT OR Apache-2.0 |
| futures-sink | 0.3.34 | MIT OR Apache-2.0 |
| getrandom | 0.4.3 | MIT OR Apache-2.0 |
| hashbrown | 0.16.1 | MIT OR Apache-2.0 |
| hashlink | 0.11.1 | MIT OR Apache-2.0 |
| heck | 0.5.0 | MIT OR Apache-2.0 |
| if-addrs | 0.15.0 | MIT OR BSD-3-Clause |
| is_terminal_polyfill | 1.70.2 | MIT OR Apache-2.0 |
| itoa | 1.0.18 | MIT OR Apache-2.0 |
| js-sys | 0.3.106 | MIT OR Apache-2.0 |
| libc | 0.2.191 | MIT OR Apache-2.0 |
| libsqlite3-sys | 0.36.0 | MIT |
| lock_api | 0.4.14 | MIT OR Apache-2.0 |
| log | 0.4.34 | MIT OR Apache-2.0 |
| mdns-sd | 0.21.6 | Apache-2.0 OR MIT |
| memchr | 2.8.3 | Unlicense OR MIT |
| mio | 1.2.4 | MIT |
| once_cell | 1.21.4 | MIT OR Apache-2.0 |
| once_cell_polyfill | 1.70.2 | MIT OR Apache-2.0 |
| pin-project-lite | 0.2.17 | Apache-2.0 OR MIT |
| pkg-config | 0.3.34 | MIT OR Apache-2.0 |
| proc-macro2 | 1.0.107 | MIT OR Apache-2.0 |
| quote | 1.0.47 | MIT OR Apache-2.0 |
| r-efi | 6.0.0 | MIT OR Apache-2.0 OR LGPL-2.1-or-later |
| rsqlite-vfs | 0.1.1 | MIT |
| rusqlite | 0.38.0 | MIT |
| rustversion | 1.0.23 | MIT OR Apache-2.0 |
| scopeguard | 1.2.0 | MIT OR Apache-2.0 |
| serde | 1.0.229 | MIT OR Apache-2.0 |
| serde_core | 1.0.229 | MIT OR Apache-2.0 |
| serde_derive | 1.0.229 | MIT OR Apache-2.0 |
| serde_json | 1.0.154 | MIT OR Apache-2.0 |
| shlex | 2.0.1 | MIT OR Apache-2.0 |
| smallvec | 1.16.4 | MIT OR Apache-2.0 |
| socket-pktinfo | 0.4.1 | MIT |
| socket2 | 0.6.5 | MIT OR Apache-2.0 |
| spin | 0.9.9 | MIT |
| sqlite-wasm-rs | 0.5.5 | MIT |
| strsim | 0.11.1 | MIT |
| syn | 3.0.9 | MIT OR Apache-2.0 |
| thiserror | 2.0.21 | MIT OR Apache-2.0 |
| thiserror-impl | 2.0.21 | MIT OR Apache-2.0 |
| tokio | 1.54.1 | MIT |
| tokio-macros | 2.7.2 | MIT |
| unicode-ident | 1.0.27 | (MIT OR Apache-2.0) AND Unicode-3.0 |
| utf8parse | 0.2.2 | Apache-2.0 OR MIT |
| uuid | 1.29.0 | Apache-2.0 OR MIT |
| vcpkg | 0.2.15 | MIT/Apache-2.0 |
| wasi | 0.11.1+wasi-snapshot-preview1 | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT |
| wasm-bindgen | 0.2.129 | MIT OR Apache-2.0 |
| wasm-bindgen-macro | 0.2.129 | MIT OR Apache-2.0 |
| wasm-bindgen-macro-support | 0.2.129 | MIT OR Apache-2.0 |
| wasm-bindgen-shared | 0.2.129 | MIT OR Apache-2.0 |
| windows-link | 0.2.1 | MIT OR Apache-2.0 |
| windows-sys | 0.61.2 | MIT OR Apache-2.0 |
| zmij | 1.0.23 | MIT |

## P10 discovery addition

`integration-tahoma` pins `mdns-sd =0.21.6` (Apache-2.0 OR MIT) with its async
feature and reuses the workspace Tokio version. Context7 returned unrelated Go
mDNS matches, so those results were rejected; versioned primary Rust docs/source
and the official Somfy developer OpenAPI were used instead. See the P10 report
for source URLs, features and limits. This adds no manufacturer HTTP client.

## P06 storage additions

Pinned rusqlite 0.38.0 uses bundled SQLite and its backup API; UUID v4 enables OS
randomness for persisted identifiers. Tokio runtime/macros/time are test-only
features for Store; production Store uses its sync channels and one OS thread.
Existing serde_json is reused at runtime. The all-target inventory above includes
WASM/UEFI conditional components which are not linked into the Linux NAS server.
The product remains one native Rust process with bundled SQLite, as planned.
