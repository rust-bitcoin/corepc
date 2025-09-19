// SPDX-License-Identifier: CC0-1.0

//! Types for methods found under the `== Blockchain ==` section of the API docs.
//!
//! These structs model the types returned by the JSON-RPC API but have concrete types
//! and are not specific to a specific version of Bitcoin Core.

use alloc::collections::BTreeMap;

use bitcoin::address::NetworkUnchecked;
use bitcoin::hashes::sha256;
use bitcoin::{
    block, Address, Amount, Block, BlockHash, CompactTarget, FeeRate, Network, OutPoint, ScriptBuf,
    Target, TxMerkleNode, TxOut, Txid, Weight, Work, Wtxid,
};
use serde::{Deserialize, Serialize};

use crate::ScriptPubkey;

/// Models the result of JSON-RPC method `dumptxoutset`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DumpTxOutSet {
    /// The number of coins written in the snapshot.
    pub coins_written: Amount,
    /// The hash of the base of the snapshot.
    pub base_hash: BlockHash,
    /// The height of the base of the snapshot.
    pub base_height: u32,
    /// The absolute path that the snapshot was written to.
    pub path: String,
    /// The hash of the UTXO set contents.
    pub tx_out_set_hash: sha256::Hash,
    /// The number of transactions in the chain up to and including the base block.
    pub n_chain_tx: u32,
}

/// Models the result of JSON-RPC method `getbestblockhash`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBestBlockHash(pub BlockHash);

/// Models the result of JSON-RPC method `getblock` with verbosity set to 0.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockVerboseZero(pub Block);

/// Models the result of JSON-RPC method `getblock` with verbosity set to 1.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
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
    /// The difficulty target.
    pub target: Option<Target>, // Only from v29 onwards
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
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockchainInfo {
    /// Current network name as defined in BIP70 (main, test, signet, regtest).
    pub chain: Network,
    /// The current number of blocks processed in the server.
    pub blocks: u32,
    /// The current number of headers we have validated.
    pub headers: u32,
    /// The hash of the currently best block.
    pub best_block_hash: BlockHash,
    /// The compact representation of the block difficulty target.
    pub bits: Option<CompactTarget>, // Only from v29 onwards
    /// The difficulty target.
    pub target: Option<Target>, // Only from v29 onwards
    /// The current difficulty.
    pub difficulty: f64,
    /// The block time expressed in UNIX epoch time. v23 and later only.
    pub time: Option<u32>,
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
    /// Status of softforks in progress, maps softfork name -> [`Softfork`] (empty from v29 onwards).
    pub softforks: BTreeMap<String, Softfork>,
    /// The block challenge (aka. block script)
    pub signet_challenge: Option<ScriptBuf>, // Only from v29 onwards
    /// Any network and blockchain warnings.
    pub warnings: Vec<String>,
}

/// Softfork status. Part of `getblockchaininfo`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Softfork {
    /// The [`SoftforkType`]: one of "buried", "bip9".
    #[serde(rename = "type")]
    pub type_: SoftforkType,
    /// The status of bip9 softforks (only for "bip9" type).
    pub bip9: Option<Bip9SoftforkInfo>,
    ///  Height of the first block which the rules are or will be enforced (only for "buried" type, or "bip9" type with "active" status).
    pub height: Option<u32>,
    /// `true` if the rules are enforced for the mempool and the next block.
    pub active: bool,
}

/// The softfork type. Part of `getblockchaininfo`.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SoftforkType {
    /// Softfork is "buried" (as defined in [BIP-90]).
    ///
    /// [BIP-90] <https://github.com/bitcoin/bips/blob/master/bip-0090.mediawiki>
    Buried,
    /// Softfork is "bip9" (see [BIP-9]).
    ///
    /// [BIP-9] <https://github.com/bitcoin/bips/blob/master/bip-0009.mediawiki>
    Bip9,
}

/// BIP-9 softfork info. Part of `getblockchaininfo`.
#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
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
    /// Minimum height of blocks for which the rules may be enforced. v0.21 and later only.
    pub min_activation_height: Option<i64>,
    /// Numeric statistics about BIP-9 signalling for a softfork (only for "started" status).
    pub statistics: Option<Bip9SoftforkStatistics>,
}

/// BIP-9 softfork status. Part of `getblockchaininfo`.
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

