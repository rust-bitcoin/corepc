// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Blockchain ==` section of the
//! API docs of Bitcoin Core `v31`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_bitreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `getmempoolcluster`.
#[macro_export]
macro_rules! impl_client_v31__get_mempool_cluster {
    () => {
        impl Client {
            pub fn get_mempool_cluster(&self, txid: Txid) -> Result<GetMempoolCluster> {
                self.call("getmempoolcluster", &[into_json(txid)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getmempoolfeeratediagram`.
#[macro_export]
macro_rules! impl_client_v31__get_mempool_feerate_diagram {
    () => {
        impl Client {
            pub fn get_mempool_feerate_diagram(&self) -> Result<GetMempoolFeerateDiagram> {
                self.call("getmempoolfeeratediagram", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `gettxspendingprevout`.
#[macro_export]
macro_rules! impl_client_v31__get_tx_spending_prevout {
    () => {
        impl Client {
            pub fn get_tx_spending_prevout(
                &self,
                outputs: &[bitcoin::OutPoint],
                mempool_only: bool,
                return_spending_tx: bool,
            ) -> Result<GetTxSpendingPrevout> {
                let json_outputs: Vec<_> = outputs.iter().map(|out| {
                    serde_json::json!({
                        "txid": out.txid.to_string(),
                        "vout": out.vout,
                    })
                }).collect();
                let options = serde_json::json!({
                    "mempool_only": mempool_only,
                    "return_spending_tx": return_spending_tx,
                });
                self.call("gettxspendingprevout", &[json_outputs.into(), options])
            }
        }
    };
}
