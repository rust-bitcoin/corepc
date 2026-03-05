# Bitcoin Core JSON-RPC support

There are two primary purposes of this repository:

1. Provide the [`corepc-types`](https://crates.io/crates/corepc-types) crate for use in production
   software.

2. Provide tools for integration testing Rust code that interacts with the Bitcoin network.
   Primarily consumers of the [`rust-bitcoin`](https://crates.io/crates/bitcoin) library. And enable
   doing so against multiple versions of Bitcoin Core.

If you require a JSON RPC client in production software it is expected you write your own and only
use the `corepc-types` crate in your dependency graph. Feel free to copy/steal/plagiarise or
otherwise enjoy yourself with anything in this repository - no attribution required.

## Contributing

PRs, feature requests, and bug reports against `corepc-types` most welcome and appreciated.

PRs, feature requests, and bug reports against the other crates welcome if you are using them for
integration testing.

**Please do not use `corepc-client` in production and raise bugs, issues, or feature requests.**

## Crate/directory listing

- `bitreq/`: [`bitreq`](https://crates.io/crates/bitreq): Simple, minimal-dependency HTTP client, with optional features for proxies, async, and https.
- `client/`: [`corepc-client`](https://crates.io/crates/corepc-client): A blocking JSON-RPC client used to test `corepc-types`.
- `contrib/`: Contains scripts including one to run local regtest `bitcoind` nodes using versions specified in the config file. A template config file and bitcoind aliases are in the subdirectory `templates/`
- `integration_test/`: Integration tests that use `corepc-client` and `corepc-node` to test `corepc-types`.
- `jsonrpc/`: [`jsonrpc`](https://crates.io/crates/jsonrpc): Rudimentary support for sending JSONRPC 2.0 requests and receiving responses.
- `node/`: [`corepc-node`](https://crates.io/crates/corepc-node): Runs `bitcoind` regtest nodes.
- `types/`: [`corepc-types`](https://crates.io/crates/corepc-types): Rust types returned by the JSON-RPC API of Bitcoin Core.
- `verify/`: Provides a tool to help verify the rest of the repository.

## Design

This repository is a bit oddly designed. It was done so very intentionally. The aim is to provide
data types for every single JSON RPC method provided by Core for many versions going back to v17.
Furthermore we want to explicitly test every method call using every data structure to ensure we got
it correct.

Fundamentally when you make an arbitrary RPC call against a miscellaneous Bitcoin Core instance you
do not know which version of Core you are talking to. No crate in this repository makes any effort
to solve this problem - it is left to the application. If however you want to run a specific RPC
call against a specific version of Core this repository is your friend.

## Original code

I don't know who is using `bitcoind` and/or `rust-bitcoincore-rpc` in the wild and I do not want to
disrupt them. As such `bitcoind` was pulled in here with permission of the original author.

Some code shamelessly stolen from `rust-bitcoincore-rpc` (credit to Steven).

- [rust-bitcoincore-rpcv0.19.0](https://github.com/rust-bitcoin/rust-bitcoincore-rpc)
- [`bitcoind`](https://crates.io/crates/bitcoind)

## Minimum Supported Rust Version (MSRV)

This library should always compile with any combination of features on **Rust 1.75.0**.

Use `Cargo-minimal.lock` to build the MSRV by copying to `Cargo.lock` and building.
