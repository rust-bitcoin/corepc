#!/usr/bin/env bash
#
# Update the minimal/recent lock file

set -euo pipefail

for file in Cargo-minimal.lock Cargo-recent.lock; do
    cp --force "$file" Cargo.lock

    cd electrsd
    cargo check --features="corepc-node_29_0,electrs_0_10_6"
    cd ..

    cargo check

    cp --force Cargo.lock "$file"
done
