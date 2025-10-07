#!/usr/bin/env bash
#
# Update the minimal/recent lock file

set -euo pipefail

for file in Cargo-minimal.lock Cargo-recent.lock; do
    cp "$file" Cargo.lock

    # Workspace crates with all features
    workspace_crates=("client" "types" "jsonrpc")
    for crate in "${workspace_crates[@]}"; do
        (cd "$crate" && cargo check --all-features)
    done

    # Crates with version features (use latest)
    version_crates=("node" "integration_test")
    for crate in "${version_crates[@]}"; do
        (cd "$crate" && cargo check --features=latest)
    done

    # Other crates
    other_crates=("verify")
    for crate in "${other_crates[@]}"; do
        (cd "$crate" && cargo check)
    done

    cp --force Cargo.lock "$file"
done
