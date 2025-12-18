// SPDX-License-Identifier: CC0-1.0

//! Types for methods that are `== Hidden ==` and not in the API docs of Bitcoin Core.
//!
//! These structs model the types returned by the JSON-RPC API but have concrete types
//! and are not specific to a specific version of Bitcoin Core.

use bitcoin::{FeeRate, Transaction, Txid, Wtxid};
use serde::{Deserialize, Serialize};

/// Models the result of JSON-RPC method `addconnection`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct AddConnection {
    /// The address of the newly added connection.
    pub address: String,
    /// Type of connection.
    pub connection_type: String,
}

/// Models the result of JSON-RPC method `estimaterawfee`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EstimateRawFee {
    /// Estimate for short time horizon.
    pub short: Option<RawFeeDetail>,
    /// Estimate for medium time horizon.
    pub medium: Option<RawFeeDetail>,
    /// Estimate for long time horizon.
    pub long: RawFeeDetail,
}

/// Estimate for a time horizon. Part of `estimaterawfee`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RawFeeDetail {
    /// Estimate fee rate in BTC/kB.
    pub fee_rate: Option<FeeRate>,
    /// Exponential decay (per block) for historical moving average of confirmation data.
    pub decay: f64,
    /// The resolution of confirmation targets at this time horizon.
    pub scale: u32,
    /// Information about the lowest range of feerates to succeed in meeting the threshold.
    pub pass: Option<RawFeeRange>,
    /// Information about the highest range of feerates to fail to meet the threshold.
    pub fail: Option<RawFeeRange>,
    /// Errors encountered during processing.
    pub errors: Option<Vec<String>>,
}

/// Information about a feerate range. Part of `estimaterawfee`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RawFeeRange {
    /// Start of feerate range.
    pub start_range: Option<FeeRate>,
    /// End of feerate range.
    pub end_range: Option<FeeRate>,
    /// Number of txs over history horizon in the feerate range that were confirmed within target.
    pub within_target: f64,
    /// Number of txs over history horizon in the feerate range that were confirmed at any point.
    pub total_confirmed: f64,
    /// Current number of txs in mempool in the feerate range unconfirmed for at least target blocks.
    pub in_mempool: f64,
    /// Number of txs over history horizon in the feerate range that left mempool unconfirmed after target.
    pub left_mempool: f64,
}

/// Models the result of JSON-RPC method `getorphantxs` with verbosity level 0.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetOrphanTxs(pub Vec<Txid>);

/// Models the result of JSON-RPC method `getorphantxs` with verbosity level 1.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetOrphanTxsVerboseOne(pub Vec<GetOrphanTxsVerboseOneEntry>);

/// Models an entry of the result list of JSON-RPC method `getorphantxs` with verbosity level 1.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetOrphanTxsVerboseOneEntry {
    /// The transaction hash in hex
    pub txid: Txid,
    /// The transaction witness hash in hex
    pub wtxid: Wtxid,
    /// The serialized transaction size in bytes
    pub bytes: u64,
    /// The virtual transaction size as defined in BIP 141. This is different from actual serialized size for witness transactions as witness data is discounted.
    pub vsize: u64,
    /// The transaction weight as defined in BIP 141.
    pub weight: u64,
    /// The entry time into the orphanage expressed in UNIX epoch time
    /// Only present in v29.
    pub entry_time: Option<u32>,
    /// The orphan expiration time expressed in UNIX epoch time
    /// Only present in v29.
    pub expiration_time: Option<u32>,
    /// List of peer ids that we store this transaction for.
    pub from: Vec<u64>,
}

/// Models the result of JSON-RPC method `getorphantxs` with verbosity level 2.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetOrphanTxsVerboseTwo(pub Vec<GetOrphanTxsVerboseTwoEntry>);

/// Models an entry of the result list of JSON-RPC method `getorphantxs` with verbosity level 2.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetOrphanTxsVerboseTwoEntry {
    /// The transaction hash in hex
    pub txid: Txid,
    /// The transaction witness hash in hex
    pub wtxid: Wtxid,
    /// The serialized transaction size in bytes
    pub bytes: u64,
    /// The virtual transaction size as defined in BIP 141. This is different from actual serialized size for witness transactions as witness data is discounted.
    pub vsize: u64,
    /// The transaction weight as defined in BIP 141.
    pub weight: u64,
    /// The entry time into the orphanage expressed in UNIX epoch time
    /// Only present in v29.
    pub entry_time: Option<u32>,
    /// The orphan expiration time expressed in UNIX epoch time
    /// Only present in v29.
    pub expiration_time: Option<u32>,
    /// List of peer ids that we store this transaction for.
    pub from: Vec<u64>,
    /// The orphan transaction.
    pub transaction: Transaction,
}
