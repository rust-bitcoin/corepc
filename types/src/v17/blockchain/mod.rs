// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v0.17` - blockchain.
//!
//! Types for methods found under the `== Blockchain ==` section of the API docs.

mod error;
mod into;

use alloc::collections::BTreeMap;

use bitcoin::hex::FromHex;
use bitcoin::{Amount, FeeRate, Network, TxMerkleNode, TxOut, Wtxid};
use serde::{Deserialize, Serialize};

// TODO: Remove wildcard, use explicit types.
pub use self::error::*;
use crate::{model, ScriptPubkey};

/// Result of JSON-RPC method `getbestblockhash`.
///
/// > getbestblockhash
/// >
/// > Returns the hash of the best (tip) block in the most-work fully-validated chain.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBestBlockHash(pub String);

/// Result of JSON-RPC method `getblock` with verbosity set to 0.
///
/// > getblock "blockhash" ( verbosity )
/// >
/// > If verbosity is 0, returns a string that is serialized, hex-encoded data for block 'hash'.
/// > If verbosity is 1, returns an Object with information about block `<hash>`.
/// > If verbosity is 2, returns an Object with information about block `<hash>` and information about each transaction.
/// >
/// > Arguments:
/// > 1. "blockhash"          (string, required) The block hash
/// > 2. verbosity              (numeric, optional, default=1) 0 for hex encoded data, 1 for a json object, and 2 for json object with transaction data
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockVerboseZero(
    /// A string that is serialized, hex-encoded data for block 'hash'.
    pub String,
);

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
    /// Any network and blockchain warnings.
    pub warnings: String,
}

/// Softfork status. Part of `getblockchaininfo`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Softfork {
    /// Name of softfork.
    pub id: String,
    /// Block version.
    pub version: i64,
    /// Progress toward rejecting pre-softfork blocks.
    pub reject: SoftforkReject,
}

/// Progress toward rejecting pre-softfork blocks. Part of `getblockchaininfo`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SoftforkReject {
    /// `true` if threshold reached.
    pub status: bool,
}

/// Status of BIP-9 softforks in progress. Part of `getblockchaininfo`.
#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
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

/// BIP-9 softfork status. Part of `getblockchaininfo`.
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

/// Result of JSON-RPC method `getblockcount`.
///
/// > getblockcount
/// >
/// > Returns the number of blocks in the longest blockchain.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockCount(pub u64);

/// Result of JSON-RPC method `getblockhash`.
///
/// > Returns hash of block in best-block-chain at height provided.
/// >
/// > Arguments:
/// > 1. height         (numeric, required) The height index
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockHash(pub String);

/// Result of JSON-RPC method `getblockheader` with verbosity set to `false`.
///
/// > If verbose is false, returns a string that is serialized, hex-encoded data for blockheader 'hash'.
/// > If verbose is true, returns an Object with information about blockheader 'hash'.
/// >
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
    /// The block hash (same as provided).
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

