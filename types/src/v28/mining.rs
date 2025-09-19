// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v28` - mining.
//!
//! Types for methods found under the `== Mining ==` section of the API docs.

use bitcoin::Weight;
use serde::{Deserialize, Serialize};

use crate::model;

/// Result of the JSON-RPC method `getmininginfo`.
///
/// > getmininginfo
/// >
/// > Returns a json object containing mining-related information.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMiningInfo {
    /// The current block.
    pub blocks: u64,
    /// The last block weight.
    #[serde(rename = "currentblockweight")]
    pub current_block_weight: Option<u64>,
    /// The last block transaction.
    #[serde(rename = "currentblocktx")]
    pub current_block_tx: Option<i64>,
    /// The current difficulty.
    pub difficulty: f64,
    /// The network hashes per second.
    #[serde(rename = "networkhashps")]
    pub network_hash_ps: i64,
    /// The size of the mempool.
    #[serde(rename = "pooledtx")]
    pub pooled_tx: i64,
    /// Current network name as defined in BIP70 (main, test, regtest).
    pub chain: String,
    /// Any network and blockchain warnings.
    pub warnings: Vec<String>,
}

impl GetMiningInfo {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> model::GetMiningInfo {
        let current_block_weight = self.current_block_weight.map(Weight::from_wu);

        model::GetMiningInfo {
            blocks: self.blocks,
            current_block_weight,
            current_block_tx: self.current_block_tx,
            bits: None,
            difficulty: self.difficulty,
            target: None,
            network_hash_ps: self.network_hash_ps,
            pooled_tx: self.pooled_tx,
            chain: self.chain,
            signet_challenge: None,
            next: None,
            warnings: self.warnings,
        }
    }
}
