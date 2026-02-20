// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v28` - blockchain.
//!
//! Types for methods found under the `== Blockchain ==` section of the API docs.

mod into;

use alloc::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::{GetBlockchainInfoError, ScanTxOutSetError, Softfork};

/// Result of JSON-RPC method `getblockchaininfo`.
///
/// Method call: `getblockchaininfo`
///
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
    /// The block time expressed in UNIX epoch time.
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
    pub warnings: Vec<String>,
}

/// Result of JSON-RPC method `scantxoutset`.
///
/// > scantxoutset "action" ( [scanobjects,...] )
/// >
/// > Arguments:
/// > 1. action                        (string, required) The action to execute
/// > 2. scanobjects                   (json array, required) Array of scan objects
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ScanTxOutSetStart {
    /// Whether the scan was completed.
    pub success: bool,
    /// The number of unspent transaction outputs scanned.
    #[serde(rename = "txouts")]
    pub tx_outs: u64,
    /// The block height at which the scan was done.
    pub height: u64,
    /// The hash of the block at the tip of the chain.
    #[serde(rename = "bestblock")]
    pub best_block: String,
    /// The unspents.
    pub unspents: Vec<ScanTxOutSetUnspent>,
    /// The total amount of all found unspent outputs in BTC.
    pub total_amount: f64,
}

/// Unspent outputs. Part of `scantxoutset`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ScanTxOutSetUnspent {
    /// The transaction id.
    pub txid: String,
    /// The vout value.
    pub vout: u32,
    /// The output script.
    #[serde(rename = "scriptPubKey")]
    pub script_pubkey: String,
    /// A specialized descriptor for the matched output script.
    #[serde(rename = "desc")]
    pub descriptor: String,
    /// The total amount in BTC of the unspent output.
    pub amount: f64,
    /// Whether this is a coinbase output.
    pub coinbase: bool,
    /// Height of the unspent transaction output.
    pub height: u64,
    /// Blockhash of the unspent transaction output.
    #[serde(rename = "blockhash")]
    pub block_hash: String,
    /// Number of confirmations of the unspent transaction output when the scan was done.
    pub confirmations: u64,
}
