// SPDX-License-Identifier: CC0-1.0

//! Types for methods found under the `== Blockchain ==` section of the API docs.
//!
//! These structs model the types returned by the JSON-RPC API but have concrete types
//! and are not specific to a specific version of Bitcoin Core.

use alloc::collections::BTreeMap;

use bitcoin::address::NetworkUnchecked;
use bitcoin::{
    block, Address, Amount, Block, BlockHash, CompactTarget, FeeRate, Network, TxMerkleNode, TxOut,
    Txid, Weight, Work, Wtxid,
};
use serde::{Deserialize, Serialize};

/// Models the result of JSON-RPC method `getbestblockhash`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBestBlockHash(pub BlockHash);

/// Models the result of JSON-RPC method `getblock` with verbosity set to 0.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetBlockVerboseZero(pub Block);

/// Models the result of JSON-RPC method `getblock` with verbosity set to 1.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBlockVerboseOne {
    /// The block hash (same as provided) in RPC call.
    pub hash: BlockHash,
    /// The number of confirmations, or -1 if the block is not on the main chain.
    pub confirmations: i64,
    /// The block size.
    pub size: u32,
    /// The block size excluding witness data.
    pub stripped_size: Option<u32>,
    /// The block weight as defined in BIP-141.
    pub weight: Weight,
    /// The block height or index.
    pub height: u32,
    /// The block version.
    pub version: block::Version,
    /// The merkle root.
    pub merkle_root: String,
    /// The transaction ids.
    pub tx: Vec<Txid>,
    /// The block time expressed in UNIX epoch time.
    pub time: u32,
    /// The median block time expressed in UNIX epoch time.
    pub median_time: Option<u32>,
    /// The nonce.
    pub nonce: u32,
    /// The bits.
    pub bits: CompactTarget,
    /// The difficulty.
    pub difficulty: f64,
    /// Expected number of hashes required to produce the chain up to this block (in hex).
    pub chain_work: Work,
    /// The number of transactions in the block.
    pub n_tx: u32,
    /// The hash of the previous block (if available).
    pub previous_block_hash: Option<BlockHash>,
    /// The hash of the next block (if available).
    pub next_block_hash: Option<BlockHash>,
}

/// Models the result of JSON-RPC method `getblockchaininfo`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBlockchainInfo {
    /// Current network name as defined in BIP70 (main, test, signet, regtest).
    pub chain: Network,
    /// The current number of blocks processed in the server.
    pub blocks: u32,
    /// The current number of headers we have validated.
    pub headers: u32,
    /// The hash of the currently best block.
    pub best_block_hash: BlockHash,
    /// The current difficulty.
    pub difficulty: f64,
    /// Median time for the current best block.
    pub median_time: u32,
    /// Estimate of verification progress (between 0 and 1).
    pub verification_progress: f64,
    /// Estimate of whether this node is in Initial Block Download (IBD) mode.
    pub initial_block_download: bool,
    /// Total amount of work in active chain.
    pub chain_work: Work,
    /// The estimated size of the block and undo files on disk.
    pub size_on_disk: u64,
    /// If the blocks are subject to pruning.
    pub pruned: bool,
    /// Lowest-height complete block stored (only present if pruning is enabled)
    pub prune_height: Option<u32>,
    /// Whether automatic pruning is enabled (only present if pruning is enabled).
    pub automatic_pruning: Option<bool>,
    /// The target size used by pruning (only present if automatic pruning is enabled).
    pub prune_target_size: Option<u32>,
    /// Status of softforks in progress, maps softfork name -> [`Softfork`].
    pub softforks: BTreeMap<String, Softfork>,
    /// Any network and blockchain warnings.
    pub warnings: Vec<String>,
}

/// Status of softfork.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Softfork {
    /// The [`SoftforkType`]: one of "burried", "bip9".
    #[serde(rename = "type")]
    pub type_: SoftforkType,
    /// The status of bip9 softforks (only for "bip9" type).
    pub bip9: Option<Bip9SoftforkInfo>,
    ///  Height of the first block which the rules are or will be enforced (only for "buried" type, or "bip9" type with "active" status).
    pub height: Option<u32>,
    /// `true` if the rules are enforced for the mempool and the next block.
    pub active: bool,
}

