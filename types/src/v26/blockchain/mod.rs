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
/// >
/// > Arguments:
/// > 1. hash_type         (string, optional, default="hash_serialized_3") Which UTXO set hash should be calculated. Options: 'hash_serialized_3' (the legacy algorithm), 'muhash', 'none'.
/// > 2. hash_or_height    (string or numeric, optional, default=the current best block) The block hash or height of the target height (only available with coinstatsindex).
/// > 3. use_index         (boolean, optional, default=true) Use coinstatsindex, if available.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxOutSetInfo {
    /// The current block height (index).
    pub height: i64,
    /// The hash of the block at the tip of the chain.
    #[serde(rename = "bestblock")]
    pub best_block: String,
    /// The number of transactions with unspent outputs (not available when coinstatsindex is used).
    pub transactions: Option<i64>,
    /// The number of unspent transaction outputs.
    #[serde(rename = "txouts")]
    pub tx_outs: i64,
    /// A meaningless metric for UTXO set size.
    #[serde(rename = "bogosize")]
    pub bogo_size: i64,
    /// The serialized hash (only present if 'hash_serialized_3' hash_type is chosen).
    pub hash_serialized_3: Option<String>,
    /// The estimated size of the chainstate on disk (not available when coinstatsindex is used).
    pub disk_size: Option<i64>,
    /// The total amount.
    pub total_amount: f64,
    /// The serialized hash (only present if 'muhash' hash_type is chosen).
    pub muhash: Option<String>,
    /// The total amount of coins permanently excluded from the UTXO set (only available if coinstatsindex is used).
    pub total_unspendable_amount: Option<f64>,
    /// Info on amounts in the block at this block height (only available if coinstatsindex is used).
    pub block_info: Option<GetTxOutSetInfoBlockInfo>,
}

/// Detailed block-level info returned by `gettxoutsetinfo` when coinstatsindex is enabled.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxOutSetInfoBlockInfo {
    /// Total amount of all prevouts spent in this block.
    #[serde(rename = "prevout_spent")]
    pub prevout_spent: f64,
    /// Coinbase subsidy amount of this block.
    pub coinbase: f64,
    /// Total amount of new outputs created by this block.
    #[serde(rename = "new_outputs_ex_coinbase")]
    pub new_outputs_ex_coinbase: f64,
    /// Total amount of unspendable outputs created in this block.
    pub unspendable: f64,
    /// Detailed view of unspendable categories.
    pub unspendables: GetTxOutSetInfoUnspendables,
}

/// Categories of unspendable amounts returned inside `BlockInfo`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxOutSetInfoUnspendables {
    /// The unspendable amount of the Genesis block subsidy.
    pub genesis_block: f64,
    /// Transactions overridden by duplicates (no longer possible with BIP30).
    pub bip30: f64,
    /// Amounts sent to scripts that are unspendable (for example OP_RETURN outputs).
    pub scripts: f64,
    /// Fee rewards that miners did not claim in their coinbase transaction.
    pub unclaimed_rewards: f64,
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

/// Result of JSON-RPC method `scanblocks` with action "start".
///
/// > scanblocks "start" [scanobjects,...] ( start_height stop_height "filtertype" "options" )
/// >
/// > Arguments:
/// > 1. scanobjects                            (json array, required) Array of scan objects
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ScanBlocksStart {
    /// The height we started the scan from
    pub from_height: i64,
    /// The height we ended the scan at
    pub to_height: i64,
    /// Blocks that may have matched a scanobject
    pub relevant_blocks: Vec<String>,
    /// True if the scan process was not aborted
    pub completed: bool,
}
