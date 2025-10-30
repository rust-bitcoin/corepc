// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Rawtransactions ==` section of the
//! API docs of Bitcoin Core `v26`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_bitreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `submitpackage`.
#[macro_export]
macro_rules! impl_client_v26__submit_package {
    () => {
        impl Client {
            pub fn submit_package(
                &self,
                package: &[bitcoin::Transaction],
            ) -> Result<SubmitPackage> {
                let package_txs = package
                    .into_iter()
                    .map(|tx| bitcoin::consensus::encode::serialize_hex(tx))
                    .collect::<Vec<_>>();
                self.call("submitpackage", &[package_txs.into()])
            }
        }
    };
}
