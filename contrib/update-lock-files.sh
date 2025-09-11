#!/usr/bin/env bash
#
# Update the minimal/recent lock file

set -euo pipefail

for file in Cargo-minimal.lock Cargo-recent.lock; do
    cp --force "$file" Cargo.lock
    # Build each crate individually to avoid conflicting features
    (cd client && cargo check --all-features)
    (cd types && cargo check --all-features)
    (cd jsonrpc && cargo check --all-features)
    (cd node && cargo check --features=29_0)
    # verify crate is excluded from workspace, handle separately
    (cd verify && cargo check)
    # Skip integration_test as it cannot use --all-features
    cp --force Cargo.lock "$file"
done
