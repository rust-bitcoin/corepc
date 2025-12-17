// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v0.17` - hidden.
//!
//! Types for methods that are excluded from the API docs by default.

mod error;
mod into;

use serde::{Deserialize, Serialize};

pub use self::error::{
    EstimateRawFeeError, WaitForBlockError, WaitForBlockHeightError, WaitForNewBlockError,
};

/// Result of JSON-RPC method `estimaterawfee`.
///
/// > estimaterawfee conf_target (threshold)
/// >
/// > WARNING: This interface is unstable and may disappear or change!
/// >
/// > WARNING: This is an advanced API call that is tightly coupled to the specific
/// >          implementation of fee estimation. The parameters it can be called with
/// >          and the results it returns will change if the internal implementation changes.
/// >
/// > Estimates the approximate fee per kilobyte needed for a transaction to begin
/// > confirmation within conf_target blocks if possible. Uses virtual transaction size as
/// > defined in BIP 141 (witness data is discounted).
/// >
/// > Arguments:
/// > 1. conf_target (numeric) Confirmation target in blocks (1 - 1008)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
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
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct RawFeeDetail {
    /// Estimate fee rate in BTC/kB.
    #[serde(rename = "feerate")]
    pub fee_rate: Option<f64>,
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
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct RawFeeRange {
    /// Start of feerate range.
    #[serde(rename = "startrange")]
    pub start_range: f64,
    /// End of feerate range.
    #[serde(rename = "endrange")]
    pub end_range: f64,
    /// Number of txs over history horizon in the feerate range that were confirmed within target.
    #[serde(rename = "withintarget")]
    pub within_target: f64,
    /// Number of txs over history horizon in the feerate range that were confirmed at any point.
    #[serde(rename = "totalconfirmed")]
    pub total_confirmed: f64,
    /// Current number of txs in mempool in the feerate range unconfirmed for at least target blocks.
    #[serde(rename = "inmempool")]
    pub in_mempool: f64,
    /// Number of txs over history horizon in the feerate range that left mempool unconfirmed after target.
    #[serde(rename = "leftmempool")]
    pub left_mempool: f64,
}

/// Result of JSON-RPC method `waitforblock`.
///
/// > waitforblock "blockhash" ( timeout )
/// >
/// > Waits for a specific new block and returns useful info about it.
/// >
/// > Returns the current block on timeout or exit.
/// >
/// > Arguments:
/// > 1. "blockhash"  (string, required) Block hash to wait for.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct WaitForBlock {
    /// The blockhash.
    pub hash: String,
    /// Block height.
    pub height: i64,
}

/// Result of JSON-RPC method `waitforblockheight`.
///
/// > waitforblockheight "height" ( timeout )
/// >
/// > Waits for (at least) block height and returns the height and hash
/// > of the current tip.
/// >
/// > Arguments:
/// > 1. "blockhash"  (string, required) Block hash to wait for
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct WaitForBlockHeight {
    /// The blockhash.
    pub hash: String,
    /// Block height.
    pub height: i64,
}

/// Result of JSON-RPC method `waitfornewblock`.
///
/// > waitfornewblock ( timeout "current_tip" )
/// >
/// > Waits for any new block and returns useful info about it.
/// >
/// > Returns the current block on timeout or exit.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct WaitForNewBlock {
    /// The blockhash.
    pub hash: String,
    /// Block height.
    pub height: i64,
}
