// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Signer ==` section of the
//! API docs of Bitcoin Core `v22`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_minreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `enumeratesigners`
#[macro_export]
macro_rules! impl_client_v22__enumerate_signers {
    () => {
        impl Client {
            pub fn enumerate_signers(&self) -> Result<EnumerateSigners> {
                self.call("enumeratesigners", &[])
            }
        }
    };
}
