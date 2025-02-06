#!/usr/bin/env bash
#
# Update the minimal/recent lock file

set -euo pipefail

REPO_DIR=$(git rev-parse --show-toplevel)

for file in Cargo-minimal.lock Cargo-recent.lock; do
    cp --force "$file" Cargo.lock

    # Same crate list as in members section of workspace manifest.
    for crate in "client" "types" "jsonrpc" "node"; do
        pushd "$REPO_DIR/$crate" > /dev/null
        cargo build --all-features
        popd > /dev/null
    done

    cp --force Cargo.lock "$file"
done
