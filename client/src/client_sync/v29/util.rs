// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Util ==` section of the
//! API docs of Bitcoin Core `v29`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_bitreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `deriveaddresses`.
#[macro_export]
macro_rules! impl_client_v29__derive_addresses {
    () => {
        impl Client {
            // For single derivation descriptors.
            pub fn derive_addresses(&self, descriptor: &str) -> Result<DeriveAddresses> {
                self.call("deriveaddresses", &[descriptor.into()])
            }

            // For multipath descriptors.
            pub fn derive_addresses_multipath(
                &self,
                descriptor: &str,
                range: (u32, u32),
            ) -> Result<DeriveAddressesMultipath> {
                let range = json!([range.0, range.1]);
                self.call("deriveaddresses", &[descriptor.into(), range.into()])
            }
        }
    };
}
