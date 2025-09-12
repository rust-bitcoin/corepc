# Bitcoin Core JSON-RPC types

This crate provides data types returned by Bitcoin Core's JSON-RPC API. Each type is specific to the
version of Core e.g., if you run the `getblockchaininfo` method against a Bitcoin Core v28 instance
you will get back the data described by `types::v28::GetBlockChainInfo`. In a similar fashion any
method `corerpcmethod` will return type `CoreRpcMethod` - snake-case as is conventional in Rust.

The version specific structs _do not_ use types form `rust-bitcoin`. For any type that can be
represented using types from `rust-bitcoin` we provide a version non-specific type in 
`model::CoreRpcMethod` and an `into_model()` method on the version specific type.

The crate supports **all** documented Core RPC methods.

(Note there are a bunch of undocumented methods that are not yet supported, coming soon.)

## Known issues

The types include docs from Core however the docs used are from the _first_ Core version in which
the method appeared. As an example, this means if you look at docs for v29 `createwallet` you will
see docs that originate in Core v17 so may or may not be stale. We hope to fix this at some stage.
For accurate documentation of the method you are best to run `bitcoin-cli help createwallet` against
a Core node of the desired version.

### Testing and Verification

Each type is integration tested, however only typically with a single test. We hope to improve
test coverage by using test vectors taken from Core source code ... at some stage.

If you experience any issues please let us know, we have done our best but this crate needs battle
testing in the wild.

## Minimum Supported Rust Version (MSRV)

This library should always compile with any combination of features on **Rust 1.63.0**.

## Licensing

The code in this project is licensed under the [Creative Commons CC0 1.0 Universal license](LICENSE).
We use the [SPDX license list](https://spdx.org/licenses/) and [SPDX IDs](https://spdx.dev/ids/).
