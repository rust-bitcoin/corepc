// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v28` - blockchain.
//!
//! Types for methods found under the `== Blockchain ==` section of the API docs.

use alloc::collections::BTreeMap;

use bitcoin::{BlockHash, Network, Work};
use serde::{Deserialize, Serialize};

use super::{GetBlockchainInfoError, Softfork};
use crate::model;

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
    pub warnings: Vec<String>,
}

impl GetBlockchainInfo {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetBlockchainInfo, GetBlockchainInfoError> {
        use GetBlockchainInfoError as E;

        let chain = Network::from_core_arg(&self.chain).map_err(E::Chain)?;
        let best_block_hash =
            self.best_block_hash.parse::<BlockHash>().map_err(E::BestBlockHash)?;
        let time = Some(crate::to_u32(self.time, "time")?);
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
            bits: None,
            target: None,
            difficulty: self.difficulty,
            time,
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
            signet_challenge: None,
            warnings: self.warnings,
        })
    }
}
