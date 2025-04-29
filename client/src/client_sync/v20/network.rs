// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Requires `Client` to be in scope.
//!
//! Specifically this is methods found under the `== Network ==` section of the
//! API docs of Bitcoin Core `v0.20`.
//!
//! See, or use the `define_jsonrpc_minreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `setnetworkactive`
#[macro_export]
macro_rules! impl_client_v20__setnetworkactive {
    () => {
        impl Client {
            pub fn set_network_active(&self, state: bool) -> Result<SetNetworkActive> {
                self.call("setnetworkactive", &[state.into()])
            }
        }
    };
}
