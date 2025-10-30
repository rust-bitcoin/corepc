// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Blockchain ==` section of the
//! API docs of Bitcoin Core `v24`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_bitreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `gettxspendingprevout`
#[macro_export]
macro_rules! impl_client_v24__get_tx_spending_prevout {
    () => {
        impl Client {
            pub fn get_tx_spending_prevout(
                &self,
                outputs: &[bitcoin::OutPoint],
            ) -> Result<GetTxSpendingPrevout> {
                let json_outputs: Vec<_> = outputs.iter().map(|out| {
                    serde_json::json!({
                        "txid": out.txid.to_string(),
                        "vout": out.vout,
                    })
                }).collect();
                self.call("gettxspendingprevout", &[json_outputs.into()])
            }
        }
    };
}
