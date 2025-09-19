// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v24` - blockchain.
//!
//! Types for methods found under the `== Blockchain ==` section of the API docs.

mod error;
mod into;

use alloc::collections::BTreeMap;

use serde::{Deserialize, Serialize};

pub use self::error::GetTxSpendingPrevoutError;
pub use super::{GetMempoolInfoError, MapMempoolEntryError, MempoolEntryError, MempoolEntryFees};

/// Result of JSON-RPC method `getmempoolancestors` with verbose set to `false`.
///
/// > getmempoolancestors txid (verbose)
/// >
/// > If txid is in the mempool, returns all in-mempool ancestors.
/// >
/// > Arguments:
/// > 1. "txid"                 (string, required) The transaction id (must be in mempool)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolAncestors(pub Vec<String>);

/// Result of JSON-RPC method `getmempoolancestors` with verbose set to true.
///
/// Map of txid to `MempoolEntry` i.e., an ancestor.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolAncestorsVerbose(pub BTreeMap<String, MempoolEntry>);

/// Result of JSON-RPC method `getmempooldescendants` with verbose set to `false`.
///
/// > getmempooldescendants txid (verbose)
/// >
/// > If txid is in the mempool, returns all in-mempool descendants.
/// >
/// > Arguments:
/// > 1. "txid"                 (string, required) The transaction id (must be in mempool)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolDescendants(pub Vec<String>);

/// Result of JSON-RPC method `getmempooldescendants` with verbose set to true.
///
/// Map of txid to [`MempoolEntry`] i.e., a descendant.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolDescendantsVerbose(pub BTreeMap<String, MempoolEntry>);

/// Result of JSON-RPC method `getmempoolentry`.
///
/// > getmempoolentry txid
/// >
/// > Returns mempool data for given transaction
/// >
/// > Arguments:
/// > 1. "txid"                 (string, required) The transaction id (must be in mempool)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolEntry(pub MempoolEntry);

/// Mempool data. Part of `getmempoolentry`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct MempoolEntry {
    /// Virtual transaction size as defined in BIP 141.
    ///
    /// This is different from actual serialized size for witness transactions as witness data is
    /// discounted. v0.19 and later only.
    pub vsize: i64,
    /// Transaction weight as defined in BIP 141.
    pub weight: i64,
    /// Local time transaction entered pool in seconds since 1 Jan 1970 GMT.
    pub time: i64,
    /// Block height when transaction entered pool.
    pub height: i64,
    /// Number of in-mempool descendant transactions (including this one).
    #[serde(rename = "descendantcount")]
    pub descendant_count: i64,
    /// Virtual transaction size of in-mempool descendants (including this one).
    #[serde(rename = "descendantsize")]
    pub descendant_size: i64,
    /// Number of in-mempool ancestor transactions (including this one).
    #[serde(rename = "ancestorcount")]
    pub ancestor_count: i64,
    /// Virtual transaction size of in-mempool ancestors (including this one).
    #[serde(rename = "ancestorsize")]
    pub ancestor_size: i64,
    /// Hash of serialized transaction, including witness data.
    pub wtxid: String,
    /// Fee object which contains the base fee, modified fee (with fee deltas), and ancestor/descendant fee totals all in BTC.
    pub fees: MempoolEntryFees,
    /// Unconfirmed transactions used as inputs for this transaction (parent transaction id).
    pub depends: Vec<String>,
    /// Unconfirmed transactions spending outputs from this transaction (child transaction id).
    #[serde(rename = "spentby")]
    pub spent_by: Vec<String>,
    /// Whether this transaction signals BIP125 replaceability or has an unconfirmed ancestor
    /// signaling BIP125 replaceability.
    #[serde(rename = "bip125-replaceable")]
    pub bip125_replaceable: bool,
    /// Whether this transaction is currently unbroadcast (initial broadcast not yet acknowledged by
    /// any peers)
    pub unbroadcast: bool,
}

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
    /// Minimum fee rate increment for mempool limiting or replacement in BTC/kvB. v24 and later only.
    #[serde(rename = "incrementalrelayfee")]
    pub incremental_relay_fee: f64,
    /// Current number of transactions that haven't passed initial broadcast yet. v21 and later only.
    #[serde(rename = "unbroadcastcount")]
    pub unbroadcast_count: i64,
    /// True if the mempool accepts RBF without replaceability signaling inspection. v24 and later
    /// only.
    #[serde(rename = "fullrbf")]
    pub full_rbf: bool,
}

/// Result of JSON-RPC method `gettxspendingprevout`.
///
/// > gettxspendingprevout [{"txid":"hex","vout":n},...]
/// >
/// > Scans the mempool to find transactions spending any of the given outputs
/// >
/// > Arguments:
/// > 1. outputs                 (json array, required) The transaction outputs that we want to check, and within each, the txid (string) vout (numeric).
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxSpendingPrevout(pub Vec<GetTxSpendingPrevoutItem>);

/// A transaction item. Part of `gettxspendingprevout`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxSpendingPrevoutItem {
    /// The transaction id of the checked output
    pub txid: String,
    /// The vout value of the checked output
    pub vout: u32,
    /// The transaction id of the mempool transaction spending this output (omitted if unspent)
    #[serde(rename = "spendingtxid")]
    pub spending_txid: Option<String>,
}
