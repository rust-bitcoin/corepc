// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Generating ==` section of the
//! API docs of Bitcoin Core `v0.20`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_minreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `generatetodescriptor`.
#[macro_export]
macro_rules! impl_client_v20__generate_to_descriptor {
    () => {
        impl Client {
            pub fn generate_to_descriptor(
                &self,
                nblocks: usize,
                descriptor: &str,
            ) -> Result<GenerateToDescriptor> {
                self.call("generatetodescriptor", &[nblocks.into(), descriptor.into()])
            }
        }
    };
}