/// The softfork type: one of "burried", "bip9".
#[derive(Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SoftforkType {
    /// Softfork is "burried" (as defined in [BIP-90]).
    ///
    /// [BIP-90] <https://github.com/bitcoin/bips/blob/master/bip-0090.mediawiki>
    Buried,
    /// Softfork is "bip9" (see [BIP-9]).
    ///
    /// [BIP-9] <https://github.com/bitcoin/bips/blob/master/bip-0009.mediawiki>
    Bip9,
}

/// Status of BIP-9 softforks.
#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct Bip9SoftforkInfo {
    /// One of "defined", "started", "locked_in", "active", "failed".
    pub status: Bip9SoftforkStatus,
    /// The bit (0-28) in the block version field used to signal this softfork (only for "started" status).
    pub bit: Option<u8>,
    /// The minimum median time past of a block at which the bit gains its meaning.
    pub start_time: u32,
    /// The median time past of a block at which the deployment is considered failed if not yet locked in.
    pub timeout: u32,
    /// Height of the first block to which the status applies.
    pub since: u32,
    /// Numeric statistics about BIP-9 signalling for a softfork (only for "started" status).
    pub statistics: Option<Bip9SoftforkStatistics>,
}

/// BIP-9 softfork status: one of "defined", "started", "locked_in", "active", "failed".
#[derive(Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
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

/// Statistics for a BIP-9 softfork.
#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct Bip9SoftforkStatistics {
    /// The length in blocks of the BIP9 signalling period.
    pub period: u32,
    /// The number of blocks with the version bit set required to activate the feature.
    pub threshold: Option<u32>,
    /// The number of blocks elapsed since the beginning of the current period.
    pub elapsed: u32,
    /// The number of blocks with the version bit set in the current period.
    pub count: u32,
    /// `false` if there are not enough blocks left in this period to pass activation threshold.
    pub possible: Option<bool>,
}

/// Models the result of JSON-RPC method `getblockcount`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetBlockCount(pub u64);

/// Models the result of JSON-RPC method `getblockfilter`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetBlockFilter {
    /// The filter data.
    pub filter: Vec<u8>,
    /// The hex-encoded filter header.
    pub header: bitcoin::bip158::FilterHash,
}

/// Models the result of JSON-RPC method `getblockhash`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetBlockHash(pub BlockHash);

/// Models the result of JSON-RPC method `getblockheader`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetBlockHeader(pub block::Header);

/// Models the result of JSON-RPC method `getblockheader`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBlockHeaderVerbose {
    /// the block hash (same as provided).
    pub hash: BlockHash,
    /// The number of confirmations, or -1 if the block is not on the main chain.
    pub confirmations: i64,
    /// The block height or index.
    pub height: u32,
    /// Block version, now repurposed for soft fork signalling.
    pub version: block::Version,
    /// The root hash of the Merkle tree of transactions in the block.
    pub merkle_root: TxMerkleNode,
    /// The timestamp of the block, as claimed by the miner (seconds since epoch (Jan 1 1970 GMT).
    pub time: u32,
    /// The median block time in seconds since epoch (Jan 1 1970 GMT).
    pub median_time: u32,
    /// The nonce.
    pub nonce: u32,
    /// The target value below which the blockhash must lie.
    pub bits: CompactTarget,
    /// The difficulty.
    pub difficulty: f64,
    /// Expected number of hashes required to produce the current chain.
    pub chain_work: Work,
    /// The number of transactions in the block.
    pub n_tx: u32,
    /// The hash of the previous block (if available).
    pub previous_block_hash: Option<BlockHash>,
    /// The hash of the next block (if available).
    pub next_block_hash: Option<BlockHash>,
}