/// Result of JSON-RPC method `getblockstats`.
///
/// > getblockstats hash_or_height ( stats )
///
/// > Returns the number of blocks in the longest blockchain.
/// > getblockstats hash_or_height ( stats )
/// >
/// > Compute per block statistics for a given window. All amounts are in satoshis.
/// > It won't work for some heights with pruning.
/// > It won't work without -txindex for utxo_size_inc, *fee or *feerate stats.
/// >
/// > Arguments:
/// > 1. "hash_or_height"     (string or numeric, required) The block hash or height of the target block
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockStats {
    /// Average fee in the block.
    #[serde(rename = "avgfee")]
    pub average_fee: u64,
    // FIXME: Remember these docs will become silently stale when unit changes in a later version of Core.
    /// Average feerate (in satoshis per virtual byte).
    #[serde(rename = "avgfeerate")]
    pub average_fee_rate: u64,
    /// Average transaction size.
    #[serde(rename = "avgtxsize")]
    pub average_tx_size: i64,
    /// The block hash (to check for potential reorgs).
    #[serde(rename = "blockhash")]
    pub block_hash: String,
    /// Feerates at the 10th, 25th, 50th, 75th, and 90th percentile weight unit (in satoshis per
    /// virtual byte).
    #[serde(rename = "feerate_percentiles")]
    pub fee_rate_percentiles: [u64; 5],
    /// The height of the block.
    pub height: i64,
    /// The number of inputs (excluding coinbase).
    #[serde(rename = "ins")]
    pub inputs: i64,
    /// Maximum fee in the block.
    #[serde(rename = "maxfee")]
    pub max_fee: u64,
    /// Maximum feerate (in satoshis per virtual byte).
    #[serde(rename = "maxfeerate")]
    pub max_fee_rate: u64,
    /// Maximum transaction size.
    #[serde(rename = "maxtxsize")]
    pub max_tx_size: i64,
    /// Truncated median fee in the block.
    #[serde(rename = "medianfee")]
    pub median_fee: u64,
    /// The block median time past.
    #[serde(rename = "mediantime")]
    pub median_time: i64,
    /// Truncated median transaction size
    #[serde(rename = "mediantxsize")]
    pub median_tx_size: i64,
    /// Minimum fee in the block.
    #[serde(rename = "minfee")]
    pub minimum_fee: u64,
    /// Minimum feerate (in satoshis per virtual byte).
    #[serde(rename = "minfeerate")]
    pub minimum_fee_rate: u64,
    /// Minimum transaction size.
    #[serde(rename = "mintxsize")]
    pub minimum_tx_size: i64,
    /// The number of outputs.
    #[serde(rename = "outs")]
    pub outputs: i64,
    /// The block subsidy.
    pub subsidy: u64,
    /// Total size of all segwit transactions.
    #[serde(rename = "swtotal_size")]
    pub segwit_total_size: i64,
    /// Total weight of all segwit transactions divided by segwit scale factor (4).
    #[serde(rename = "swtotal_weight")]
    pub segwit_total_weight: u64,
    /// The number of segwit transactions.
    #[serde(rename = "swtxs")]
    pub segwit_txs: i64,
    /// The block time.
    pub time: i64,
    /// Total amount in all outputs (excluding coinbase and thus reward [ie subsidy + totalfee]).
    pub total_out: u64,
    /// Total size of all non-coinbase transactions.
    pub total_size: i64,
    /// Total weight of all non-coinbase transactions divided by segwit scale factor (4).
    pub total_weight: u64,
    /// The fee total.
    #[serde(rename = "totalfee")]
    pub total_fee: u64,
    /// The number of transactions (excluding coinbase).
    pub txs: i64,
    /// The increase/decrease in the number of unspent outputs.
    pub utxo_increase: i32,
    /// The increase/decrease in size for the utxo index (not discounting op_return and similar).
    #[serde(rename = "utxo_size_inc")]
    pub utxo_size_increase: i32,
}

/// Result of JSON-RPC method `getchaintips`.
///
/// > Return information about all known tips in the block tree, including the main chain as well as orphaned branches.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetChainTips(pub Vec<ChainTips>);

/// Chain tip item. Part of `getchaintips`.
#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ChainTips {
    /// Height of the chain tip.
    pub height: i64,
    /// Block hash of the tip.
    pub hash: String,
    /// Zero for main chain.
    #[serde(rename = "branchlen")]
    pub branch_length: i64,
    /// "active" for the main chain.
    pub status: ChainTipsStatus,
}

/// Chain tip status. Part of `getchaintips`.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ChainTipsStatus {
    /// This branch contains at least one invalid block.
    Invalid,
    /// Not all blocks for this branch are available, but the headers are valid.
    HeadersOnly,
    /// All blocks are available for this branch, but they were never fully validated.
    ValidHeaders,
    /// This branch is not part of the active chain, but is fully validated.
    ValidFork,
    /// This is the tip of the active main chain, which is certainly valid.
    Active,
}

/// Result of JSON-RPC method `getchaintxstats`.
///
/// > getchaintxstats ( nblocks blockhash )
/// >
/// > Compute statistics about the total number and rate of transactions in the chain.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetChainTxStats {
    /// The timestamp for the final block in the window in UNIX format.
    pub time: i64,
    /// The total number of transactions in the chain up to that point.
    #[serde(rename = "txcount")]
    pub tx_count: i64,
    /// The hash of the final block in the window.
    pub window_final_block_hash: String,
    /// Size of the window in number of blocks.
    pub window_block_count: i64,
    /// The number of transactions in the window. Only returned if "window_block_count" is > 0.
    pub window_tx_count: Option<i64>,
    /// The elapsed time in the window in seconds. Only returned if "window_block_count" is > 0.
    pub window_interval: Option<i64>,
    /// The average rate of transactions per second in the window. Only returned if "window_interval" is > 0.
    #[serde(rename = "txrate")]
    pub tx_rate: Option<i64>,
}

