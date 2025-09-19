// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v23` - blockchain.
//!
//! Types for methods found under the `== Blockchain ==` section of the API docs.

mod error;
mod into;

use alloc::collections::BTreeMap;

use serde::{Deserialize, Serialize};

pub use self::error::GetDeploymentInfoError;
pub use super::{
    GetBlockchainInfoError, MapMempoolEntryError, MempoolEntryError, MempoolEntryFees, Softfork,
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
    /// The block time expressed in UNIX epoch time. v23 and later only.
    pub time: i64,
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

/// Result of the JSON-RPC method `getdeploymentinfo`.
///
/// > getdeploymentinfo ("blockhash")
/// >
/// > Returns an object containing various state info regarding deployments of consensus changes.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDeploymentInfo {
    /// Requested block hash (or tip).
    pub hash: String,
    /// Requested block height (or tip).
    pub height: u32,
    /// Deployments info, keyed by deployment name.
    pub deployments: std::collections::BTreeMap<String, DeploymentInfo>,
}

/// Deployment info. Part of `getdeploymentinfo`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DeploymentInfo {
    /// One of "buried", "bip9".
    #[serde(rename = "type")]
    pub deployment_type: String,
    /// Height of the first block which the rules are or will be enforced (only for "buried" type, or "bip9" type with "active" status).
    pub height: Option<u32>,
    /// True if the rules are enforced for the mempool and the next block.
    pub active: bool,
    /// Status of bip9 softforks (only for "bip9" type).
    pub bip9: Option<Bip9Info>,
}

/// Status of bip9 softforks. Part of `getdeploymentinfo`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Bip9Info {
    /// The bit (0-28) in the block version field used to signal this softfork (only for "started" and "locked_in" status).
    pub bit: Option<u8>,
    /// The minimum median time past of a block at which the bit gains its meaning.
    pub start_time: i64,
    /// The median time past of a block at which the deployment is considered failed if not yet locked in.
    pub timeout: i64,
    /// Minimum height of blocks for which the rules may be enforced.
    pub min_activation_height: u32,
    /// Status of deployment at specified block (one of "defined", "started", "locked_in", "active", "failed").
    pub status: String,
    /// Height of the first block to which the status applies.
    pub since: u32,
    /// Status of deployment at the next block.
    pub status_next: String,
    /// Numeric statistics about signalling for a softfork (only for "started" and "locked_in" status).
    pub statistics: Option<Bip9Statistics>,
    /// Indicates blocks that signalled with a # and blocks that did not with a -.
    pub signalling: Option<String>,
}

/// Numeric statistics about signalling for a softfork. Part of `getdeploymentinfo`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Bip9Statistics {
    /// The length in blocks of the signalling period.
    pub period: u32,
    /// The number of blocks with the version bit set required to activate the feature (only for "started" status).
    pub threshold: Option<u32>,
    /// The number of blocks elapsed since the beginning of the current period.
    pub elapsed: u32,
    /// The number of blocks with the version bit set in the current period.
    pub count: u32,
    /// Returns false if there are not enough blocks left in this period to pass activation threshold (only for "started" status).
    pub possible: Option<bool>,
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
    /// This is different from actual serialized size for witness transactions as witness data is
    /// discounted. v0.19 and later only.
    pub vsize: i64,
    /// Transaction weight as defined in BIP 141.
    pub weight: i64,
    /// Transaction fee, denominated in BTC.
    /// (DEPRECATED, returned only if config option -deprecatedrpc=fees is passed).
    pub fee: Option<f64>,
    /// Transaction fee with fee deltas used for mining priority, denominated in BTC.
    /// (DEPRECATED, returned only if config option -deprecatedrpc=fees is passed).
    #[serde(rename = "modifiedfee")]
    pub modified_fee: Option<f64>,
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
    /// Transaction fees of in-mempool descendants (including this one) with fee deltas used for
    /// mining priority, denominated in sats.
    /// (DEPRECATED, returned only if config option -deprecatedrpc=fees is passed).
    #[serde(rename = "descendantfees")]
    pub descendant_fees: Option<f64>,
    /// Number of in-mempool ancestor transactions (including this one).
    #[serde(rename = "ancestorcount")]
    pub ancestor_count: i64,
    /// Virtual transaction size of in-mempool ancestors (including this one).
    #[serde(rename = "ancestorsize")]
    pub ancestor_size: i64,
    /// Transaction fees of in-mempool ancestors (including this one) with fee deltas used for mining
    /// priority, denominated in sats.
    /// (DEPRECATED, returned only if config option -deprecatedrpc=fees is passed).
    #[serde(rename = "ancestorfees")]
    pub ancestor_fees: Option<f64>,
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

/// Result of JSON-RPC method `savemempool`.
///
/// > savemempool
///
/// > Dumps the mempool to disk. It will fail until the previous dump is fully loaded.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SaveMempool {
    /// The directory and file where the mempool was saved.
    pub filename: String,
}