/// BIP-9 softfork statistics. Part of `getblockchaininfo`.
#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
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
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockCount(pub u64);

/// Models the result of JSON-RPC method `getblockfilter`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockFilter {
    /// The filter data.
    pub filter: Vec<u8>,
    /// The hex-encoded filter header.
    pub header: bitcoin::bip158::FilterHash,
}

/// Models the result of JSON-RPC method `getblockhash`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockHash(pub BlockHash);

/// Models the result of JSON-RPC method `getblockheader`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockHeader(pub block::Header);

/// Models the result of JSON-RPC method `getblockheader`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
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
    /// The difficulty target.
    pub target: Option<Target>, // Only from v29 onwards
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
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
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
    /// The increase/decrease in the number of unspent outputs, not counting unspendables.
    /// v25 and later only.
    pub utxo_increase_actual: Option<i32>,
    /// The increase/decrease in size for the utxo index, not counting unspendables.
    /// v25 and later only.
    pub utxo_size_increase_actual: Option<i32>,
}

/// Models the result of JSON-RPC method `getchainstates`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetChainStates {
    /// The number of headers seen so far.
    pub headers: u32,
    /// List of the chainstates ordered by work, with the most-work (active) chainstate last.
    pub chain_states: Vec<ChainState>,
}

/// A single chainstate. Part of `getchainstates`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ChainState {
    /// Number of blocks in this chainstate.
    pub blocks: u32,
    /// Blockhash of the tip.
    pub best_block_hash: BlockHash,
    /// nBits: compact representation of the block difficulty target.
    pub bits: Option<CompactTarget>, // v29 and later only.
    /// The difficulty target.
    pub target: Option<Target>, // v29 and later only.
    /// Difficulty of the tip.
    pub difficulty: f64,
    /// Progress towards the network tip (0..=1).
    pub verification_progress: f64,
    /// The base block of the snapshot this chainstate is based on, if any.
    pub snapshot_block_hash: Option<BlockHash>,
    /// Size of the coinsdb cache.
    pub coins_db_cache_bytes: u64,
    /// Size of the coinstip cache.
    pub coins_tip_cache_bytes: u64,
    /// Whether the chainstate is fully validated.
    pub validated: bool,
}

/// Models the result of JSON-RPC method `getchaintips`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetChainTips(pub Vec<ChainTips>);

/// An individual list item from the result of JSON-RPC method `getchaintips`.
#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
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

/// Chain tips status. Part of `getchaintips`.
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
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetChainTxStats {
    /// The timestamp for the final block in the window in UNIX format.
    pub time: u32,
    /// The total number of transactions in the chain up to that point.
    pub tx_count: u32,
    /// The hash of the final block in the window.
    pub window_final_block_hash: BlockHash,
    /// The height of the final block in the window. v0.19 and later only.
    pub window_final_block_height: Option<u32>,
    /// Size of the window in number of blocks.
    pub window_block_count: u32,
    /// The number of transactions in the window. Only returned if "window_block_count" is > 0.
    pub window_tx_count: Option<u32>,
    /// The elapsed time in the window in seconds. Only returned if "window_block_count" is > 0.
    pub window_interval: Option<u32>,
    /// The average rate of transactions per second in the window. Only returned if "window_interval" is > 0.
    pub tx_rate: Option<u32>,
}

/// Models the result of JSON-RPC method `getdeploymentinfo`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDeploymentInfo {
    /// Requested block hash (or tip).
    pub hash: BlockHash,
    /// Requested block height (or tip).
    pub height: u32,
    /// Deployments info, keyed by deployment name.
    pub deployments: std::collections::BTreeMap<String, DeploymentInfo>,
}

/// Deployment info. Part of `getdeploymentinfo`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DeploymentInfo {
    /// One of "buried", "bip9".
    pub deployment_type: String,
    /// Height of the first block which the rules are or will be enforced (only for "buried" type, or "bip9" type with "active" status).
    pub height: Option<u32>,
    /// True if the rules are enforced for the mempool and the next block.
    pub active: bool,
    /// Status of bip9 softforks (only for "bip9" type).
    pub bip9: Option<Bip9Info>,
}

