#!/bin/sh
# Owner-run only, after the review and isolation checks in the hardware guide.
set -eu
umask 077
unset RUST_LOG
cd "$(dirname "$0")/../.."
exec cargo run --manifest-path experiments/hap-proof/Cargo.toml --release --locked -- --serve-isolated-lan "$@"
