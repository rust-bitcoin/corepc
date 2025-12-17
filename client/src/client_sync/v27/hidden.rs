// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is `== Hidden ==` methods that are not listed in the
//! API docs of Bitcoin Core `v29`.
//!
//! All macros require `Client` to be in scope.
//!
//! See, or use the `define_jsonrpc_bitreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `addconnection`.
#[macro_export]
macro_rules! impl_client_v27__add_connection {
    () => {
        impl Client {
            pub fn add_connection(
                &self,
                address: &str,
                connection_type: &str,
                v2transport: bool,
            ) -> Result<AddConnection> {
                self.call(
                    "addconnection",
                    &[into_json(address)?, into_json(connection_type)?, into_json(v2transport)?],
                )
            }
        }
    };
}
