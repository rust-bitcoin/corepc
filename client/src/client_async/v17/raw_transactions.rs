// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing async JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Rawtransactions ==` section of the
//! API docs of Bitcoin Core `v0.17`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_bitreq_async_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `getrawtransaction`.
#[macro_export]
macro_rules! impl_async_client_v17__get_raw_transaction {
    () => {
        impl Client {
            pub async fn get_raw_transaction(
                &self,
                txid: bitcoin::Txid,
            ) -> Result<GetRawTransaction> {
                self.call("getrawtransaction", &[into_json(&txid)?, false.into()]).await
            }

            pub async fn get_raw_transaction_verbose(
                &self,
                txid: Txid,
            ) -> Result<GetRawTransactionVerbose> {
                self.call("getrawtransaction", &[into_json(&txid)?, true.into()]).await
            }
        }
    };
}
