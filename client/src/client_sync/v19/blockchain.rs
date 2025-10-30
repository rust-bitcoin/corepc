// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Blockchain ==` section of the
//! API docs of Bitcoin Core `v0.19`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_bitreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `getblockfilter`.
#[macro_export]
macro_rules! impl_client_v19__get_block_filter {
    () => {
        impl Client {
            pub fn get_block_filter(&self, block: BlockHash) -> Result<GetBlockFilter> {
                self.call("getblockfilter", &[into_json(block)?])
            }
        }
    };
}
