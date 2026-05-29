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