/// Status of bip9 softforks. Part of `getdeploymentinfo`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Bip9Info {
    /// The bit (0-28) in the block version field used to signal this softfork (only for "started" and "locked_in" status).
    pub bit: Option<u8>,
    /// The minimum median time past of a block at which the bit gains its meaning.
    pub start_time: i64,
    /// The median time past of a block at which the deployment is considered failed if not yet locked in.
    pub timeout: i64,
    /// Minimum height of blocks for which the rules may be enforced.
    pub min_activation_height: u32,
    /// Status of deployment at specified block (one of "defined", "started", "locked_in", "active", "failed").
    pub status: String,
    /// Height of the first block to which the status applies.
    pub since: u32,
    /// Status of deployment at the next block.
    pub status_next: String,
    /// Numeric statistics about signalling for a softfork (only for "started" and "locked_in" status).
    pub statistics: Option<Bip9Statistics>,
    /// Indicates blocks that signalled with a # and blocks that did not with a -.
    pub signalling: Option<String>,
}

/// Numeric statistics about signalling for a softfork. Part of `getdeploymentinfo`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Bip9Statistics {
    /// The length in blocks of the signalling period.
    pub period: u32,
    /// The number of blocks with the version bit set required to activate the feature (only for "started" status).
    pub threshold: Option<u32>,
    /// The number of blocks elapsed since the beginning of the current period.
    pub elapsed: u32,
    /// The number of blocks with the version bit set in the current period.
    pub count: u32,
    /// Returns false if there are not enough blocks left in this period to pass activation threshold (only for "started" status).
    pub possible: Option<bool>,
}

/// Models the result of the JSON-RPC method `getdescriptoractivity`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDescriptorActivity {
    /// A list of activity events related to the descriptors.
    pub activity: Vec<ActivityEntry>,
}

/// A spend or receive activity entry. Part of `getdescriptoractivity`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum ActivityEntry {
    /// The spend activity using `model::SpendActivity`.
    Spend(SpendActivity),
    /// The receive activity using `model::ReceiveActivity`.
    Receive(ReceiveActivity),
}

/// Models a 'spend' activity event. Part of `getdescriptoractivity`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SpendActivity {
    /// The total amount of the spent output.
    pub amount: Amount,
    /// The blockhash (omitted if unconfirmed).
    pub block_hash: Option<BlockHash>,
    /// Height of the spend (omitted if unconfirmed).
    pub height: Option<u32>,
    /// The txid of the spending transaction.
    pub spend_txid: Txid,
    /// The vout of the spend.
    pub spend_vout: u32,
    /// The txid of the prevout.
    pub prevout_txid: Txid,
    /// The vout of the prevout.
    pub prevout_vout: u32,
    /// The prev scriptPubKey.
    pub prevout_spk: ScriptPubkey,
}

/// Models a 'receive' activity event. Part of `getdescriptoractivity`
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ReceiveActivity {
    /// The total amount in BTC of the new output.
    pub amount: Amount,
    /// The block that this receive is in (omitted if unconfirmed).
    pub block_hash: Option<BlockHash>,
    /// The height of the receive (omitted if unconfirmed).
    pub height: Option<u32>,
    /// The txid of the receiving transaction.
    pub txid: Txid,
    /// The vout of the receiving output.
    pub vout: u32,
    /// The ScriptPubKey.
    pub output_spk: ScriptPubkey,
}

/// Models the result of JSON-RPC method `getdifficulty`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDifficulty(pub f64);

/// Models the result of JSON-RPC method `getmempoolancestors` with verbose set to false.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolAncestors(pub Vec<Txid>);

/// Models the result of JSON-RPC method `getmempoolancestors` with verbose set to true.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolAncestorsVerbose(pub BTreeMap<Txid, MempoolEntry>);

/// Models the result of JSON-RPC method `getmempooldescendants` with verbose set to false.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolDescendants(pub Vec<Txid>);

/// Models the result of JSON-RPC method `getmempooldescendants` with verbose set to true.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolDescendantsVerbose(pub BTreeMap<Txid, MempoolEntry>);

/// Models the result of JSON-RPC method `getmempoolentry`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolEntry(pub MempoolEntry);

