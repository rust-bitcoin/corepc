// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is `== Hidden ==` methods that are not listed in the
//! API docs of Bitcoin Core `v29`.
//!
//! All macros require `Client` to be in scope.
//!
//! See, or use the `define_jsonrpc_bitreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `getorphantxs` verbosity level 0.
#[macro_export]
macro_rules! impl_client_v29__get_orphan_txs {
    () => {
        impl Client {
            pub fn get_orphan_txs(&self) -> Result<GetOrphanTxs> { self.call("getorphantxs", &[]) }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getorphantxs` verbosity level 1.
#[macro_export]
macro_rules! impl_client_v29__get_orphan_txs_verbosity_1 {
    () => {
        impl Client {
            pub fn get_orphan_txs_verbosity_1(&self) -> Result<GetOrphanTxsVerboseOne> {
                self.call("getorphantxs", &[into_json(1)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getorphantxs` verbosity level 2.
#[macro_export]
macro_rules! impl_client_v29__get_orphan_txs_verbosity_2 {
    () => {
        impl Client {
            pub fn get_orphan_txs_verbosity_2(&self) -> Result<GetOrphanTxsVerboseTwo> {
                self.call("getorphantxs", &[into_json(2)?])
            }
        }
    };
}
