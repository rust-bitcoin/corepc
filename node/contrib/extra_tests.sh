#!/usr/bin/env bash
#
# Run Bitcoin Core version specific tests.
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

set -euox pipefail

FEATURES=("28_0" "27_1" "27_0" "26_2" "26_1" "26_0" "25_2" "24_2"  \
          "23_2" "22_1" "0_21_2" "0_20_2" "0_19_1" "0_18_1" "0_17_2")

# Use the current `Cargo.lock` file without updating it.
cargo="cargo --locked"


main() {
    $cargo check --all-features
    $cargo check --doc

    # Build every version.
    for feature in "${FEATURES[@]}"; do
        $cargo build --features="$feature"
    done

    # But only run tests for the latest version. This is ok because we are mainly checking
    # MSRV and docs with this script. Integration test will check every Core version.
    $cargo test --features=download,29_0
}

#
# Main script
#
main "$@"
exit 0
