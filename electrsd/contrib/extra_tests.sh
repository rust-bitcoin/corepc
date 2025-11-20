#!/usr/bin/env bash
#
# Additional tests for `electrs`.
#
# This script was written based on CI in
# `github.com/RCasatta/electrsd` when the crate was imported to this repo.
# (HEAD on that repo was: dc88d81520c9ec507fc830b88a3b92a034dbaf93).

set -euox pipefail

# Use the current `Cargo.lock` file without updating it.
cargo="cargo --locked"


main() {
    corepc_node_versions
    electrs_versions
    electrs_esplora
}

corepc_node_versions() {
    local corepc_node_versions=("corepc-node_29_0" "corepc-node_28_2" "corepc-node_27_2" "corepc-node_26_2" "corepc-node_25_2")

    for feature in "${corepc_node_versions}"; do
        $cargo test --features="$feature,electrs_0_10_6"
    done
}

electrs_versions() {
    local electrs_versions=("electrs_0_10_6" "electrs_0_9_11" "electrs_0_9_1" "electrs_0_8_10")

    for feature in "${electrs_versions}"; do
        $cargo test --features="corepc-node_29_0,$feature"
    done
}

electrs_esplora() {
    $cargo test --features="corepc-node_22_1,legacy,esplora_a33e97e1"
}

#
# Main script
#
main "$@"
exit 0