/// Mempool data. Part of `getmempoolentry`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct MempoolEntry {
    /// Virtual transaction size as defined in BIP 141. v0.19 and later only.
    ///
    /// This is different from actual serialized size for witness transactions as witness data is discounted.
    pub vsize: Option<u32>,
    /// Same as vsize. This was deprecated with Bitcoin Core v0.19 and hence
    /// will be `None` for v0.19 and later.
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
    /// (No docs in Core v0.17). Part of `getmempoolentry`.
    pub fees: MempoolEntryFees,
    /// Unconfirmed transactions used as inputs for this transaction (parent transaction id).
    pub depends: Vec<Txid>,
    /// Unconfirmed transactions spending outputs from this transaction (child transaction id).
    pub spent_by: Vec<Txid>,
    /// Whether this transaction could be replaced due to BIP125 (replace-by-fee)
    pub bip125_replaceable: Option<bool>,
    /// Whether this transaction is currently unbroadcast (initial broadcast not yet acknowledged by
    /// any peers). v0.21 and later only.
    pub unbroadcast: Option<bool>,
}

/// Fee object. Part of `getmempoolentry`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
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
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolInfo {
    /// True if the mempool is fully loaded. v0.19 and later only.
    pub loaded: Option<bool>,
    /// Current transaction count.
    pub size: u32,
    /// Sum of all virtual transaction sizes as defined in BIP 141.
    ///
    /// Differs from actual serialized size because witness data is discounted.
    pub bytes: u32,
    /// Total memory usage for the mempool.
    pub usage: u32,
    /// Total fees for the mempool in BTC, ignoring modified fees through prioritisetransaction. v23
    /// and later only.
    pub total_fee: Option<f64>,
    /// Maximum memory usage for the mempool.
    pub max_mempool: u32,
    /// Minimum fee rate in BTC/kB for a transaction to be accepted.
    ///
    /// This is the maximum of `minrelaytxfee` and the minimum mempool fee.
    pub mempool_min_fee: Option<FeeRate>,
    /// Current minimum relay fee for transactions.
    pub min_relay_tx_fee: Option<FeeRate>,
    /// Minimum fee rate increment for mempool limiting or replacement in BTC/kvB. v24 and later only.
    pub incremental_relay_fee: Option<FeeRate>,
    ///  Current number of transactions that haven't passed initial broadcast yet. v21 and later only.
    pub unbroadcast_count: Option<u32>,
    /// True if the mempool accepts RBF without replaceability signaling inspection. v24 and later
    /// only.
    pub full_rbf: Option<bool>,
}

/// Models the result of JSON-RPC method `getrawmempool` with verbose set to false.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawMempool(pub Vec<Txid>);

/// Models the result of JSON-RPC method `getrawmempool` with verbose set to true.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawMempoolVerbose(pub BTreeMap<Txid, MempoolEntry>);

/// Models the result of JSON-RPC method `gettxout`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
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
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
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
    /// The serialized hash (only present if 'hash_serialized_3' hash_type is chosen).
    /// v26 and later only.
    pub hash_serialized_3: Option<String>,
    /// The estimated size of the chainstate on disk.
    pub disk_size: u32,
    /// The total amount.
    pub total_amount: Amount,
}

/// Models the result of JSON-RPC method `gettxspendingprevout`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxSpendingPrevout(pub Vec<GetTxSpendingPrevoutItem>);

/// A transaction item. Part of `gettxspendingprevout`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxSpendingPrevoutItem {
    /// The outpoint containing the transaction id and vout value of the checked output.
    pub outpoint: OutPoint,
    /// The transaction id of the mempool transaction spending this output (omitted if unspent).
    pub spending_txid: Option<Txid>,
}

/// Models the result of JSON-RPC method `loadtxoutset`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct LoadTxOutSet {
    /// The number of coins loaded from the snapshot.
    pub coins_loaded: Amount,
    /// The hash of the base of the snapshot.
    pub tip_hash: BlockHash,
    /// The height of the base of the snapshot.
    pub base_height: u32,
    /// The absolute path that the snapshot was loaded from.
    pub path: String,
}

/// Models the result of the JSON-RPC method `scanblocks` whan `action = start`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ScanBlocksStart {
    /// The height we started the scan from
    pub from_height: u32,
    /// The height we ended the scan at
    pub to_height: u32,
    /// Blocks that may have matched a scanobject
    pub relevant_blocks: Vec<BlockHash>,
}

/// Models the result of JSON-RPC method `verifytxoutproof`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct VerifyTxOutProof(pub Vec<Txid>);
