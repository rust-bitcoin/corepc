// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v28` - blockchain.
//!
//! Types for methods found under the `== Blockchain ==` section of the API docs.

use alloc::collections::BTreeMap;

use bitcoin::{BlockHash, Network, Work};
use serde::{Deserialize, Serialize};

use super::{GetBlockchainInfoError, Softfork};
use crate::model;
use crate::v22::ScanTxOutSetStatus;

/// Result of JSON-RPC method `getblockchaininfo`.
///
/// Method call: `getblockchaininfo`
///
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
    pub warnings: Vec<String>,
}

impl GetBlockchainInfo {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetBlockchainInfo, GetBlockchainInfoError> {
        use GetBlockchainInfoError as E;

        let chain = Network::from_core_arg(&self.chain).map_err(E::Chain)?;
        let best_block_hash =
            self.best_block_hash.parse::<BlockHash>().map_err(E::BestBlockHash)?;
        let chain_work = Work::from_unprefixed_hex(&self.chain_work).map_err(E::ChainWork)?;
        let prune_height =
            self.prune_height.map(|h| crate::to_u32(h, "prune_height")).transpose()?;
        let prune_target_size =
            self.prune_target_size.map(|h| crate::to_u32(h, "prune_target_size")).transpose()?;
        let softforks = BTreeMap::new(); // TODO: Handle softforks stuff.

        Ok(model::GetBlockchainInfo {
            chain,
            blocks: crate::to_u32(self.blocks, "blocks")?,
            headers: crate::to_u32(self.headers, "headers")?,
            best_block_hash,
            difficulty: self.difficulty,
            median_time: crate::to_u32(self.median_time, "median_time")?,
            verification_progress: self.verification_progress,
            initial_block_download: self.initial_block_download,
            chain_work,
            size_on_disk: self.size_on_disk,
            pruned: self.pruned,
            prune_height,
            automatic_pruning: self.automatic_pruning,
            prune_target_size,
            softforks,
            warnings: self.warnings,
        })
    }
}

/// Result of JSON-RPC method `scantxoutset`.
///
/// > scantxoutset "action" ( [scanobjects,...] )
/// >
/// > Arguments:
/// > 1. action                        (string, required) The action to execute
/// >    "start" for starting a scan
/// >    "abort" for aborting the current scan (returns true when abort was successful)
/// >    "status" for progress report (in %) of the current scan
/// 2. scanobjects                   (json array, required) Array of scan objects
///    Every scan object is either a string descriptor or an object:
///    [
///    "descriptor",             (string) An output descriptor
///    {                         (json object) An object with output descriptor and metadata
///    "desc": "str",          (string, required) An output descriptor
///    "range": n or \[n,n\],   (numeric or array, optional, default=1000) The range of HD chain indexes to explore (either end or \[begin,end\])
///    },
///    ...
///    ]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)] // v28
pub struct ScanTxOutSetStart {
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

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ScanTxOutSetUnspent {
    /// The transaction id
    pub txid: String,
    /// The vout value
    pub vout: u32,
    /// The script key
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: String,
    /// An output descriptor
    pub desc: String,
    /// The total amount in BTC of unspent output
    pub amount: f64,
    /// Whether this is a coinbase output
    pub coinbase: bool,
    /// Height of the unspent transaction output
    pub height: u64,
    /// Blockhash of the unspent transaction output
    pub blockhash: String,
    /// Number of confirmations of the unspent transaction output when the scan was done
    pub confirmations: u64,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ScanTxOutSet {
    Start(ScanTxOutSetStart),
    Abort(bool),
    Status(Option<ScanTxOutSetStatus>),
}
