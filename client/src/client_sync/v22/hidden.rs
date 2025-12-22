// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is `== Hidden ==` methods that are not listed in the
//! API docs of Bitcoin Core `v22`.
//!
//! All macros require `Client` to be in scope.
//!
//! See, or use the `define_jsonrpc_bitreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `addconnection`.
#[macro_export]
macro_rules! impl_client_v22__add_connection {
    () => {
        impl Client {
            pub fn add_connection(
                &self,
                address: &str,
                connection_type: &str,
            ) -> Result<AddConnection> {
                self.call("addconnection", &[into_json(address)?, into_json(connection_type)?])
            }
        }
    };
}
