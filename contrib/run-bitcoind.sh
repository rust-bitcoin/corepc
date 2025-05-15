#!/usr/bin/env bash
#
# Run local regtest `bitcoind` nodes using versions specified in the config file.
# Config file default: ~/.run-bitcoind.conf
# Config file optional: ./run-bitcoind.conf

set -euo pipefail

# RPC authentication username.
RPC_USER="user"
# RPC authentication password.
RPC_PASSWORD="password"

usage() {
    cat <<'EOF'
Usage:

    ./run-bitcoind.sh [COMMAND]

COMMAND

   - all                      Start all bitcoind versions defined in the config file.
   - start [VERSION_ALIAS]    Start bitcoind nodes for the specified version alias (default: v22).
   - stop                     Kill all bitcoind nodes and clean up test directories.

CONFIGURATION

- Priority

    1. RUN_BITCOIND_CONF environment variable
    2. ./run-bitcoind.conf (script directory)
    3. ~/.run-bitcoind.conf (home directory)

- Config format

    <VERSION_ALIAS> <VERSION_NUMBER> <VERSION_ID> <BITCOIND_PATH>

    VERSION_ALIAS       Passed as command line argument to $(start).
    VERISON_NUMBER      The Bitocin Core version number.
    VERSION_ID          Used as part of port numbers.
    BITCOIND_PATH       Path to $(bitcoind) binary.

- Examples

    v28 28.1 281 /opt/bitcoin-28.0/bin/bitcoind
    v24 24.2 242 /opt/bitcoin-24.2/bin/bitcoind
    v21 0.21.2 212 /opt/bitcoin-0.21.2/bin/bitcoind

EOF
}

main() {
    local cmd="${1:-usage}"
    local version="${2:-}"

    # FIXME: This is a hackish way to get the help flag.
    if [ "$cmd" = "usage" ] || [ "$cmd" = "-h" ] || [ "$cmd" = "--help" ] || [ "$cmd" = "help" ]; then
        usage
        exit 0
    fi

    case $cmd in
        all|start)
            # Config loading logic
            local config_file=${RUN_BITCOIND_CONF:-}

            if [ -z "$config_file" ]; then
                local script_dir

                script_dir=$(dirname "$0")
                local local_config="${script_dir}/run-bitcoind.conf"

                if [ -f "$local_config" ]; then
                    config_file="$local_config"
                else
                    config_file="$HOME/.run-bitcoind.conf"
                fi
            fi

            if [ ! -f "$config_file" ]; then
                err "Config file $config_file not found. Please create it."
            fi

            # Load config into parallel arrays
            VERSION_ALIASES=()
            VERSION_NUMBERS=()
            VERSION_IDS=()
            BITCOIND_PATHS=()

            while IFS= read -r line; do
                line=$(echo "$line" | sed -e 's/#.*//' -e 's/^[[:space:]]*//' -e 's/[[:space:]]*$//')
                [ -z "$line" ] && continue
                read -r alias version_number version_id path <<<"$line"
                VERSION_ALIASES+=("$alias")
                VERSION_NUMBERS+=("$version_number")
                VERSION_IDS+=("$version_id")
                BITCOIND_PATHS+=("$path")
            done < "$config_file"
            ;;
    esac

    case $cmd in
        all)
            for index in "${!VERSION_ALIASES[@]}"; do
                start "${VERSION_ALIASES[$index]}" \
                      "${VERSION_NUMBERS[$index]}" \
                      "${VERSION_IDS[$index]}" \
                      "${BITCOIND_PATHS[$index]}"
            done
            ;;

        start)
            if [ -z "$version" ]; then
                version="v22"  # Default version
            fi

            found=false
            for index in "${!VERSION_ALIASES[@]}"; do
                if [ "${VERSION_ALIASES[$index]}" = "$version" ]; then
                    start "$version" \
                          "${VERSION_NUMBERS[$index]}" \
                          "${VERSION_IDS[$index]}" \
                          "${BITCOIND_PATHS[$index]}"
                    found=true
                    break
                fi
            done

            if [ "$found" = false ]; then
                err "Version '$version' not found in config file."
            fi
            ;;

        stop)
            pkill bitcoind || true
            rm -rf /tmp/corepc-*/2/regtest/wallets > /dev/null 2>&1
            echo "Stopped all bitcoind instances and cleaned wallets."
            ;;
        *)
            usage
            say "Error: unknown command '$cmd'"
            ;;
    esac
}

start() {
    local version="$1"
    local version_number="$2"
    local version_id="$3"
    local bitcoind_path="$4"

    if [ ! -x "$bitcoind_path" ]; then
        err "bitcoind binary not found or not executable at '$bitcoind_path'"
    fi

    run_bitcoind "$version" "$version_number" "$version_id" "$bitcoind_path"
}

run_bitcoind() {
    local version="$1"              # e.g., v28
    local version_number="$2"       # e.g., 28.1.0
    local version_id="$3"           # e.g., 281
    local bitcoind="$4"

    local test_dir="/tmp/corepc-${version_number}"
    local rpc_port="${version_id}49"

    if ! "$bitcoind" -version | grep -q "$version_number"; then
        echo "Version mismatch: Expected $version_number, got $("$bitcoind" -version | head -n1)"
        exit 1
    fi

    rm -rf "${test_dir}"
    mkdir -p "${test_dir}/1" "${test_dir}/2"

    local block_filter_arg=""
    if [[ "$version_number" =~ ^0\.(19|2) ]]; then
        block_filter_arg="-blockfilterindex=1"
    fi

    local fallback_fee_arg=""
    if [[ "$version_number" =~ ^[0-9]+\. ]]; then
        fallback_fee_arg="-fallbackfee=0.00001000"
    fi

    echo "Starting bitcoind v${version_number} (alias: ${version})..."
    "$bitcoind" -regtest $fallback_fee_arg $block_filter_arg \
                -datadir="${test_dir}/1" \
                -bind="127.0.0.1:${version_id}48" \
                -server=0 \
                -printtoconsole=0 &

    sleep 1  # Allow first node to start

    "$bitcoind" -regtest $fallback_fee_arg $block_filter_arg \
                -datadir="${test_dir}/2" \
                -connect=127.0.0.1:"${version_id}48" \
                -rpcport="$rpc_port" \
                -rpcuser="$RPC_USER" \
                -rpcpassword="$RPC_PASSWORD" \
                -server=1 \
                -txindex=1 \
                -printtoconsole=0 \
                -zmqpubrawblock=tcp://0.0.0.0:"${version_id}32" \
                -zmqpubrawtx=tcp://0.0.0.0:"${version_id}33" &

    sleep 1  # Let nodes connect
    echo "Two connected bitcoind v${version_number} instances running, one node has JSON-RPC listening on port ${rpc_port}"
}

say() {
    echo "run-bitcoind: $1"
}

err() {
    echo "$1" >&2
    exit 1
}

#
# Main script
#
main "$@"
exit 0
