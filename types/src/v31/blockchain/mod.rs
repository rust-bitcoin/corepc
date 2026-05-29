// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v31` - blockchain.
//!
//! Types for methods found under the `== Blockchain ==` section of the API docs.

mod error;
mod into;

use alloc::collections::BTreeMap;

use serde::{Deserialize, Serialize};

pub use self::error::{
    GetMempoolClusterError, GetMempoolFeerateDiagramError, MapMempoolEntryError, MempoolEntryError,
    MempoolEntryFeesError,
};
pub use super::GetMempoolInfoError;

/// Result of JSON-RPC method `getmempoolcluster`.
///
/// > getmempoolcluster "txid"
/// >
/// > Returns mempool data for given cluster
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolCluster {
    /// Total sigops-adjusted weight (as defined in BIP 141 and modified by `-bytespersigop`).
    #[serde(rename = "clusterweight")]
    pub cluster_weight: u64,
    /// Number of transactions.
    #[serde(rename = "txcount")]
    pub tx_count: u64,
    /// Chunks in this cluster (in mining order).
    pub chunks: Vec<Chunk>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Chunk {
    /// Fees of the transactions in this chunk.
    #[serde(rename = "chunkfee")]
    pub chunk_fee: f64,
    /// Sigops-adjusted weight of all transactions in this chunk.
    #[serde(rename = "chunkweight")]
    pub chunk_weight: u64,
    /// Transactions in this chunk in mining order.
    pub txs: Vec<String>,
}

/// Result of JSON-RPC method `getmempoolentry`.
///
/// > getmempoolentry "txid"
/// >
/// > Returns mempool data for given transaction
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolEntry(pub MempoolEntry);

/// Result of JSON-RPC method `getrawmempool` with verbose set to `true`.
///
/// Map of txid to [`MempoolEntry`].
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawMempoolVerbose(pub BTreeMap<String, MempoolEntry>);

/// Result of JSON-RPC method `getmempoolancestors` with verbose set to `true`.
///
/// Map of txid to [`MempoolEntry`].
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolAncestorsVerbose(pub BTreeMap<String, MempoolEntry>);

/// Result of JSON-RPC method `getmempooldescendants` with verbose set to `true`.
///
/// Map of txid to [`MempoolEntry`].
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolDescendantsVerbose(pub BTreeMap<String, MempoolEntry>);

/// Mempool data. Part of `getmempoolentry`, `getrawmempool`, `getmempoolancestors` and
/// `getmempooldescendants`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct MempoolEntry {
    /// Virtual transaction size as defined in BIP 141.
    ///
    /// This is different from actual serialized size for witness transactions as witness data is
    /// discounted.
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
    /// Sigops-adjusted weight (as defined in BIP 141 and modified by `-bytespersigop`) of this
    /// transaction's chunk.
    #[serde(rename = "chunkweight")]
    pub chunk_weight: i64,
    /// Hash of serialized transaction, including witness data.
    pub wtxid: String,
    /// Fee object which contains the base fee, modified fee (with fee deltas), ancestor/descendant
    /// fee totals and chunk fee, all in BTC.
    pub fees: MempoolEntryFees,
    /// Unconfirmed transactions used as inputs for this transaction (parent transaction id).
    pub depends: Vec<String>,
    /// Unconfirmed transactions spending outputs from this transaction (child transaction id).
    #[serde(rename = "spentby")]
    pub spent_by: Vec<String>,
    /// Whether this transaction signals BIP125 replaceability or has an unconfirmed ancestor
    /// signaling BIP125 replaceability (DEPRECATED).
    #[serde(rename = "bip125-replaceable")]
    pub bip125_replaceable: bool,
    /// Whether this transaction is currently unbroadcast (initial broadcast not yet acknowledged by
    /// any peers).
    pub unbroadcast: bool,
}

/// Fee object. Part of `getmempoolentry`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct MempoolEntryFees {
    /// Transaction fee, denominated in BTC.
    pub base: f64,
    /// Transaction fee with fee deltas used for mining priority, denominated in BTC.
    pub modified: f64,
    /// Transaction fees of in-mempool ancestors (including this one) with fee deltas used for
    /// mining priority, denominated in BTC.
    pub ancestor: f64,
    /// Transaction fees of in-mempool descendants (including this one) with fee deltas used for
    /// mining priority, denominated in BTC.
    pub descendant: f64,
    /// Transaction fees of chunk, denominated in BTC.
    pub chunk: f64,
}

/// Result of JSON-RPC method `getmempoolinfo` with verbose set to `true`.
///
/// > getmempoolinfo
/// >
/// > Returns details on the active state of the TX memory pool.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolInfo {
    /// True if the initial load attempt of the persisted mempool finished.
    pub loaded: bool,
    /// Current tx count.
    pub size: i64,
    /// Sum of all virtual transaction sizes as defined in BIP 141.
    ///
    /// Differs from actual serialized size because witness data is discounted.
    pub bytes: i64,
    /// Total memory usage for the mempool.
    pub usage: i64,
    /// Total fees for the mempool in BTC, ignoring modified fees through prioritisetransaction.
    pub total_fee: f64,
    /// Maximum memory usage for the mempool.
    #[serde(rename = "maxmempool")]
    pub max_mempool: i64,
    /// Minimum fee rate in BTC/kvB for a transaction to be accepted.
    ///
    /// This is the maximum of `minrelaytxfee` and the minimum mempool fee.
    #[serde(rename = "mempoolminfee")]
    pub mempool_min_fee: f64,
    /// Current minimum relay fee for transactions.
    #[serde(rename = "minrelaytxfee")]
    pub min_relay_tx_fee: f64,
    /// Minimum fee rate increment for mempool limiting or replacement in BTC/kvB.
    #[serde(rename = "incrementalrelayfee")]
    pub incremental_relay_fee: f64,
    /// Current number of transactions that haven't passed initial broadcast yet.
    #[serde(rename = "unbroadcastcount")]
    pub unbroadcast_count: i64,
    /// True if the mempool accepts RBF without replaceability signaling inspection (DEPRECATED).
    #[serde(rename = "fullrbf")]
    pub full_rbf: bool,
    /// True if the mempool accepts transactions with bare multisig outputs.
    #[serde(rename = "permitbaremultisig")]
    pub permit_bare_multisig: bool,
    /// Maximum number of bytes that can be used by OP_RETURN outputs in the mempool.
    #[serde(rename = "maxdatacarriersize")]
    pub max_data_carrier_size: u64,
    /// Maximum number of transactions that can be in a cluster (configured by `-limitclustercount`).
    #[serde(rename = "limitclustercount")]
    pub limit_cluster_count: i64,
    /// Maximum size of a cluster in virtual bytes (configured by `-limitclustersize`).
    #[serde(rename = "limitclustersize")]
    pub limit_cluster_size: i64,
    /// True if the mempool is in a known-optimal transaction ordering.
    pub optimal: bool,
}

/// Result of JSON-RPC method `getmempoolfeeratediagram`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolFeerateDiagram(pub Vec<FeerateDiagramEntry>);

/// A point on the mempool feerate diagram. Part of `getmempoolfeeratediagram`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct FeerateDiagramEntry {
    /// Cumulative sigops-adjusted weight.
    pub weight: i64,
    /// Cumulative fee.
    pub fee: f64,
}
