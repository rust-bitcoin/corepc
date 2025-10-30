// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Generating ==` section of the
//! API docs of Bitcoin Core `v0.21`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_bitreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `generateblock`.
#[macro_export]
macro_rules! impl_client_v21__generate_block {
    () => {
        impl Client {
            pub fn generate_block(
                &self,
                output: &str,
                transactions: &[String],
            ) -> Result<GenerateBlock> {
                self.call("generateblock", &[into_json(output)?, into_json(transactions)?])
            }
        }
    };
}
