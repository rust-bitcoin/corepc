#!/usr/bin/env bash
#
# Run Bitcoin Core version specific tests (the `integration_test` and `node` crates.)
#
# The `node` crate features are different from a normal crate because:
#
# - `node` cannot be built with --no-default-features
# - `node` expects at least one version feature e.g., --features=28_0
# - `node` supports downloading the Bitcoin Core binary and also running `bitcoind` from the host.
#
# In CI we always want to download the Bitcoin Core binary. This means we always enable `download`.
# Also, we always enable exactly one feature (even though multiple features will just cause the
# higher one to override the lower one).
#
# These comments apply to the integration test crate also because it depends on `node`.

set -euox pipefail

REPO_DIR=$(git rev-parse --show-toplevel)
NODE_DIR="$REPO_DIR/node"
INTEGRATION_TEST_DIR="$REPO_DIR/integration_test"

main() {
    local version_feature="${1}"

    do_node_tests "$version_feature"
    do_integration_tests "$version_feature"
}

do_node_tests() {
    local version_feature="${1}"

    pushd "$NODE_DIR" > /dev/null
    cargo test --features=download,"$version_feature"
    popd > /dev/null
}

do_integration_tests() {
    local version_feature="${1}"

    pushd "$INTEGRATION_TEST_DIR" > /dev/null
    cargo test --features="$version_feature"
    popd > /dev/null
}

#
# Main script
#
main "$@"
exit 0
