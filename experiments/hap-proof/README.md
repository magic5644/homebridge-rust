# P04 Rust HAP evaluation fixture

Decision: **stop-and-review**. This throwaway probe is not a product backend.
Read [ADR 0001](../../docs/decisions/0001-homekit-backend.md) and the
[proof ledger / owner pairing procedure](../../docs/hardware/hap-proof.md).

The separate Unix-only Cargo workspace uses Rust 1.99.1 and pins upstream
`ewilken/hap-rs` at `acf45bdb00e23c81b67b3d5116c113335dc83f16`.
`prepare.sh` clones into ignored `vendor/hap` and applies `candidate.patch`:

- Alias the duplicate interface-discovery dependency to unblock compilation.
- Expose the existing TLV module for direct offline negative tests.

No custom crypto, protocol fix, auth bypass or certificate-validation change is
made. No product crate or product lockfile depends on this fixture.

From the repository root, on the owner's machine:

```sh
rtk proxy sh experiments/hap-proof/prepare.sh
rtk proxy cargo test --manifest-path experiments/hap-proof/Cargo.toml --locked
rtk proxy cargo test --manifest-path experiments/hap-proof/Cargo.toml --locked qualification_ -- --ignored
```

The second test invocation deliberately fails two qualification gates; it is not
a successful conformance suite. The normal suite reproduces and labels the known
defects, checks fictional characteristic reads/writes and storage reload/removal.
No test starts a server, advertises mDNS or connects to an accessory/controller.

The binary refuses to serve without `--serve-isolated-lan` and an explicit IP.
Do not launch it before the review/isolation steps in the proof ledger. The
owner-only wrapper sets private permissions and disables logging. It uses a
fresh random bridge identity and PIN persisted under ignored `private/hap-proof`,
TCP 52104, bridge AID 1, fictional switch AID 2 and covering AID 3. It contains no
manufacturer client or physical command route. `--without-covering` supports a
restart-based topology trial. Drop-on-Ctrl-C is not validated graceful shutdown.

CI runs only offline checks and a generic Linux x86_64 build. Apple Home,
notifications, secure sessions and human security review remain separate gates.
