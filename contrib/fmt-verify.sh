#!/usr/bin/env bash
#
# The `verify` crate is not part of the workspace and cannot be formatted
# with workspace-level `cargo fmt`.

set -euox pipefail

REPO_DIR=$(git rev-parse --show-toplevel)

cargo +"$(cat ./nightly-version)" fmt \
      --manifest-path "$REPO_DIR/verify/Cargo.toml" \
      --all -- --check
