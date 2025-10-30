// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Util ==` section of the
//! API docs of Bitcoin Core `v0.18`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_bitreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `deriveaddresses`.
#[macro_export]
macro_rules! impl_client_v18__derive_addresses {
    () => {
        impl Client {
            pub fn derive_addresses(&self, descriptor: &str) -> Result<DeriveAddresses> {
                self.call("deriveaddresses", &[descriptor.into()])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getdescriptorinfo`.
#[macro_export]
macro_rules! impl_client_v18__get_descriptor_info {
    () => {
        impl Client {
            pub fn get_descriptor_info(&self, descriptor: &str) -> Result<GetDescriptorInfo> {
                self.call("getdescriptorinfo", &[descriptor.into()])
            }
        }
    };
}
