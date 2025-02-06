#!/usr/bin/env bash
#
# Script used to run linter.
#
# This is the same as `run_task.sh lint` except doesn't lint `node` with no
# features because `node` requires a version feature to be enabled.

set -euox pipefail

REPO_DIR=$(git rev-parse --show-toplevel)

# Use the current `Cargo.lock` file without updating it.
cargo="cargo --locked"

main() {
    check_required_commands
    need_nightly
    lint
}

# Lint various feature combinations to try and catch mistakes in feature gating.
lint() {
    lint_members_excluding_node
    lint_node
}

lint_members_excluding_node() {
    # Same crate list as in members section of workspace manifest excluding `node`.
    for crate in "client" "types" "jsonrpc"; do
        pushd "$REPO_DIR/$crate" > /dev/null
        $cargo clippy --all-targets --keep-going -- -D warnings
        $cargo clippy --all-targets --all-features --keep-going -- -D warnings
        $cargo clippy --all-targets --no-default-features --keep-going -- -D warnings
        popd > /dev/null
    done
}

lint_node() {
    local features=("28_0" "27_2" "27_1" "26_2" "26_0" "25_2" "24_2" "23_2" "22_1" "0_21_2" "0_20_2" "0_19_1" "0_18_1" "0_17_1")

    # lint for each version feature.
    pushd "$REPO_DIR/node" > /dev/null
    for feature in "${features[@]}"; do
        $cargo clippy --features="$feature" --all-targets --keep-going -- -D warnings
    done

    # And with all features to catch possible mistakes with `download` feature.
    $cargo clippy --all-targets --all-features --keep-going -- -D warnings

    popd > /dev/null
}

# Check all the commands we use are present in the current environment.
check_required_commands() {
    need_cmd cargo
}

err() {
    echo "$1" >&2
    exit 1
}

need_cmd() {
    if ! command -v "$1" > /dev/null 2>&1
    then err "need '$1' (command not found)"
    fi
}

need_nightly() {
    cargo_ver=$(cargo --version)
    if echo "$cargo_ver" | grep -q -v nightly; then
        err "Need a nightly compiler; have $(cargo --version)"
    fi
}

#
# Main script
#
main "$@"
exit 0
