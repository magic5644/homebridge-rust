# Research and verification — October 17, 2026

## Method and limits

Official documentation and project repositories were inspected read-only. Context7 was used for Axum, Svelte, and Docker: `resolve-library-id`, official-project selection, then `query-docs` for a lightweight NAS home-automation Rust server. Other specific details were cross-checked against primary documentation. Exact versions are pinned as development starts.

Selected Context7 IDs: `/tokio-rs/axum`, `/sveltejs/svelte`, `/docker/docs`. Full search output is not embedded in the product; links below identify sources. P02 records its additional Rust/clap/Actions/rustup research in [dependencies](dependencies.md).

## Sources and associated decisions

| Primary source | Use in the proposal |
|---|---|
| [Homebridge](https://github.com/homebridge/homebridge) | Engine, plugin ecosystem, compatibility scope, possible Matter usage |
| [Homebridge UI](https://github.com/homebridge/homebridge-config-ui-x) | Administration functions to cover gradually |
| [HAP-NodeJS](https://github.com/homebridge/HAP-NodeJS) | Reference HomeKit transport and temporary fallback |
| [Child Bridges](https://github.com/homebridge/homebridge/wiki/Child-Bridges) | Coexistence, isolation, limits, distinct identities |
| [hap-rs](https://github.com/ewilken/hap-rs) | Rust candidate to evaluate, not an approved dependency |
| [Inspected hap-rs commit](https://github.com/ewilken/hap-rs/commit/acf45bdb00e23c81b67b3d5116c113335dc83f16) | Latest commit returned by repository API: April 14, 2023 |
| [Somfy developer guide](https://github.com/Somfy-Developer/Somfy-TaHoma-Developer-Mode) | Local authentication, certificate, discovery, recommended event tracking |
| [OpenAPI TaHoma](https://github.com/Somfy-Developer/Somfy-TaHoma-Developer-Mode/blob/main/docs/openapi.yaml) | Routes/verbs/capabilities to use; do not extrapolate undocumented scenarios |
| [homebridge-tahoma](https://github.com/dubocr/homebridge-tahoma) | Existing-option comparison and cloud/local identifier-change risks |
| [Axum](https://docs.rs/axum/latest/axum/) | HTTP server and Tokio/Tower integration |
| [Tokio — graceful shutdown](https://tokio.rs/tokio/topics/shutdown) | Task cancellation/tracking |
| [Svelte](https://svelte.dev/docs/svelte/overview) | Compiled UI without mandatory production Node server |
| [Vite — static deployment](https://vite.dev/guide/static-deploy.html) | Asset build embeddable in Rust server |
| [reqwest](https://docs.rs/reqwest/latest/reqwest/) | Reused async client, TLS/proxy configuration |
| [rusqlite](https://docs.rs/rusqlite/latest/rusqlite/) | SQLite and consistent backup |
| [mdns-sd](https://docs.rs/mdns-sd/latest/mdns_sd/) | Local-service discovery |
| [Docker host networking](https://docs.docker.com/engine/network/drivers/host/) | Linux host-network constraints |
| [Docker Compose services](https://docs.docker.com/reference/compose-file/services/) | Network, user, volume options |
| [QNAP TS-251+](https://www.qnap.com/en/product/ts-251%2B/specs/hardware) | NAS target and hardware architecture |
| [QNAP QDK](https://github.com/qnap-dev/QDK) | Native QPKG distribution |
| [QDK guide](https://github.com/qnap-dev/QDK/blob/master/docs/QDK-Developer-Guide.md) | Lifecycle and packaging parameters |
| [Homebridge license](https://github.com/homebridge/homebridge/blob/latest/LICENSE) | Apache-2.0 at research time |
| [HAP-NodeJS license](https://github.com/homebridge/HAP-NodeJS/blob/latest/LICENSE) | Apache-2.0 at research time |
| [Homebridge UI license](https://github.com/homebridge/homebridge-config-ui-x/blob/latest/LICENSE) | MIT at research time |

## Local observations without private addresses

1. Empty project directory at startup; no old code to migrate.
2. GitHub authenticated as `magic5644`; owner explicitly requested a public repository.
3. Homebridge UI responsive; owner performed login.
4. UI: Homebridge 2.4.0, UI 5.29.0, Node 24.21.0; Docker, Ubuntu 22.04.5, x64. These describe the container, not QTS.
5. Visible plugin: homebridge-tahoma 2.2.64, child bridge.
6. Visible accessories: 11 coverings, 2 thermostats, 1 garage, 9 Switch. This proves neither models nor measurement accuracy.
7. TaHoma developer mDNS announcement present. HTTPS with announced Overkiz CA/DNS name validated; `GET /apiVersion` returns 200 with `protocolVersion=2026.4.5-7`.
8. No token used for this TaHoma test; no authenticated setup, physical commands, configuration changes, restarts, or updates.
9. No per-process measurements; memory/performance gains remain hypotheses to test.

Owner confirms QNAP TS-251+, Celeron J1900, 2 GB, TaHoma V2, io-homecontrol, and RTS. Gateway PIN, HomeKit identifiers, device names, and internal addresses are not recorded here.

## Explicitly separated inferences

- Reachable local API makes local integration plausible; it does not prove every function is accessible.
- Switch elements may represent scenarios; their nature remains unqualified.
- An old Rust HAP stack may require maintenance/review; a commit date alone does not establish a vulnerability.
- Rust may reduce some runtime costs; actual reduction depends on retained plugins, HAP, logs, and communication frequency.
- Total/available widget memory does not establish the current server's cost.

## Outstanding checks

Authenticated local TaHoma reading, device models, QTS/Container Station version, nine-action effects, garage state/safety, two-thermostat nature, Matter use, CPU/memory baseline, Rust HAP feasibility, and native J1900 installation. None is claimed as established.
