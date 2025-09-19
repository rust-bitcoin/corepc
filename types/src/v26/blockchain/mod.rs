// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v26` - blockchain.
//!
//! Types for methods found under the `== Blockchain ==` section of the API docs.

mod error;
mod into;

use serde::{Deserialize, Serialize};

pub use self::error::{
    DumpTxOutSetError, GetChainStatesError, GetTxOutSetInfoError, LoadTxOutSetError,
};

/// Result of JSON-RPC method `dumptxoutset`.
///
/// > dumptxoutset "path"
/// >
/// > Write the serialized UTXO set to disk.
/// >
/// > Arguments:
/// > 1. path    (string, required) Path to the output file. If relative, will be prefixed by datadir.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DumpTxOutSet {
    /// The number of coins written in the snapshot.
    pub coins_written: f64,
    /// The hash of the base of the snapshot.
    pub base_hash: String,
    /// The height of the base of the snapshot.
    pub base_height: i64,
    /// The absolute path that the snapshot was written to.
    pub path: String,
    /// The hash of the UTXO set contents.
    #[serde(rename = "txoutset_hash")]
    pub tx_out_set_hash: String,
    /// The number of transactions in the chain up to and including the base block.
    #[serde(rename = "nchaintx")]
    pub n_chain_tx: i64,
}

/// Result of JSON-RPC method `getchainstates`.
///
/// > getchainstates
/// >
/// > Return information about chainstates.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetChainStates {
    /// The number of headers seen so far.
    pub headers: i64,
    /// List of the chainstates ordered by work, with the most-work (active) chainstate last.
    #[serde(rename = "chainstates")]
    pub chain_states: Vec<ChainState>,
}

/// A single chainstate. Part of `getchainstates`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ChainState {
    /// Number of blocks in this chainstate.
    pub blocks: i64,
    /// Blockhash of the tip.
    #[serde(rename = "bestblockhash")]
    pub best_block_hash: String,
    /// Difficulty of the tip.
    pub difficulty: f64,
    /// Progress towards the network tip.
    #[serde(rename = "verificationprogress")]
    pub verification_progress: f64,
    /// The base block of the snapshot this chainstate is based on, if any.
    #[serde(rename = "snapshot_blockhash")]
    pub snapshot_block_hash: Option<String>,
    /// Size of the coinsdb cache.
    pub coins_db_cache_bytes: u64,
    /// Size of the coinstip cache.
    pub coins_tip_cache_bytes: u64,
    /// Whether the chainstate is fully validated.
    pub validated: bool,
}

/// Result of JSON-RPC method `gettxoutsetinfo`.
///
/// > gettxoutsetinfo
/// >
/// > Returns statistics about the unspent transaction output set.
/// > Note this call may take some time.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxOutSetInfo {
    /// The current block height (index).
    pub height: i64,
    /// The hash of the block at the tip of the chain.
    #[serde(rename = "bestblock")]
    pub best_block: String,
    /// The number of transactions with unspent outputs.
    pub transactions: i64,
    /// The number of unspent transaction outputs.
    #[serde(rename = "txouts")]
    pub tx_outs: i64,
    /// A meaningless metric for UTXO set size.
    #[serde(rename = "bogosize")]
    pub bogo_size: i64,
    /// The estimated size of the chainstate on disk.
    pub disk_size: i64,
    /// The total amount.
    pub total_amount: f64,
    /// The serialized hash (only present if 'hash_serialized_3' hash_type is chosen).
    /// v26 and later only.
    pub hash_serialized_3: Option<String>,
}

/// Result of JSON-RPC method `loadtxoutset`.
///
/// > loadtxoutset "path"
/// >
/// > Load the serialized UTXO set from a file.
/// > Once this snapshot is loaded, its contents will be deserialized into a second chainstate data structure, which is then used to sync to the network's tip.
/// >
/// > Arguments:
/// > 1. path    (string, required) path to the snapshot file. If relative, will be prefixed by datadir.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct LoadTxOutSet {
    /// The number of coins loaded from the snapshot.
    pub coins_loaded: f64,
    /// The hash of the base of the snapshot.
    pub tip_hash: String,
    /// The height of the base of the snapshot.
    pub base_height: i64,
    /// The absolute path that the snapshot was loaded from.
    pub path: String,
}
