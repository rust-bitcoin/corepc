// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v29` - blockchain.
//!
//! Types for methods found under the `== Blockchain ==` section of the API docs.
use serde::{Deserialize, Serialize};

mod error;
mod into;

use alloc::collections::BTreeMap;

use bitcoin::{Network, TxMerkleNode};

pub use self::error::{
    GetBlockHeaderError, GetBlockHeaderVerboseError, GetBlockVerboseOneError,
    GetBlockchainInfoError, GetDescriptorActivityError,
};
use crate::model;

/// Result of JSON-RPC method `getblock` with verbosity set to 1.
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
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
    /// The merkle root
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
    /// The bits.
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
    /// The bits
    pub bits: String,
    /// The difficulty target.
    pub target: String,
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
    /// Status of softforks in progress.
    pub softforks: Vec<Softfork>,
    /// Status of BIP-9 softforks in progress, maps softfork name -> [`Softfork`].
    pub bip9_softforks: BTreeMap<String, Bip9Softfork>,
    /// The block challenge (aka. block script)
    pub signet_challenge: String,
    /// Any network and blockchain warnings.
    pub warnings: String,
}

/// Status of softfork.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Softfork {
    /// Name of softfork.
    pub id: String,
    /// Block version.
    pub version: i64,
    /// Progress toward rejecting pre-softfork blocks.
    pub reject: SoftforkReject,
}

/// Progress toward rejecting pre-softfork blocks.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct SoftforkReject {
    /// `true` if threshold reached.
    pub status: bool,
}

/// Status of BIP-9 softforksin progress.
#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct Bip9Softfork {
    /// One of "defined", "started", "locked_in", "active", "failed".
    pub status: Bip9SoftforkStatus,
    /// The bit (0-28) in the block version field used to signal this softfork (only for "started" status).
    pub bit: Option<u8>,
    /// The minimum median time past of a block at which the bit gains its meaning.
    #[serde(rename = "startTime")]
    pub start_time: i64,
    /// The median time past of a block at which the deployment is considered failed if not yet locked in.
    pub timeout: i64,
    /// Height of the first block to which the status applies.
    pub since: i64,
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

/// Result of JSON-RPC method `getblockheader` with verbosity set to `false`.
///
/// > Arguments:
/// > 1. "hash"          (string, required) The block hash
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBlockHeader(pub String);

/// Result of JSON-RPC method `getblockheader` with verbosity set to `true`.
///
/// > If verbose is false, returns a string that is serialized, hex-encoded data for blockheader 'hash'.
/// > If verbose is true, returns an Object with information about blockheader `<hash>`.
/// >
/// > Arguments:
/// > 1. "hash"          (string, required) The block hash
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
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
    /// The difficulty target (hex-encoded). From v29+
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

/// Result of JSON-RPC method `getdescriptoractivity`.
///
/// > getdescriptoractivity ( ["blockhash",...] [scanobjects,...] include_mempool )
/// >
/// > Arguments:
/// > 1. blockhashes  (json array, optional) The list of blockhashes to examine for activity. Order doesn't matter. Must be along main chain or an error is thrown.
/// > 2. scanobjects  (json array, optional) Array of scan objects. Required for "start" action
/// > 3. include_mempool  (boolean, optional, default=true) Whether to include unconfirmed activitydata
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetDescriptorActivity {
    pub activity: Vec<ActivityEntry>,
}

/// A script pubkey.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ScriptPubkey {
    /// Script assembly.
    pub asm: String,
    /// Script hex.
    pub hex: String,
    /// The type, eg pubkeyhash.
    #[serde(rename = "type")]
    pub type_: String,
    /// bitcoin address.
    pub address: Option<String>,
}

/// Represents a 'spend' activity event.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SpendActivity {
    // Note: 'type' field is used for deserialization tag, not included here explicitly
    /// The total amount in BTC of the spent output
    pub amount: f64,
    /// The blockhash
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "blockhash")]
    pub block_hash: Option<String>,
    /// Height of the spend
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// The txid of the spending transaction
    pub spend_txid: String,
    /// The vout of the spend
    pub spend_vout: u32,
    /// The txid of the prevout
    pub prevout_txid: String,
    /// The vout of the spend
    pub prevout_vout: u32,
    /// The prev scriptPubKey
    pub prevout_spk: ScriptPubkey,
}

/// Represents a 'receive' activity event.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ReceiveActivity {
    // Note: 'type' field is used for deserialization tag, not included here explicitly
    /// The total amount in BTC of the new output
    pub amount: f64,
    /// The block that this receive is in
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "blockhash")]
    pub block_hash: Option<String>,
    /// The height of the receive
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// The txid of the receiving transaction
    pub txid: String,
    /// The vout of the receiving output
    pub vout: u32,
    /// The ScriptPubKey
    pub output_spk: ScriptPubkey,
}

/// Enum representing either a spend or receive activity entry.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ActivityEntry {
    /// The spend activity
    Spend(SpendActivity),
    /// The receive activity
    Receive(ReceiveActivity),
}
