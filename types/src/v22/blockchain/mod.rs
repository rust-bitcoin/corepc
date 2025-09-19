// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v22` - blockchain.
//!
//! Types for methods found under the `== Blockchain ==` section of the API docs.

mod into;

use serde::{Deserialize, Serialize};

pub use super::GetMempoolInfoError;

/// Result of JSON-RPC method `getmempoolinfo` with verbose set to `true`.
///
/// > getmempoolinfo
/// >
/// > Returns details on the active state of the TX memory pool.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolInfo {
    /// True if the mempool is fully loaded. v0.19 and later only.
    pub loaded: bool,
    /// Current transaction count.
    pub size: i64,
    /// Sum of all virtual transaction sizes as defined in BIP 141.
    ///
    /// Differs from actual serialized size because witness data is discounted.
    pub bytes: i64,
    /// Total memory usage for the mempool.
    pub usage: i64,
    /// Total fees for the mempool in BTC, ignoring modified fees through prioritisetransaction. v23
    /// and later only.
    pub total_fee: f64,
    /// Maximum memory usage for the mempool.
    #[serde(rename = "maxmempool")]
    pub max_mempool: i64,
    /// Minimum fee rate in BTC/kB for a transaction to be accepted.
    ///
    /// This is the maximum of `minrelaytxfee` and the minimum mempool fee.
    #[serde(rename = "mempoolminfee")]
    pub mempool_min_fee: f64,
    /// Current minimum relay fee for transactions.
    #[serde(rename = "minrelaytxfee")]
    pub min_relay_tx_fee: f64,
    /// Current number of transactions that haven't passed initial broadcast yet. v21 and later only.
    #[serde(rename = "unbroadcastcount")]
    pub unbroadcast_count: i64,
}
