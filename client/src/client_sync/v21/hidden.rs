// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is `== Hidden ==` methods that are not listed in the
//! API docs of Bitcoin Core `v0.21`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_minreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `addpeeraddress`.
#[macro_export]
macro_rules! impl_client_v21__add_peer_address {
    () => {
        impl Client {
            pub fn add_peer_address(&self, address: &str, port: u16) -> Result<AddPeerAddress> {
                self.call("addpeeraddress", &[address.into(), port.into()])
            }
        }
    };
}
