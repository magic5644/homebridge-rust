# P04 — Independent AI HAP/security review

Date: October 18, 2026. Decision: **retain `stop-and-review`**.
Reviewer: dedicated AI subagent `hap_security_review`, explicitly requested by
the owner. This is an AI specialist review, **not the mandatory human sign-off**.
The primary agent checked the key source traces and advisory versions before
incorporating these findings. No fix or new HAP implementation is delivered.

## Scope and evidence

Reviewed PR [#3](https://github.com/magic5644/homebridge-rust/pull/3), implementation
`8db503ba00a8f605a514de279dff70ca8c2ed82b` and report/head
`dbd5b5838dcd91094fa042ed1b493efe1f2810a7`. Source references below are relative
to the pinned upstream [source tree](https://github.com/ewilken/hap-rs/tree/acf45bdb00e23c81b67b3d5116c113335dc83f16/src),
with only the experiment dependency alias and TLV visibility patch applied.
The downloaded source is ignored under `experiments/hap-proof/vendor/hap`.
These are candidate-library defects; the product does not depend on that library.

The review was read-only. No server, mDNS advertisement, controller pairing,
private household state access, equipment command or remote write occurred.
Unless explicitly described as reproduced by a test, findings are static source
observations, not a demonstrated network exploit or Apple Home failure.

## Findings

### P1 — Characteristic routes have no authenticated-session gate

`transport/tcp.rs:424–432` forwards plaintext TCP before session establishment.
`transport/http/server.rs:106–140` dispatches accessory/characteristic routes
without requiring an authenticated controller. The characteristic read and
write handlers discard `controller_id` at `handler/characteristics.rs:34/130`;
the write handler reaches `write_characteristic` at `:150–154`. The JSON handler
wrapper adds no authentication gate. Characteristic metadata permissions do not
establish caller identity. Reads/writes/subscriptions are therefore reachable
through the unauthenticated dispatch path. No network exploit was executed.

Required: require a verified, authorized session before protected routes or event
subscription; test unpaired/plaintext GET/PUT/subscription rejection and prove
that no write callback runs. Review against the applicable HAP revision and
[Apple's archived primary implementation](https://github.com/apple/HomeKitADK/blob/fb201f98f5fdc7fef6a455054f08b59cca5d1ec8/HAP/HAPIPAccessoryServer.c).

### P1 — Pair Setup admission and retry accounting are incomplete

`transport/http/handler/pair_setup.rs:162–175` leaves already-paired and
concurrent-pairing rejection unimplemented. A successful start resets the
failure counter at `:123–125`, and per-connection handlers reset its lifetime.
Thus retry accounting does not enforce an accessory-wide limit across new M1
requests/reconnections. This does not demonstrate pairing without the PIN.
`session.shared_secret` is also assigned at `:228` before the client proof is
accepted at `:230–231`; failed authentication must not retain usable setup state.

Required: coordinated setup admission, closed setup while paired as specified,
shared bounded retry/backoff accounting and terminal state clearing. Test wrong
proof/PIN across reconnects, concurrent setup and out-of-order transitions.

### P1 — Revocation does not close active sessions or handle last-admin changes

`handler/pairings.rs:215–225` checks admin status, deletes a record and emits
`ControllerUnpaired`. The connection listener in `http/server.rs:220–246`
handles characteristic events only. No active-session cancellation registry is
present. Characteristic requests do not recheck stored membership. The server
updates discovery when total pairing count reaches zero, without implementing
last-admin removal/demotion semantics when regular controllers remain.

Required: invalidate requests/events on already-open revoked sessions, enforce
last-admin semantics, and test admin removal/demotion with regular controllers
remaining. File deletion alone is not protocol revocation evidence.

### P1 — Encrypted record framing and partial writes lose transport state

`transport/tcp.rs:293/315` processes an encrypted record only when buffered
length equals exactly one frame. Coalesced frames can overshoot this equality;
there is no input payload-size check at `:313`. At `:463/472`, underlying write
results are discarded, while `:474` reports all plaintext consumed after counters
have advanced. Short writes or Pending can therefore lose ciphertext. The
outgoing loop at `:341–353` likewise does not retain a dequeued message on Pending
or its unwritten suffix. These defects are source-confirmed, not runtime-tested.

Required: bounded framing with retained leftovers, pending ciphertext/offsets,
correct counter advancement and explicit EOF/error handling. Test arbitrary TCP
fragmentation/coalescing, short writes, Pending, oversize frames and slow readers.

### P1 — Handler field-length validation remains unsafe beyond TLV decoding

The existing offline tests reproduce the truncated TLV decoder panic.
Independently, `handler/pair_verify.rs:129–131` slices an unchecked public-key
length, and `:198–199` subtracts a tag length from unchecked input. Pair Setup
uses the same slicing at `handler/pair_setup.rs:257–258`. A valid outer TLV can
still carry invalid field lengths. Empty-field indexing adds related cases.
These can panic connection tasks; process-wide termination is not established.

Required: exact field-length/type/state validation before indexing or crypto,
fallible decoding and handler-level negative tests/fuzzing. Fixing only the TLV
decoder would not close the rejection cases.

### P2 — Disconnect churn retains connection listeners

`http/server.rs:220` registers a listener per connection, capturing subscriptions
and an outgoing sender. `event.rs:15–29` appends and visits listeners but provides
no removal. Disconnect churn retains listeners and increases work for subsequent
events, in addition to the already documented unbounded queues/body aggregation.
`max_peers = 4` limits pairing records, not TCP connections or listeners.

Required: tie listener lifetime to connection cancellation; test repeated
connect/disconnect, bounded connection admission, task/fd counts and memory.

### P2 — Locked crypto versions match published advisory ranges

The experiment lockfile contains `curve25519-dalek` 2.1.3 and 3.2.1, affected by
[RUSTSEC-2024-0344](https://rustsec.org/advisories/RUSTSEC-2024-0344.html), and
`ed25519-dalek` 1.0.1, affected by
[RUSTSEC-2022-0093](https://rustsec.org/advisories/RUSTSEC-2022-0093.html).
This establishes affected package versions, not demonstrated remote key
extraction. The ed25519 advisory requires a signing-oracle/public-key-control
condition not demonstrated through this network interface. Upgrade/replace or
record a narrowly justified applicability assessment before approval. This is
not an exhaustive dependency/advisory audit.

### Additional persistence and session review requirements

`storage/file_storage.rs:102–107` returns success after a BufWriter write without
an explicit checked flush; drop-time flush errors can be lost. Checked flush/sync,
atomic replacement or transactional storage, locking and fault injection are
required. Existing private-directory precautions do not repair durability.

Favorable source observations: directional HKDF labels differ, AEAD authenticates
length bytes, and plaintext is released after tag verification. These do not
prove overall protocol correctness. Counter overflow, wrong tags/counters,
session failure/reset, secret erasure and low-order X25519 inputs need explicit
tests and expert assessment; no practical overflow exploit is claimed.

## Verification and disposition

The independent reviewer reproduced the required workspace checks successfully,
plus six passing offline probe tests/two ignored gates; explicitly running those
two qualification gates fails with exit 101. It verified source/patch provenance,
clean diff whitespace and successful [CI for dbd5b58](https://github.com/magic5644/homebridge-rust/actions/runs/39118670487).
Primary sources and RustSec advisories were consulted; Context7 returned no
relevant Rust HAP match, while RustCrypto AEAD documentation was resolved/queried.

Keep the bridge stopped pending competent human review of these findings and
any decision to permit a disposable isolated trial. A separate Apple Home is
not network isolation. Apple interoperability, complete independent encrypted
protocol tests, runtime resource measurements and human sign-off remain absent.
No finding is claimed fixed and no new acceptance checkbox is satisfied by this
AI review alone. The [ADR](../decisions/0001-homekit-backend.md) remains authoritative.

## Bounded next work for human assessment

Prepare an independently implemented controller test harness with these gates:
unauthenticated access denied; setup state/admission/retry limits; valid and invalid
SRP/signature/tag vectors; active-session revocation/last-admin semantics; HTTP
and encrypted-record segmentation/coalescing/short writes; persistence fault
injection; malformed handler fields; resource churn and shutdown. Do not use
candidate-internal success alone as independent conformance evidence.

Before P22 protocol implementation, its proposed stage 0 needs an applicable
protocol revision, vector provenance, maintained pinned crypto dependencies,
measurable limits, estimates and a named human reviewer. The owner is not asked
to learn Rust or personally certify protocol security. Choosing P22 still needs
the plan's decision/dependency gates; this report does not start it.
