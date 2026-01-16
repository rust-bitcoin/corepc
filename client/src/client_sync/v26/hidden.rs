// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is `== Hidden ==` methods that are not listed in the
//! API docs of Bitcoin Core `v26`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_bitreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `getrawaddrman`.
#[macro_export]
macro_rules! impl_client_v26__get_raw_addrman {
    () => {
        impl Client {
            pub fn get_raw_addrman(&self) -> Result<GetRawAddrMan> {
                self.call("getrawaddrman", &[])
            }
        }
    };
}
