// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v0.21` - blockchain.
//!
//! Types for methods found under the `== Blockchain ==` section of the API docs.

mod into;

use alloc::collections::BTreeMap;

use serde::{Deserialize, Serialize};

pub use super::{
    Bip9SoftforkStatistics, Bip9SoftforkStatus, GetBlockchainInfoError, GetMempoolInfoError,
    MapMempoolEntryError, MempoolEntryError, MempoolEntryFees,
};

/// Result of JSON-RPC method `getblockchaininfo`.
///
/// > getblockchaininfo
/// >
/// > Returns an object containing various state info regarding blockchain processing.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockchainInfo {
    /// Current network name as defined in BIP70 (main, test, signet, regtest).
    pub chain: String,
    /// The current number of blocks processed in the server.
    pub blocks: i64,
    /// The current number of headers we have validated.
    pub headers: i64,
    /// The hash of the currently best block.
    #[serde(rename = "bestblockhash")]
    pub best_block_hash: String,
    /// The current difficulty.
    pub difficulty: f64,
    /// Median time for the current best block.
    #[serde(rename = "mediantime")]
    pub median_time: i64,
    /// Estimate of verification progress (between 0 and 1).
    #[serde(rename = "verificationprogress")]
    pub verification_progress: f64,
    /// Estimate of whether this node is in Initial Block Download (IBD) mode.
    #[serde(rename = "initialblockdownload")]
    pub initial_block_download: bool,
    /// Total amount of work in active chain, in hexadecimal.
    #[serde(rename = "chainwork")]
    pub chain_work: String,
    /// The estimated size of the block and undo files on disk.
    pub size_on_disk: u64,
    /// If the blocks are subject to pruning.
    pub pruned: bool,
    /// Lowest-height complete block stored (only present if pruning is enabled).
    #[serde(rename = "pruneheight")]
    pub prune_height: Option<i64>,
    /// Whether automatic pruning is enabled (only present if pruning is enabled).
    pub automatic_pruning: Option<bool>,
    /// The target size used by pruning (only present if automatic pruning is enabled).
    pub prune_target_size: Option<i64>,
    /// Status of softforks in progress, maps softfork name -> [`Softfork`].
    #[serde(default)]
    pub softforks: BTreeMap<String, Softfork>,
    /// Any network and blockchain warnings.
    pub warnings: String,
}

/// Softfork status. Part of `getblockchaininfo`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Softfork {
    /// The [`SoftforkType`]: one of "buried", "bip9".
    #[serde(rename = "type")]
    pub type_: SoftforkType,
    /// The status of bip9 softforks (only for "bip9" type).
    pub bip9: Option<Bip9SoftforkInfo>,
    ///  Height of the first block which the rules are or will be enforced (only for "buried" type, or "bip9" type with "active" status).
    pub height: Option<i64>,
    /// `true` if the rules are enforced for the mempool and the next block.
    pub active: bool,
}

/// The softfork type. Part of `getblockchaininfo`.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SoftforkType {
    /// Softfork is "buried" (as defined in [BIP-90]).
    ///
    /// [BIP-90] <https://github.com/bitcoin/bips/blob/master/bip-0090.mediawiki>
    Buried,
    /// Softfork is "bip9" (see [BIP-9]).
    ///
    /// [BIP-9] <https://github.com/bitcoin/bips/blob/master/bip-0009.mediawiki>
    Bip9,
}

/// BIP-9 softfork info. Part of `getblockchaininfo`.
#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Bip9SoftforkInfo {
    /// One of "defined", "started", "locked_in", "active", "failed".
    pub status: Bip9SoftforkStatus,
    /// The bit (0-28) in the block version field used to signal this softfork (only for "started" status).
    pub bit: Option<u8>,
    /// The minimum median time past of a block at which the bit gains its meaning.
    pub start_time: i64,
    /// The median time past of a block at which the deployment is considered failed if not yet locked in.
    pub timeout: i64,
    /// Height of the first block to which the status applies.
    pub since: i64,
    /// Minimum height of blocks for which the rules may be enforced. v0.21 and later only.
    pub min_activation_height: i64,
    /// Numeric statistics about BIP-9 signalling for a softfork (only for "started" status).
    pub statistics: Option<Bip9SoftforkStatistics>,
}

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
    /// This is different from actual serialized size for witness transactions as witness data is discounted.  v0.19 and later only.
    pub vsize: i64,
    /// Transaction weight as defined in BIP 141.
    pub weight: i64,
    /// DEPRECATED: Transaction fee in BTC.
    pub fee: f64,
    /// DEPRECATED: Transaction fee with fee deltas used for mining priority.
    #[serde(rename = "modifiedfee")]
    pub modified_fee: f64,
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
    /// DEPRECATED: Modified fees (see above) of in-mempool descendants (including this one).
    #[serde(rename = "descendantfees")]
    pub descendant_fees: f64,
    /// Number of in-mempool ancestor transactions (including this one).
    #[serde(rename = "ancestorcount")]
    pub ancestor_count: i64,
    /// Virtual transaction size of in-mempool ancestors (including this one).
    #[serde(rename = "ancestorsize")]
    pub ancestor_size: i64,
    /// DEPRECATED: Modified fees (see above) of in-mempool ancestors (including this one).
    #[serde(rename = "ancestorfees")]
    pub ancestor_fees: f64,
    /// Hash of serialized transaction, including witness data.
    pub wtxid: String,
    /// Fee object which contains the base fee, modified fee (with fee deltas), and ancestor/descendant fee totals all in BTC.
    pub fees: MempoolEntryFees,
    /// Unconfirmed transactions used as inputs for this transaction (parent transaction id).
    pub depends: Vec<String>,
    /// Unconfirmed transactions spending outputs from this transaction (child transaction id).
    #[serde(rename = "spentby")]
    pub spent_by: Vec<String>,
    /// Whether this transaction could be replaced due to BIP125 (replace-by-fee)
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
