# P04 HAP proof ledger and isolated pairing procedure

Date: October 18, 2026. Decision: **`stop-and-review`**.
See [ADR 0001](../decisions/0001-homekit-backend.md) for pinned sources, defects,
proposed P22 stages and mandatory review. No household compatibility is claimed.

## Evidence ledger

| Requirement | Actual evidence | Remaining gate |
|---|---|---|
| Compile unmodified Rust candidate | Rust 1.99.1, `acf45bd`: dependency resolution fails (`ifaddrs` native link conflict) | Candidate cannot be used unchanged |
| Compile patched fictional bridge | macOS arm64 release build passes; separate experiment lockfile; minimal dependency alias and TLV visibility patch | Does not prove HAP behavior |
| Generic Linux x86_64 | Local cross-build attempted, fails E0463: Linux standard library not installed in Homebrew toolchain; Podman socket unavailable. [CI run 39118484187](https://github.com/magic5644/homebridge-rust/actions/runs/39118484187) passes the explicit generic `x86-64` release build for `8db503b` | Not a NAS/J1900 run; latest-head CI is recorded in the PR |
| Upstream automated tests | Nine library tests pass after dependency alias in scratch checkout, 51 upstream warnings; includes one SRP proof unit case | No complete independent encrypted protocol suite |
| Fictional reads/writes | In-process switch and covering values tested through candidate characteristic APIs | No HTTP/controller read/write proof |
| Malformed data | Offline tests reproduce panic on truncated TLV; advertised covering bound accepts 101. Explicit qualification run fails both tests | Candidate fails these qualification gates; normal green test run only characterizes the defects |
| Persistence/restart | Private temporary FileStorage save/drop/reopen preserves identity/public key/PIN and controller record; deletion removes controller record; corrupt/missing identity rejected by experiment loader | No process restart with a paired controller; no crash-safety or revocation proof |
| Discovery/pairing/session verification | Owner-run fixture bridge prepared, not launched | No mDNS discovery, Pair Setup/Verify, encrypted-session or wrong-PIN test result |
| Notifications | Experiment can toggle fictional switch every 5 s and mirror fictional covering target into current position | No controller subscription/notification/resubscription evidence |
| Accessory addition/removal | Fixed AIDs: bridge 1, switch 2, covering 3; owner can restart with/without covering | No topology/configuration-number interoperability result; upstream removal persistence gap remains |
| Controller removal | Storage API record deletion only | No admin/non-admin protocol checks, last-admin or active-session invalidation evidence |
| Clean shutdown | Ctrl-C cancellation path prepared, not exercised with server | SIGTERM, socket cleanup, mDNS withdrawal, durable state and ≤10 s shutdown unproven |
| Apple Home | No authorized iPhone/iPad test session or owner results supplied | Pairing and every Apple step below remain **not run** |
| Human HAP/security review | Agent targeted source inspection and offline checks only | Named competent human review tied to exact commit is **not obtained** |

No server or multicast advertiser was started during this evaluation. No test
used TaHoma credentials or a physical command path. No Homebridge configuration,
production bridge, pairing, room, scene or automation was changed.

## Independent AI review follow-up

The owner-requested [review report](../reviews/P04-hap-security.md) records further
source-confirmed authorization, setup/revocation, encrypted transport and resource
lifetime defects, plus advisory-affected dependency versions. No runtime exploit
or fix is claimed. Keep the bridge stopped while these findings await competent
human assessment. The dormant procedure below does not authorize a trial.
This AI review does not fulfill the human-review row of the evidence ledger.

## Reproduce offline checks

Run from the repository root. On the owner's machine, prefix commands with
`rtk` (`rtk proxy` for commands it does not support); CI uses native commands.
Use a fresh experiment checkout for preparation: it refuses an existing vendor
folder rather than resetting local changes.

```sh
rtk proxy sh experiments/hap-proof/prepare.sh
rtk proxy cargo fmt --manifest-path experiments/hap-proof/Cargo.toml --package hap-proof -- --check
rtk proxy cargo clippy --manifest-path experiments/hap-proof/Cargo.toml --all-targets --no-deps --locked -- -D warnings
rtk proxy cargo test --manifest-path experiments/hap-proof/Cargo.toml --locked
rtk proxy cargo build --manifest-path experiments/hap-proof/Cargo.toml --release --locked
rtk proxy sh scripts/check.sh
rtk proxy python3 -B -m unittest discover -s tests -v
```

The experiment is not a workspace member and is not a delivered backend.
Its `vendor/hap` source is pinned by `prepare.sh`; `candidate.patch` is completely
reviewable. The experiment lockfile pins transitive versions. Clippy checks the
probe with `--no-deps`; upstream warnings remain disclosed, not repaired or
certified by this check.

Run the deliberately failing qualification gates separately:

```sh
rtk proxy cargo test --manifest-path experiments/hap-proof/Cargo.toml --locked qualification_ -- --ignored
```

Expected current result: exit 101, **two failed tests**. They are explicitly
ignored in the ordinary characterization suite so CI can preserve a reproducible
rejection case. A green CI run does not mean these gates pass. Before any future
approval they must pass normally, alongside the full protocol suite.

The nine upstream tests were run with `cargo test --manifest-path
/tmp/homebridge-p04-hap-rs/Cargo.toml --lib` on the pinned scratch clone after
only the interface dependency alias. To reproduce, use a new scratch clone at
`acf45bd`, apply the experiment patch, and run the library tests. Upstream has
no committed Cargo.lock; this historical run is not a frozen transitive-dependency
proof. The checked-in probe lockfile is the reproducible evaluation dependency set.

Linux CI runs the same offline probe plus:

```sh
RUSTFLAGS='-C target-cpu=x86-64' cargo build --manifest-path experiments/hap-proof/Cargo.toml --release --target x86_64-unknown-linux-gnu --locked
```

There is no LAN/hardware execution or artifact publication in CI.

## Measurements and limitations

The initial macOS arm64 release executable was 3,548,448 bytes. An offline test
harness sample (six checks, two ignored gates) used 4,734,976 bytes maximum RSS;
CPU user/system and wall time each rounded to 0.00 s at the tool resolution. These are developer-host
artifact/test measurements, not bridge idle/load measurements. No steady-state
bridge RSS/PSS, CPU, event latency, pairing latency or J1900 measurement exists.
No resource gain against Homebridge is claimed. Full comparable measurements
remain at P31/P38; prototype load measurements remain an open P04 item.

After a reviewer clears an isolated runtime trial, measure the exact binary,
OS/architecture and commit with `/usr/bin/time -l` on macOS or `/usr/bin/time -v`
on Linux; sample idle then controller read/write/event bursts and a slow reader.
Record warmup/window duration, RSS peak, CPU seconds, task/fd counts and queue
limits. Use the same host/scope for comparisons. Never publish raw logs or state.

## Owner-assisted Apple Home procedure — not yet executed

This is a prepared procedure, not a request to change production. Before starting,
a competent reviewer must assess the known rejection cases and approve a
**disposable isolated lab trial** or require fixes first. A lab trial cannot
approve the backend while the qualification tests fail. The owner supplies an
isolated test LAN/interface and iPhone/iPad; current production pairing access
is neither required nor requested. Do not reuse the existing Homebridge identity
or storage, do not install on the production NAS, and do not attach real devices.

1. Use a disposable host/profile connected only to the test LAN and a separate
   Apple Home named for P04 testing. Ensure only fictional accessories are in
   that Home; leave the production Home, hub, scenes and automations untouched.
   Confirm TCP port 52104 is free. mDNS may use interfaces beyond the configured
   TCP bind address: network isolation must be at the host/network level.
2. Run the offline checks above and record the tested commit. On the owner
   terminal, start the prepared bridge using the **test interface IP**:

   ```sh
   rtk proxy sh experiments/hap-proof/run-isolated.sh TEST_INTERFACE_IP
   ```

   Replace the placeholder locally; never put an actual address in the public
   report. The wrapper uses `umask 077`, disables logging and stores generated
   identity/PIN/pairings only in ignored `private/hap-proof/`. No PIN is embedded
   in this repository. Read the generated `config.json`'s PIN **locally** in a
   private editor; do not paste config, PIN, QR, keys or raw logs into chat/CI/PRs.
   Config corruption fails closed; do not delete state to make a pairing succeed.
3. On iPhone/iPad in the test Home, use Add → Add Accessory and the manual-code
   option if offered; select only `P04 Fictional Bridge` and enter its local PIN.
   Apple documents [accessory addition](https://support.apple.com/en-us/104998).
   Record OS/app versions and sanitized pass/fail/error categories. Pairing is
   **not** possible from Home on Mac according to that Apple support page.
4. Verify bridge discovery, successful pairing and stable accessory IDs. Read
   the fictional switch and covering. Write switch on/off and covering 0/50/100;
   observe cached reads. Covering current position mirrors the target every
   five seconds as an explicit simulation, not a physical measurement. Switch
   toggles every five seconds to exercise notifications; it may override a
   manual write. Use a second test controller/subscriber to distinguish events
   from polling. Record subscriptions, disconnect/reconnect and resubscription.
5. Stop only the test process with Ctrl-C; record exit latency and confirm the
   advertisement/socket disappears. Restart with the same private state and
   confirm reconnect without re-pairing and unchanged AID/IID/bridge identity.
   Separately test SIGTERM on the test PID with supervision; mark unsupported
   behavior as failed, never call it graceful shutdown without evidence.
6. Stop and restart with `--without-covering` appended to the launch command,
   then stop/restart without that option to add it back. Verify switch identity,
   bridge configuration-number/discovery changes and Apple accessory refresh.
   This exercises topology across restart; dynamic in-session removal/addition
   requires an additional protocol test and is not claimed by these steps.
7. Remove **only P04 Fictional Bridge** from the test Home. With an independent
   controller, verify admin removal, non-admin denial, last-admin semantics and
   denial of requests on an already-open revoked session. Storage file deletion
   is not a substitute. Re-pair only this disposable bridge if needed; do not
   reset or import any production pairing or Homebridge persist directory.
8. Run malformed/oversize requests, wrong PIN/proof/signature/tag and permission
   tests only against the disposable bridge via the independent test controller.
   Do not probe production. Record rejection and server survival; observed panics
   fail qualification. A complete independent controller harness is still missing.
9. Stop the test process. Keep state private for repeatability; remove only the
   fictional bridge from the test Home when finished. Any local state deletion
   is owner-controlled and affects this test identity only.

For each step, record: date, exact implementation commit/patch, host/OS and
controller versions, fictional AID/IID, expected/observed outcome, pass/fail/not
run, redacted evidence and reviewer. Never publish identifiers, addresses, PINs,
keys, screenshots of the real Home or packet captures containing private data.

## Mandatory human review record

Status: **not obtained**. Reviewer/date/tested commit: **not supplied**.
Required scope: applicable HAP revision/vector provenance; cryptographic library
selection/advisories and SRP transcript compatibility; pairing state transitions,
nonce/counter uniqueness, tag handling, retry limits; permissions and revocation;
private atomic persistence and log redaction; malformed inputs/resource bounds;
shutdown; actual Apple Home evidence. Record defects and resolutions before
changing the ADR. CI and an agent inspection cannot sign this record.
