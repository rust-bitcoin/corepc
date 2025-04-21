// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Blockchain ==` section of the
//! API docs of Bitcoin Core `v0.23`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_minreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `savemempool`
#[macro_export]
macro_rules! impl_client_v23__savemempool {
    () => {
        impl Client {
            pub fn save_mempool(&self) -> Result<SaveMempool> { self.call("savemempool", &[]) }
        }
    };
}