/// Models the result of JSON-RPC method `getblockstats`.
pub struct GetBlockStats {
    /// Average fee in the block.
    pub average_fee: Amount,
    /// Average feerate.
    pub average_fee_rate: Option<FeeRate>,
    /// Average transaction size.
    pub average_tx_size: u32,
    /// The block hash (to check for potential reorgs).
    pub block_hash: BlockHash,
    /// Feerates at the 10th, 25th, 50th, 75th, and 90th percentile weight unit (in satoshis per virtual byte).
    pub fee_rate_percentiles: Vec<Option<FeeRate>>,
    /// The height of the block.
    pub height: u32,
    /// The number of inputs (excluding coinbase).
    pub inputs: u32,
    /// Maximum fee in the block.
    pub max_fee: Amount,
    /// Maximum feerate (in satoshis per virtual byte).
    pub max_fee_rate: Option<FeeRate>,
    /// Maximum transaction size.
    pub max_tx_size: u32,
    /// Truncated median fee in the block.
    pub median_fee: Amount,
    /// The block median time past.
    pub median_time: u32,
    /// Truncated median transaction size
    pub median_tx_size: u32,
    /// Minimum fee in the block.
    pub minimum_fee: Amount,
    /// Minimum feerate (in satoshis per virtual byte).
    pub minimum_fee_rate: Option<FeeRate>,
    /// Minimum transaction size.
    pub minimum_tx_size: u32,
    /// The number of outputs.
    pub outputs: u32,
    /// The block subsidy.
    pub subsidy: Amount,
    /// Total size of all segwit transactions.
    pub segwit_total_size: u32,
    /// Total weight of all segwit transactions divided by segwit scale factor (4).
    pub segwit_total_weight: Option<Weight>,
    /// The number of segwit transactions.
    pub segwit_txs: u32,
    /// The block time.
    pub time: u32,
    /// Total amount in all outputs (excluding coinbase and thus reward [ie subsidy + totalfee]).
    pub total_out: Amount,
    /// Total size of all non-coinbase transactions.
    pub total_size: u32,
    /// Total weight of all non-coinbase transactions divided by segwit scale factor (4).
    pub total_weight: Option<Weight>,
    /// The fee total.
    pub total_fee: Amount,
    /// The number of transactions (excluding coinbase).
    pub txs: u32,
    /// The increase/decrease in the number of unspent outputs.
    pub utxo_increase: i32,
    /// The increase/decrease in size for the utxo index (not discounting op_return and similar).
    pub utxo_size_increase: i32,
}

/// Models the result of JSON-RPC method `getchaintips`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetChainTips(pub Vec<ChainTips>);

/// An individual list item from the result of JSON-RPC method `getchaintips`.
#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct ChainTips {
    /// Height of the chain tip.
    pub height: u32,
    /// Block hash of the tip.
    pub hash: BlockHash,
    /// Zero for main chain.
    pub branch_length: u32,
    /// "active" for the main chain.
    pub status: ChainTipsStatus,
}

/// The `status` field from an individual list item from the result of JSON-RPC method `getchaintips`.
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

/// Models the result of JSON-RPC method `getchaintxstats`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetChainTxStats {
    /// The timestamp for the final block in the window in UNIX format.
    pub time: u32,
    /// The total number of transactions in the chain up to that point.
    pub tx_count: u32,
    /// The hash of the final block in the window.
    pub window_final_block_hash: BlockHash,
    /// Size of the window in number of blocks.
    pub window_block_count: u32,
    /// The number of transactions in the window. Only returned if "window_block_count" is > 0.
    pub window_tx_count: Option<u32>,
    /// The elapsed time in the window in seconds. Only returned if "window_block_count" is > 0.
    pub window_interval: Option<u32>,
    /// The average rate of transactions per second in the window. Only returned if "window_interval" is > 0.
    pub tx_rate: Option<u32>,
}

/// Models the result of JSON-RPC method `getdifficulty`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetDifficulty(pub f64);

/// Models the result of JSON-RPC method `getmempoolancestors` with verbose set to false.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetMempoolAncestors(pub Vec<Txid>);

/// Models the result of JSON-RPC method `getmempoolancestors` with verbose set to true.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetMempoolAncestorsVerbose(pub BTreeMap<Txid, MempoolEntry>);

/// Models the result of JSON-RPC method `getmempoolancestors` with verbose set to false.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetMempoolDescendants(pub Vec<Txid>);

/// Models the result of JSON-RPC method `getmempooldescendants` with verbose set to true.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetMempoolDescendantsVerbose(pub BTreeMap<Txid, MempoolEntry>);

/// Models the result of JSON-RPC method `getmempoolentry`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetMempoolEntry(pub MempoolEntry);

