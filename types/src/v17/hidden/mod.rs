// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v0.17` - hidden.
//!
//! Types for methods that are excluded from the API docs by default.

mod error;
mod into;

use serde::{Deserialize, Serialize};

pub use self::error::{WaitForBlockError, WaitForBlockHeightError, WaitForNewBlockError};

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
