// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Util ==` section of the
//! API docs of Bitcoin Core `v0.21`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_bitreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `getindexinfo`.
#[macro_export]
macro_rules! impl_client_v21__get_index_info {
    () => {
        impl Client {
            pub fn get_index_info(&self) -> Result<GetIndexInfo> { self.call("getindexinfo", &[]) }
        }
    };
}