/// A relative (ancestor or descendant) transaction of a transaction in the mempool.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MempoolEntry {
    /// Virtual transaction size as defined in BIP 141.
    ///
    /// This is different from actual serialized size for witness transactions as witness data is discounted.
    ///
    /// This was deprecated with Bitcoin Core v0.19 and hence will be `None` for v0.19 and later.
    pub size: Option<u32>,
    /// Transaction weight as defined in BIP 141
    ///
    /// This  was introduced with Bitcoin Core v0.19 and will hence be `None` for previous
    /// versions.
    pub weight: Option<u32>,
    /// Local time transaction entered pool in seconds since 1 Jan 1970 GMT.
    pub time: u32,
    /// Block height when transaction entered pool.
    pub height: u32,
    /// Number of in-mempool descendant transactions (including this one).
    pub descendant_count: u32,
    /// Virtual transaction size of in-mempool descendants (including this one).
    pub descendant_size: u32,
    /// Number of in-mempool ancestor transactions (including this one).
    pub ancestor_count: u32,
    /// Virtual transaction size of in-mempool ancestors (including this one).
    pub ancestor_size: u32,
    /// Hash of serialized transaction, including witness data.
    pub wtxid: Wtxid,
    /// (No docs in Core v17.)
    pub fees: MempoolEntryFees,
    /// Unconfirmed transactions used as inputs for this transaction (parent transaction id).
    pub depends: Vec<Txid>,
    /// Unconfirmed transactions spending outputs from this transaction (child transaction id).
    pub spent_by: Vec<Txid>,
}

/// (No docs in Core v17.)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MempoolEntryFees {
    /// Transaction fee in BTC.
    pub base: Amount,
    /// Transaction fee with fee deltas used for mining priority.
    pub modified: Amount,
    /// Modified fees (see above) of in-mempool ancestors (including this one).
    pub ancestor: Amount,
    /// Modified fees (see above) of in-mempool descendants (including this one).
    pub descendant: Amount,
}

/// Models the result of JSON-RPC method `getmempoolinfo` with verbose set to true.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetMempoolInfo {
    /// Current transaction count.
    pub size: u32,
    /// Sum of all virtual transaction sizes as defined in BIP 141.
    ///
    /// Differs from actual serialized size because witness data is discounted.
    pub bytes: u32,
    /// Total memory usage for the mempool.
    pub usage: u32,
    /// Maximum memory usage for the mempool.
    pub max_mempool: u32,
    /// Minimum fee rate in BTC/kB for a transaction to be accepted.
    ///
    /// This is the maximum of `minrelaytxfee` and the minimum mempool fee.
    pub mempool_min_fee: Option<FeeRate>,
    /// Current minimum relay fee for transactions.
    pub min_relay_tx_fee: Option<FeeRate>,
}

/// Models the result of JSON-RPC method `getrawmempool` with verbose set to false.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetRawMempool(pub Vec<Txid>);

/// Models the result of JSON-RPC method `getrawmempool` with verbose set to true.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetRawMempoolVerbose(pub BTreeMap<Txid, MempoolEntry>);

/// Models the result of JSON-RPC method `gettxout`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetTxOut {
    /// The hash of the block at the tip of the chain.
    pub best_block: BlockHash,
    /// The number of confirmations (signed to match other types with the same field name).
    pub confirmations: u32,
    /// The returned `TxOut` (strongly typed).
    pub tx_out: TxOut,
    /// Address that `tx_out` spends to.
    ///
    /// Only if a well-defined address exists.
    pub address: Option<Address<NetworkUnchecked>>,
    /// Coinbase or not.
    pub coinbase: bool,
}

/// Models the result of JSON-RPC method `gettxoutsetinfo`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetTxOutSetInfo {
    /// The current block height (index).
    pub height: u32,
    /// The hash of the block at the tip of the chain.
    pub best_block: BlockHash,
    /// The number of transactions with unspent outputs.
    pub transactions: u32,
    /// The number of unspent transaction outputs.
    pub tx_outs: u32,
    /// A meaningless metric for UTXO set size.
    pub bogo_size: u32,
    /// The serialized hash.
    ///
    /// This was removed in Bitcoin Core v26, and hence will be `None` for v26 and later.
    pub hash_serialized_2: Option<String>, // FIXME: What sort of hash is this?
    /// The estimated size of the chainstate on disk.
    pub disk_size: u32,
    /// The total amount.
    pub total_amount: Amount,
}

/// Models the result of JSON-RPC method `verifytxoutproof`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct VerifyTxOutProof(pub Vec<Txid>);
