// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v29` - blockchain.
//!
//! Types for methods found under the `== Blockchain ==` section of the API docs.
use serde::{Deserialize, Serialize};

mod error;
mod into;

use bitcoin::{Network, TxMerkleNode};

pub use self::error::{
    GetBlockHeaderError, GetBlockHeaderVerboseError, GetBlockVerboseOneError,
    GetBlockchainInfoError, GetChainStatesError, GetDescriptorActivityError,
};
use crate::{model, ScriptPubkey};

/// Result of JSON-RPC method `getblock` with verbosity set to 1.
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockVerboseOne {
    /// The block hash (same as provided) in RPC call.
    pub hash: String,
    /// The number of confirmations, or -1 if the block is not on the main chain.
    pub confirmations: i64,
    /// The block size.
    pub size: i64,
    /// The block size excluding witness data.
    #[serde(rename = "strippedsize")]
    pub stripped_size: Option<i64>,
    /// The block weight as defined in BIP-141.
    pub weight: u64,
    /// The block height or index.
    pub height: i64,
    /// The block version.
    pub version: i32,
    /// The block version formatted in hexadecimal.
    #[serde(rename = "versionHex")]
    pub version_hex: String,
    /// The merkle root.
    #[serde(rename = "merkleroot")]
    pub merkle_root: String,
    /// The transaction ids.
    pub tx: Vec<String>,
    /// The block time expressed in UNIX epoch time.
    pub time: i64,
    /// The median block time expressed in UNIX epoch time.
    #[serde(rename = "mediantime")]
    pub median_time: Option<i64>,
    /// The nonce (this should be only 4 bytes).
    pub nonce: i64,
    /// nBits: compact representation of the block difficulty target.
    pub bits: String,
    /// The difficulty target.
    pub target: String,
    /// The difficulty.
    pub difficulty: f64,
    /// Expected number of hashes required to produce the chain up to this block (in hex).
    #[serde(rename = "chainwork")]
    pub chain_work: String,
    /// The number of transactions in the block.
    #[serde(rename = "nTx")]
    pub n_tx: i64,
    /// The hash of the previous block (if available).
    #[serde(rename = "previousblockhash")]
    pub previous_block_hash: Option<String>,
    /// The hash of the next block (if available).
    #[serde(rename = "nextblockhash")]
    pub next_block_hash: Option<String>,
}

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
    /// nBits: compact representation of the block difficulty target.
    pub bits: String,
    /// The difficulty target.
    pub target: String,
    /// The current difficulty.
    pub difficulty: f64,
    /// The block time expressed in UNIX epoch time. v23 and later only.
    pub time: i64,
    /// The median block time expressed in UNIX epoch time.
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
    /// The block challenge (aka. block script).
    pub signet_challenge: Option<String>,
    /// Any network and blockchain warnings.
    pub warnings: Vec<String>,
}

/// Result of JSON-RPC method `getblockheader` with verbosity set to `false`.
///
/// > Arguments:
/// > 1. "hash"          (string, required) The block hash
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockHeader(pub String);

/// Result of JSON-RPC method `getblockheader` with verbosity set to `true`.
///
/// > If verbose is false, returns a string that is serialized, hex-encoded data for blockheader 'hash'.
/// > If verbose is true, returns an Object with information about blockheader `<hash>`.
/// >
/// > Arguments:
/// > 1. "hash"          (string, required) The block hash
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockHeaderVerbose {
    /// The block hash.
    pub hash: String,
    /// The number of confirmations, or -1 if the block is not on the main chain.
    pub confirmations: i64,
    /// The block height or index.
    pub height: i64,
    /// The block version.
    pub version: i32,
    /// The block version formatted in hexadecimal.
    #[serde(rename = "versionHex")]
    pub version_hex: String,
    /// The merkle root.
    #[serde(rename = "merkleroot")]
    pub merkle_root: String,
    /// The block time in seconds since epoch (Jan 1 1970 GMT).
    pub time: i64,
    /// The median block time in seconds since epoch (Jan 1 1970 GMT).
    #[serde(rename = "mediantime")]
    pub median_time: i64,
    /// The nonce.
    pub nonce: i64,
    /// The bits.
    pub bits: String,
    /// The difficulty target (hex-encoded). From v29+.
    pub target: String,
    /// The difficulty.
    pub difficulty: f64,
    /// Expected number of hashes required to produce the current chain (in hex).
    #[serde(rename = "chainwork")]
    pub chain_work: String,
    /// The number of transactions in the block.
    #[serde(rename = "nTx")]
    pub n_tx: u32,
    /// The hash of the previous block (if available).
    #[serde(rename = "previousblockhash")]
    pub previous_block_hash: Option<String>,
    /// The hash of the next block (if available).
    #[serde(rename = "nextblockhash")]
    pub next_block_hash: Option<String>,
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
    /// nBits: compact representation of the block difficulty target.
    pub bits: String,
    /// The difficulty target.
    pub target: String,
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

/// Result of JSON-RPC method `getdescriptoractivity`.
///
/// > getdescriptoractivity ( ["blockhash",...] [scanobjects,...] include_mempool )
/// >
/// > Arguments:
/// > 1. blockhashes  (json array, optional) The list of blockhashes to examine for activity. Order doesn't matter. Must be along main chain or an error is thrown.
/// > 2. scanobjects  (json array, optional) Array of scan objects. Required for "start" action
/// > 3. include_mempool  (boolean, optional, default=true) Whether to include unconfirmed activitydata
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDescriptorActivity {
    pub activity: Vec<ActivityEntry>,
}

/// Enum representing either a spend or receive activity entry. Part of `getdescriptoractivity`.
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ActivityEntry {
    /// The spend activity.
    Spend(SpendActivity),
    /// The receive activity.
    Receive(ReceiveActivity),
}

/// Represents a 'spend' activity event. Part of `getdescriptoractivity`.
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SpendActivity {
    // Note: 'type' field is used for deserialization tag, not included here explicitly.
    /// The total amount in BTC of the spent output.
    pub amount: f64,
    /// The block hash.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "blockhash")]
    pub block_hash: Option<String>,
    /// Height of the spend.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// The txid of the spending transaction.
    pub spend_txid: String,
    /// The vout of the spend.
    pub spend_vout: u32,
    /// The txid of the prevout.
    pub prevout_txid: String,
    /// The vout of the spend.
    pub prevout_vout: u32,
    /// The prev scriptPubKey.
    pub prevout_spk: ScriptPubkey,
}

/// Represents a 'receive' activity event. Part of `getdescriptoractivity`.
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ReceiveActivity {
    // Note: 'type' field is used for deserialization tag, not included here explicitly.
    /// The total amount in BTC of the new output.
    pub amount: f64,
    /// The block that this receive is in.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "blockhash")]
    pub block_hash: Option<String>,
    /// The height of the receive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// The txid of the receiving transaction.
    pub txid: String,
    /// The vout of the receiving output.
    pub vout: u32,
    /// The ScriptPubKey.
    pub output_spk: ScriptPubkey,
}
