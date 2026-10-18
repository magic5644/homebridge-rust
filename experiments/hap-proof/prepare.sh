#!/bin/sh
# Fetch the pinned evaluation source; never modify a product dependency.
set -eu
cd "$(dirname "$0")"
if [ -e vendor/hap ]; then
    echo "vendor/hap already exists; use a fresh experiment checkout" >&2
    exit 1
fi
mkdir -p vendor
git clone --quiet https://github.com/ewilken/hap-rs.git vendor/hap
git -C vendor/hap checkout --quiet acf45bdb00e23c81b67b3d5116c113335dc83f16
git -C vendor/hap apply ../../candidate.patch
