// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v25` - blockchain.
//!
//! Types for methods found under the `== Blockchain ==` section of the API docs.

mod into;

use alloc::collections::BTreeMap;

use serde::{Deserialize, Serialize};

pub use super::{GetBlockStatsError, ScanTxOutSetError};

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
#[serde(deny_unknown_fields)]
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
    /// The increase/decrease in the number of unspent outputs, not counting unspendables.
    /// v25 and later only.
    pub utxo_increase_actual: Option<i32>,
    /// The increase/decrease in size for the utxo index, not counting unspendables.
    /// v25 and later only.
    #[serde(rename = "utxo_size_inc_actual")]
    pub utxo_size_increase_actual: Option<i32>,
}

/// Result of JSON-RPC method `getblockchaininfo`.
///
/// > getblockchaininfo
/// >
/// > Returns an object containing various state info regarding blockchain processing.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
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
    pub warnings: String,
}

/// Status of softfork.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Softfork {
    /// The [`SoftforkType`]: one of "buried", "bip9".
    #[serde(rename = "type")]
    pub type_: SoftforkType,
    /// The status of bip9 softforks (only for "bip9" type).
    pub bip9: Option<Bip9SoftforkInfo>,
    ///  Height of the first block which the rules are or will be enforced (only for "buried" type, or "bip9" type with "active" status).
    pub height: Option<i64>,
    /// `true` if the rules are enforced for the mempool and the next block.
    pub active: bool,
}

/// The softfork type: one of "buried", "bip9".
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

/// Status of BIP-9 softforks.
#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bip9SoftforkInfo {
    /// One of "defined", "started", "locked_in", "active", "failed".
    pub status: Bip9SoftforkStatus,
    /// The bit (0-28) in the block version field used to signal this softfork (only for "started" status).
    pub bit: Option<u8>,
    /// The minimum median time past of a block at which the bit gains its meaning.
    pub start_time: i64,
    /// The median time past of a block at which the deployment is considered failed if not yet locked in.
    pub timeout: i64,
    /// Height of the first block to which the status applies.
    pub since: i64,
    /// Numeric statistics about BIP-9 signalling for a softfork (only for "started" status).
    pub statistics: Option<Bip9SoftforkStatistics>,
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

/// Statistics for a BIP-9 softfork.
#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bip9SoftforkStatistics {
    /// The length in blocks of the BIP9 signalling period.
    pub period: i64,
    /// The number of blocks with the version bit set required to activate the feature.
    pub threshold: Option<i64>,
    /// The number of blocks elapsed since the beginning of the current period.
    pub elapsed: i64,
    /// The number of blocks with the version bit set in the current period.
    pub count: i64,
    /// `false` if there are not enough blocks left in this period to pass activation threshold.
    pub possible: Option<bool>,
}

/// Result of JSON-RPC method `getblockfilter`.
///
/// > getblockfilter "blockhash" ( "filtertype" )
/// >
/// > Retrieve a BIP 157 content filter for a particular block.
/// >
/// > Arguments:
/// > 1. blockhash     (string, required) The hash of the block
/// > 2. filtertype    (string, optional, default=basic) The type name of the filter
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GetBlockFilter {
    /// The hex-encoded filter data.
    pub filter: String,
    /// The hex-encoded filter header.
    pub header: String,
}

/// Result of JSON-RPC method `getchaintxstats`.
///
/// > getchaintxstats ( nblocks blockhash )
/// >
/// > Compute statistics about the total number and rate of transactions in the chain.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GetChainTxStats {
    /// The timestamp for the final block in the window in UNIX format.
    pub time: i64,
    /// The total number of transactions in the chain up to that point.
    #[serde(rename = "txcount")]
    pub tx_count: i64,
    /// The hash of the final block in the window.
    pub window_final_block_hash: String,
    /// The height of the final block in the window.
    pub window_final_block_height: i64,
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

/// Result of JSON-RPC method `getmempoolancestors` with verbose set to `false`.
///
/// > getmempoolancestors txid (verbose)
/// >
/// > If txid is in the mempool, returns all in-mempool ancestors.
/// >
/// > Arguments:
/// > 1. "txid"                 (string, required) The transaction id (must be in mempool)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GetMempoolAncestors(pub Vec<String>);

/// Result of JSON-RPC method `getmempoolancestors` with verbose set to true.
///
/// Map of txid to `MempoolEntry` i.e., an ancestor.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
pub struct GetMempoolDescendants(pub Vec<String>);

/// Result of JSON-RPC method `getmempooldescendants` with verbose set to true.
///
/// Map of txid to [`MempoolEntry`] i.e., a descendant.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
pub struct GetMempoolEntry(pub MempoolEntry);

/// A relative (ancestor or descendant) transaction of a transaction in the mempool.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MempoolEntry {
    /// Virtual transaction size as defined in BIP 141.
    ///
    /// This is different from actual serialized size for witness transactions as witness data is discounted.  v0.19 and later only.
    pub vsize: i64,
    /// DEPRECATED: same as vsize. Only returned if bitcoind is started with -deprecatedrpc=size
    /// size will be completely removed in v0.20.
    pub size: Option<i64>,
    /// Transaction weight as defined in BIP 141.
    pub weight: i64,
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
    /// Fee object which contains the base fee, modified fee (with fee deltas), and
    /// ancestor/descendant fee totals all in BTC.
    pub fees: MempoolEntryFees,
    /// Unconfirmed transactions used as inputs for this transaction (parent transaction id).
    pub depends: Vec<String>,
    /// Unconfirmed transactions spending outputs from this transaction (child transaction id).
    #[serde(rename = "spentby")]
    pub spent_by: Vec<String>,
    /// Whether this transaction could be replaced due to BIP125 (replace-by-fee)
    #[serde(rename = "bip125-replaceable")]
    pub bip125_replaceable: bool,
}

/// The `fees` field from the result of JSON-RPC method `getmempoolentry`.
///
/// Contains the base fee, modified fee (with fee deltas), and ancestor/descendant fee totals,
/// all in BTC.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
pub struct GetMempoolInfo {
    /// True if the mempool is fully loaded. v0.19 and later only.
    pub loaded: bool,
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

/// Result of JSON-RPC method `scantxoutset`.
///
/// > scantxoutset "action" ( [scanobjects,...] )
/// >
/// > Arguments:
/// > 1. action                        (string, required) The action to execute
/// > 2. scanobjects                   (json array, required) Array of scan objects
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ScanTxOutSetStart {
    /// Whether the scan is completed
    pub success: bool,
    /// The number of unspent transaction outputs scanned
    pub txouts: u64,
    /// The current block height (index)
    pub height: u64,
    /// The hash of the block at the tip of the chain
    #[serde(rename = "bestblock")]
    pub best_block: String,
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
    pub script_pubkey: String,
    /// An output descriptor
    #[serde(rename = "desc")]
    pub descriptor: String,
    /// The total amount in BTC of unspent output
    pub amount: f64,
    /// Whether this is a coinbase output
    pub coinbase: bool,
    /// Height of the unspent transaction output
    pub height: u64,
}
