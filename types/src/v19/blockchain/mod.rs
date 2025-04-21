// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v0.19` - blockchain.
//!
//! Types for methods found under the `== Blockchain ==` section of the API docs.

mod error;
mod into;

use alloc::collections::BTreeMap;

use serde::{Deserialize, Serialize};

// TODO: Remove wildcard, use explicit types.
pub use self::error::*;
pub use crate::v18::ScanTxOutSetUnspent;

/// Result of JSON-RPC method `getblockchaininfo`.
///
/// > getblockchaininfo
/// >
/// > Returns an object containing various state info regarding blockchain processing.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
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

/// Status of softfork.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Softfork {
    /// The [`SoftforkType`]: one of "burried", "bip9".
    #[serde(rename = "type")]
    pub type_: SoftforkType,
    /// The status of bip9 softforks (only for "bip9" type).
    pub bip9: Option<Bip9SoftforkInfo>,
    ///  Height of the first block which the rules are or will be enforced (only for "buried" type, or "bip9" type with "active" status).
    pub height: Option<i64>,
    /// `true` if the rules are enforced for the mempool and the next block.
    pub active: bool,
}

/// The softfork type: one of "burried", "bip9".
#[derive(Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SoftforkType {
    /// Softfork is "burried" (as defined in [BIP-90]).
    ///
    /// [BIP-90] <https://github.com/bitcoin/bips/blob/master/bip-0090.mediawiki>
    Buried,
    /// Softfork is "bip9" (see [BIP-9]).
    ///
    /// [BIP-9] <https://github.com/bitcoin/bips/blob/master/bip-0009.mediawiki>
    Bip9,
}

/// Status of BIP-9 softforks.
#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
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
    /// Numeric statistics about BIP-9 signalling for a softfork (only for "started" status).
    pub statistics: Option<Bip9SoftforkStatistics>,
}

/// BIP-9 softfork status: one of "defined", "started", "locked_in", "active", "failed".
#[derive(Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Bip9SoftforkStatus {
    /// BIP-9 softfork status "defined".
    Defined,
    /// BIP-9 softfork status "started".
    Started,
    /// BIP-9 softfork status "locked_in".
    LockedIn,
    /// BIP-9 softfork status "active".
    Active,
    /// BIP-9 softfork status "failed".
    Failed,
}

/// Statistics for a BIP-9 softfork.
#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct Bip9SoftforkStatistics {
    /// The length in blocks of the BIP9 signalling period.
    pub period: i64,
    /// The number of blocks with the version bit set required to activate the feature.
    pub threshold: Option<i64>,
    /// The number of blocks elapsed since the beginning of the current period.
    pub elapsed: i64,
    /// The number of blocks with the version bit set in the current period.
    pub count: i64,
    /// `false` if there are not enough blocks left in this period to pass activation threshold.
    pub possible: Option<bool>,
}

/// Result of JSON-RPC method `getblockfilter`.
///
/// > getblockfilter "blockhash" ( "filtertype" )
/// >
/// > Retrieve a BIP 157 content filter for a particular block.
/// >
/// > Arguments:
/// > 1. blockhash     (string, required) The hash of the block
/// > 2. filtertype    (string, optional, default=basic) The type name of the filter
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetBlockFilter {
    /// The hex-encoded filter data.
    pub filter: String,
    /// The hex-encoded filter header.
    pub header: String,
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
pub struct GetMempoolAncestors(pub Vec<String>);

/// Result of JSON-RPC method `getmempoolancestors` with verbose set to true.
///
/// Map of txid to `MempoolEntry` i.e., an ancestor.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
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
pub struct GetMempoolDescendants(pub Vec<String>);

/// Result of JSON-RPC method `getmempooldescendants` with verbose set to true.
///
/// Map of txid to [`MempoolEntry`] i.e., a descendant.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
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
pub struct GetMempoolEntry(pub MempoolEntry);

/// A relative (ancestor or descendant) transaction of a transaction in the mempool.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MempoolEntry {
    /// Virtual transaction size as defined in BIP 141.
    ///
    /// This is different from actual serialized size for witness transactions as witness data is discounted.
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
    /// (No docs in Core v19.)
    pub fees: MempoolEntryFees,
    /// Unconfirmed transactions used as inputs for this transaction (parent transaction id).
    pub depends: Vec<String>,
    /// Unconfirmed transactions spending outputs from this transaction (child transaction id).
    #[serde(rename = "spentby")]
    pub spent_by: Vec<String>,
}

/// (No docs in Core v19.)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MempoolEntryFees {
    /// Transaction fee in BTC.
    pub base: f64,
    /// Transaction fee with fee deltas used for mining priority in BTC.
    pub modified: f64,
    /// Modified fees (see above) of in-mempool ancestors (including this one) in BTC
    pub ancestor: f64,
    /// Modified fees (see above) of in-mempool descendants (including this one) in BTC.
    pub descendant: f64,
}

/// Result of JSON-RPC method `scantxoutset`.
///
/// > scantxoutset "action" ( [scanobjects,...] )
/// >
/// > Arguments:
/// > 1. action                        (string, required) The action to execute
/// >   "start" for starting a scan
/// >   "abort" for aborting the current scan (returns true when abort was successful)
/// >   "status" for progress report (in %) of the current scan
/// 2. scanobjects                   (json array, required) Array of scan objects
/// > Every scan object is either a string descriptor or an object:
/// > [
/// > "descriptor",             (string) An output descriptor
/// > {                         (json object) An object with output descriptor and metadata
/// > "desc": "str",          (string, required) An output descriptor
/// > "range": n or \[n,n\],   (numeric or array, optional, default=1000) The range of HD chain indexes to explore (either end or \[begin,end\])
/// > },
/// > ...
/// > ]

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ScanTxOutSet {
    /// Whether the scan is completed
    pub success: bool,
    /// The number of unspent transaction outputs scanned
    pub txouts: u64,
    /// The current block height (index)
    pub height: u64,
    /// The hash of the block at the tip of the chain
    pub bestblock: String,
    /// The unspents
    pub unspents: Vec<ScanTxOutSetUnspent>,
    /// The total amount of all found unspent outputs in BTC
    pub total_amount: f64,
}
