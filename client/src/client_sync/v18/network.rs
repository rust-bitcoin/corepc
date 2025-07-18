// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Requires `Client` to be in scope.
//!
//! Specifically this is methods found under the `== Network ==` section of the
//! API docs of Bitcoin Core `v0.18`.
//!
//! See, or use the `define_jsonrpc_minreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `getnodeaddresses`.
#[macro_export]
macro_rules! impl_client_v18__get_node_addresses {
    () => {
        impl Client {
            pub fn get_node_addresses(&self) -> Result<GetNodeAddresses> {
                self.call("getnodeaddresses", &[])
            }
        }
    };
}
