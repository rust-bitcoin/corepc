// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Requires `Client` to be in scope.
//!
//! Specifically this is methods found under the `== Network ==` section of the
//! API docs of Bitcoin Core `v26`.
//!
//! See, or use the `define_jsonrpc_bitreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `getaddrmaninfo`.
#[macro_export]
macro_rules! impl_client_v26__get_addr_man_info {
    () => {
        impl Client {
            pub fn get_addr_man_info(&self) -> Result<GetAddrManInfo> {
                self.call("getaddrmaninfo", &[])
            }
        }
    };
}
