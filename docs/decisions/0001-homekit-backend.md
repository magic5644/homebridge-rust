# ADR 0001 — Rust HomeKit backend evaluation

Date: October 18, 2026. Ticket: [P04](../superpowers/plans/2026-10-17-homebridge-rust.md#p04--rust-homekit-decision-maximum-three-day-budget).

## Decision: `stop-and-review`

No backend is approved. The evaluated accessory stack requires dependency,
validation, persistence and resource-control work. Apple Home interoperability,
a complete protocol test suite and competent human HAP/security review are
missing. Compilation does not satisfy those gates. P21 is unavailable; P22 is a
**proposed implementation path**, conditional on human review changing this
ADR to `native-build-required`. Neither ticket was implemented here. P05's P04
completion dependency remains unsatisfied. On October 23, 2026, the owner explicitly
authorized only P05 core types and tests despite this dependency. This limited
exception does not complete P04, approve a HomeKit backend, authorize launching
the experiment, or waive later security, interoperability or hardware gates.

The server, integrations and HomeKit transport remain Rust-only. No Node or
JavaScript server fallback is permitted. Existing Homebridge remains untouched
for current household functions. Browser assets are outside this decision.

This is a bounded evaluation and implementation plan within P04's maximum
three-engineering-day budget, not delivery of a new HAP stack. More research,
protocol implementation, pairing and review require separately scoped work;
time spent waiting for evidence does not manufacture approval.

## Candidates and source provenance

Observed on October 18, 2026; repository default-branch commits were checked
through GitHub and the accessory source was cloned for inspection.

| Candidate | Pinned evidence | Disposition |
|---|---|---|
| `ewilken/hap-rs`, manifest `0.1.0-pre.15` | [`acf45bdb00e23c81b67b3d5116c113335dc83f16`](https://github.com/ewilken/hap-rs/tree/acf45bdb00e23c81b67b3d5116c113335dc83f16), April 14, 2023 | Evaluated with the isolated experiment; not suitable for approval as shipped |
| `bobrik/hap-rs`, `rJunx/hap-rs` default branches | Both resolve to the same `acf45bd` commit | No independent fix at those heads; recent repository push timestamps are not maintenance evidence. Other branches/forks are not exhaustively audited |
| `phunapps/hap-rust` | [`8174cc3c7b7500f9dff10aacbe4a089a7b613c97`](https://github.com/phunapps/hap-rust/tree/8174cc3c7b7500f9dff10aacbe4a089a7b613c97); [controller crate documentation](https://docs.rs/crate/hap-controller/3.0.4) | Controller-side library, explicitly excludes accessory-side serving; potential future independent test controller, not a backend replacement. Not compiled or approved here |

The bounded search does not prove that every Rust HAP project is unsuitable.
No currently inspected candidate has the evidence required for approval.
The candidate has MIT/Apache-2.0 licensing; no upstream implementation is copied
into the product. Its source stays in an ignored experiment directory.

Context7 was called using `resolve-library-id` for `hap-rs`, then `query-docs`
on the returned `/websites/rs`; the query returned no matching HAP documentation.
A second resolution for `ewilken/hap-rs` returned unrelated Go/Python products,
which were rejected. Primary pinned source was used instead. Context7 resolved
`RustCrypto` and `query-docs` on `/rustcrypto/aeads` supplied AEAD guidance;
this is primitive documentation, not HAP protocol or security approval.

## Targeted audit and required fixes

All source references below are relative to the pinned accessory candidate;
[the source tree](https://github.com/ewilken/hap-rs/tree/acf45bdb00e23c81b67b3d5116c113335dc83f16/src)
and the experiment's `candidate.patch` make this reproducible.

| Area | Evidence and limit | Required work before approval |
|---|---|---|
| Toolchain/dependency resolution | Rust 1.99.1: unmodified manifest fails because `get_if_addrs-sys` and `if-addrs-sys` both link `ifaddrs` | Experiment aliases `get_if_addrs` to `if-addrs` 0.6; do not silently apply this as a product dependency |
| Crypto/dependency maintenance | Experiment lockfile: `ed25519-dalek` 1.0.1, `x25519-dalek` 0.6.0, `curve25519-dalek` 2.1.3 and 3.2.1, `chacha20poly1305` 0.8.2, `srp` 0.5.0, `hkdf` 0.11.0, `zeroize` 1.3.0; upstream emitted 51 warnings locally | Update and assess advisory/licensing/MSRV status, validate HAP vectors and review `pair_setup.rs::verify_client_proof`. Old versions alone do not prove exploitability; no full advisory/security audit was performed |
| Malformed protocol data | `tlv.rs::decode` indexes/slices unchecked; offline truncated-input qualification fails with panic | Fallible bounded TLV decoder, fragmented/duplicate/truncated/empty-field tests and fuzzing, no panic or partial authenticated action |
| Characteristic values | `characteristic/mod.rs::set_value` leaves bounds checks commented out; target 101 accepted despite 0–100 metadata | Validate type/range/step/length before callbacks and updates; check every HTTP consumer and fixture |
| Persistence/privacy | `storage/file_storage.rs`: truncate-in-place writes, no explicit file mode/fsync/atomic replacement or process lock; debug logs include Config/Pairing | Transactional identity/pairing storage, private permissions, no secret Debug/logs, corrupt-state fail closed, crash/disk-full/concurrent-open tests. Experiment disables logging and uses private state; these do not repair upstream storage |
| Authorization/revocation | `pairings.rs::check_admin` exists; storage deletion test passes | Verify unauthenticated and non-admin rejection, last-admin semantics, active-session revocation, invalid signatures/proofs/tags, pairing retry limits. Storage removal is not protocol revocation evidence |
| Memory/CPU | `transport/tcp.rs` uses `mpsc::unbounded`; HTTP handlers aggregate bodies; no demonstrated end-to-end admission/backpressure bounds | Bound requests/connections/queues/events, deadlines and slow-reader behavior; measure steady-state and adversarial load on generic x86_64/J1900 before resource claims |
| Topology/IDs | `server/ip.rs` keeps an AID cache; removal increments `configuration_number` without saving Config in that path | Persist stable AID/IID/configuration changes atomically, refresh discovery and test additions/removals across restart; never reuse removed IDs accidentally |
| Test quality | Nine upstream unit tests pass after dependency fix, including one SRP proof case; six experiment tests pass | Add independent controller protocol tests, negative vectors, restart/revocation/topology tests and actual Apple Home evidence; known-defect characterization tests are not conformance |
| Shutdown | Upstream exposes `run_handle`; experiment drops futures on Ctrl-C | Prove mDNS withdrawal, socket/task cleanup, persistent state integrity, SIGTERM and bounded shutdown ≤10 s; cancellation alone is not proof |

The two-line experiment patch also exposes the existing TLV module for direct
offline tests. It adds no cryptographic implementation, bypass, or certificate
validation change. The experiment is a separate Cargo workspace and lockfile;
the four product packages and product lockfile are unchanged.

## Independent AI specialist review

The owner-requested [HAP/security review](../reviews/P04-hap-security.md) confirms
additional static-source rejection reasons: unauthenticated characteristic
routing, incomplete setup admission/retry accounting and active-session revocation,
encrypted framing/partial-write defects, handler length-check panics and retained
connection listeners. Locked crypto versions also match two published RustSec
advisories; remote exploitability is not demonstrated. The review records exact
source references, required tests and the persistence flush limitation.

These findings strengthen `stop-and-review`. They are not fixes or permission to
launch the bridge. The specialist is an AI agent; mandatory human review remains
outstanding. A human assessing a disposable trial must consider these findings,
not just the two original failing qualification tests.

## Evidence and approval gates

[The proof ledger](../hardware/hap-proof.md) is authoritative for checks, missing
access, measurements and owner-assisted steps. Generic Linux x86_64 compilation
is a separate CI step; it does not establish J1900/QNAP compatibility. No bridge
was advertised, no Apple pairing was performed, and no real device was commanded
in this session. Full encrypted protocol verification and human review remain
mandatory, even if CI is green.

## Proposed P22 stages, subject to review

Retain `transport_homekit::run` from the contracts. Use one Rust process, Store
for persistent identity/IDs, the shared CommandRouter for writes, and bounded
cancellable tasks. No P22 modules are implemented by this spike.

| Stage | Bounded deliverable | Exit evidence / review gate |
|---|---|---|
| 0 — Protocol and primitive selection | Reviewer confirms lawfully available HAP revision, IP-only scope, source/vector provenance, maintained dependency versions/licenses/advisories and a costed plan | Human HAP/security approval before protocol implementation; stop if primitive/HAP compatibility cannot be established |
| 1 — Codec/discovery | Bounded HTTP/TLV8, supported fictional services, stable AID/IID/configuration numbers and mDNS lifecycle | Positive/negative/fuzz tests, fragmentation and oversize limits; no unauthenticated characteristic access |
| 2 — Pairing and secure sessions | Pair Setup SRP-6a/SHA-512, Pair Verify Ed25519/X25519, HKDF-SHA-512, ChaCha20-Poly1305 record framing with authenticated lengths and directional counters | Known HAP vectors, wrong proof/signature/tag/counter tests, retry limits, independent controller handshake; expert review of transcript/state transitions |
| 3 — Durable permissions | Atomic private identities/pairings, admin permissions, controller removal and last-admin handling | Crash/corrupt/disk-full/restart and active-session revocation tests; no identity reset on an unexplained storage error |
| 4 — Operations and resources | Reads/writes/events through cached Store/router, resync after event loss, ownership checks, deadlines/shutdown and backpressure | Protocol contract tests, slow-client/multiclient stress, no uncertain command replay, bounded RSS/CPU/task/fd measurements |
| 5 — Interoperability and release review | Isolated Apple Home switch/covering exercise, restart/topology/controller-removal evidence | Named competent human HAP/security sign-off tied to tested commit and scope; later J1900 and household acceptance remain P31/P37/P38 |

Proposed primitives: RustCrypto `srp`, `sha2`, `hkdf`, `chacha20poly1305`;
Dalek `ed25519-dalek`, `x25519-dalek`; OS-backed randomness and `zeroize`.
These are evaluation candidates, not newly pinned product dependencies. Use
library SRP/proof APIs and known HAP vectors; if a primitive cannot express the
required transcript safely, stop for expert selection rather than writing new
cryptography. Nonces must be unique per directional session key; failed AEAD
verification must never release plaintext. The [`srp` documentation](https://docs.rs/srp/0.6.0/srp/)
explicitly says it has no independent third-party audit; reuse does not eliminate
our security review requirement. [RustCrypto AEADs](https://github.com/RustCrypto/AEADs),
[Dalek](https://github.com/dalek-cryptography/curve25519-dalek),
and [HKDF documentation](https://docs.rs/hkdf/latest/hkdf/) provide primitive references.

[Apple's open-source ADK](https://github.com/apple/HomeKitADK/tree/fb201f98f5fdc7fef6a455054f08b59cca5d1ec8)
is an archived primary protocol/test reference, not a Rust implementation to embed.
No claim is made of access to the current MFi specification or certification tools.
Apple describes access through its [accessory development program](https://developer.apple.com/apple-home/).
The human reviewer must confirm the applicable protocol revision and distribution
requirements before an approval or certification claim. No certification is claimed.

Each stage needs its own estimate and review after stage 0; this ADR does not
promise completion of any HAP stack within P04's three-day budget.
