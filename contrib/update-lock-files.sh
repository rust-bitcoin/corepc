#!/usr/bin/env bash
#
# Update the minimal/recent lock file

set -euo pipefail

REPO_DIR="$(git rev-parse --show-toplevel)"

# Targets where `--all-features` is used.
ALL_FEATURE_CRATES=(bitreq client fuzz jsonrpc types verify)

# Targets with conflicting features and only speficic features are used.
SPECIFIC_FEATURES_CRATES=(integration_test node)
SPECIFIC_FEATURES=(latest)

update_lock_files() {
    for crate in "${ALL_FEATURE_CRATES[@]}"; do
        cargo check --manifest-path "$REPO_DIR/$crate/Cargo.toml" --all-features
    done

    for crate in "${SPECIFIC_FEATURES_CRATES[@]}"; do
        cargo check --manifest-path "$REPO_DIR/$crate/Cargo.toml" --no-default-features --features="${SPECIFIC_FEATURES[*]}"
    done
}

for file in Cargo-minimal.lock Cargo-recent.lock; do
    cp --force "$file" Cargo.lock
    update_lock_files
    cp --force Cargo.lock "$file"
done
