// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v25` - blockchain.
//!
//! Types for methods found under the `== Blockchain ==` section of the API docs.

mod error;
mod into;

use serde::{Deserialize, Serialize};

pub use self::error::ScanBlocksStartError;
pub use super::GetBlockStatsError;

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
    /// The increase/decrease in the number of unspent outputs, not counting unspendables.
    /// v25 and later only.
    pub utxo_increase_actual: Option<i32>,
    /// The increase/decrease in size for the utxo index, not counting unspendables.
    /// v25 and later only.
    #[serde(rename = "utxo_size_inc_actual")]
    pub utxo_size_increase_actual: Option<i32>,
}

/// Result of JSON-RPC method `scanblocks` with action "abort".
///
/// > scanblocks "abort"
/// >
/// > Aborts the current scan and returns whether an abort was successful.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ScanBlocksAbort(pub bool);

/// Result of JSON-RPC method `scanblocks` with action "start".
///
/// > scanblocks "start" [scanobjects,...] ( start_height stop_height "filtertype" "options" )
/// >
/// > Arguments:
/// > 1. scanobjects                            (json array, required) Array of scan objects
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ScanBlocksStart {
    /// The height we started the scan from
    pub from_height: i64,
    /// The height we ended the scan at
    pub to_height: i64,
    /// Blocks that may have matched a scanobject
    pub relevant_blocks: Vec<String>,
}

/// Result of JSON-RPC method `scanblocks` with action "status".
///
/// > scanblocks "status"
/// >
/// > Returns progress report (in %) of the current scan.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ScanBlocksStatus {
    /// Approximate percent complete
    pub progress: f64,
    /// Height of the block currently being scanned
    pub current_height: i64,
}