/// Result of JSON-RPC method `getdifficulty`.
///
/// > getdifficulty
///
/// > Returns the proof-of-work difficulty as a multiple of the minimum difficulty.
/// >
/// > Result:
/// > n.nnn       (numeric) the proof-of-work difficulty as a multiple of the minimum difficulty.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDifficulty(pub f64);

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
/// Map of txid to [`MempoolEntry`] i.e., an ancestor.
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
    /// This is different from actual serialized size for witness transactions as witness data is discounted.
    pub size: i64,
    /// DEPRECATED: Transaction fee in BTC.
    pub fee: f64,
    /// DEPRECATED: Transaction fee with fee deltas used for mining priority.
    #[serde(rename = "modifiedfee")]
    pub modified_fee: f64,
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
    /// DEPRECATED: Modified fees (see above) of in-mempool descendants (including this one).
    #[serde(rename = "descendantfees")]
    pub descendant_fees: f64,
    /// Number of in-mempool ancestor transactions (including this one).
    #[serde(rename = "ancestorcount")]
    pub ancestor_count: i64,
    /// Virtual transaction size of in-mempool ancestors (including this one).
    #[serde(rename = "ancestorsize")]
    pub ancestor_size: i64,
    /// DEPRECATED: Modified fees (see above) of in-mempool ancestors (including this one).
    #[serde(rename = "ancestorfees")]
    pub ancestor_fees: f64,
    /// Hash of serialized transaction, including witness data.
    pub wtxid: String,
    /// (No docs in Core v0.17.)
    pub fees: MempoolEntryFees,
    /// Unconfirmed transactions used as inputs for this transaction (parent transaction id).
    pub depends: Vec<String>,
    /// Unconfirmed transactions spending outputs from this transaction (child transaction id).
    #[serde(rename = "spentby")]
    pub spent_by: Vec<String>,
}

/// Fee object. Part of `getmempoolentry`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
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

/// Result of JSON-RPC method `getmempoolinfo` with verbose set to `true`.
///
/// > getmempoolinfo
/// >
/// > Returns details on the active state of the TX memory pool.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolInfo {
    /// Current transaction count.
    pub size: i64,
    /// Sum of all virtual transaction sizes as defined in BIP 141.
    ///
    /// Differs from actual serialized size because witness data is discounted.
    pub bytes: i64,
    /// Total memory usage for the mempool.
    pub usage: i64,
    /// Maximum memory usage for the mempool.
    #[serde(rename = "maxmempool")]
    pub max_mempool: i64,
    /// Minimum fee rate in BTC/kB for a transaction to be accepted.
    ///
    /// This is the maximum of `minrelaytxfee` and the minimum mempool fee.
    #[serde(rename = "mempoolminfee")]
    pub mempool_min_fee: f64,
    /// Current minimum relay fee for transactions.
    #[serde(rename = "minrelaytxfee")]
    pub min_relay_tx_fee: f64,
}

/// Result of JSON-RPC method `getrawmempool` with verbose set to `false`.
/// > getrawmempool ( verbose )
/// >
/// > Returns all transaction ids in memory pool as a json array of string transaction ids.
/// >
/// > Hint: use getmempoolentry to fetch a specific transaction from the mempool.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawMempool(pub Vec<String>);

/// Result of JSON-RPC method `getrawmempool` with verbose set to `true`.
///
/// Map of txid to [`MempoolEntry`].
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawMempoolVerbose(pub BTreeMap<String, MempoolEntry>);

/// Result of JSON-RPC method `gettxout`.
///
/// > gettxout "txid" n ( include_mempool )
/// >
/// > Returns details about an unspent transaction output.
/// >
/// > Arguments:
/// > 1. txid               (string, required) The transaction id
/// > 2. n                  (numeric, required) vout number
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxOut {
    /// The hash of the block at the tip of the chain.
    #[serde(rename = "bestblock")]
    pub best_block: String,
    /// The number of confirmations.
    pub confirmations: u32, // TODO: Change this to an i64.
    /// The transaction value in BTC.
    pub value: f64,
    /// The script pubkey.
    #[serde(rename = "scriptPubKey")]
    pub script_pubkey: ScriptPubkey,
    /// Coinbase or not.
    pub coinbase: bool,
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
    /// The serialized hash.
    pub hash_serialized_2: String,
    /// The estimated size of the chainstate on disk.
    pub disk_size: i64,
    /// The total amount.
    pub total_amount: f64,
}

/// Result of JSON-RPC method `pruneblockchain`.
///
/// > pruneblockchain height
/// >
/// > Arguments:
/// > 1. "height"       (numeric, required) The block height to prune up to. May be set to a discrete height, or a unix timestamp
/// >                   to prune blocks whose block time is at least 2 hours older than the provided timestamp.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct PruneBlockchain(
    /// The height of the last block pruned.
    pub i64,
);

/// Result of JSON-RPC method `verifychain`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct VerifyChain(pub bool);

/// Result of JSON-RPC method `verifytxoutproof`.
///
/// > verifytxoutproof "proof"
/// >
/// > Verifies that a proof points to a transaction in a block, returning the transaction it commits to
/// > and throwing an RPC error if the block is not in our best chain
/// >
/// > Arguments:
/// > 1. "proof"    (string, required) The hex-encoded proof generated by gettxoutproof
///
/// Inner field is the txid(s) which the proof commits to, or empty array if the proof can not be validated.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct VerifyTxOutProof(pub Vec<String>);
