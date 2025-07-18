// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Mining ==` section of the
//! API docs of Bitcoin Core `v26`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_minreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `getprioritisedtransactions`.
#[macro_export]
macro_rules! impl_client_v26__get_prioritised_transactions {
    () => {
        impl Client {
            pub fn get_prioritised_transactions(&self) -> Result<GetPrioritisedTransactions> {
                self.call("getprioritisedtransactions", &[])
            }
        }
    };
}
