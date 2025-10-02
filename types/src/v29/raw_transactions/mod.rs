// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v29` - raw transactions.
//!
//! Types for methods found under the `== Rawtransactions ==` section of the API docs.

mod into;

use serde::{Deserialize, Serialize};

pub use super::{MempoolAcceptanceError, TestMempoolAcceptError};

/// Arguments of JSON-RPC method `createrawtransaction`.
///
/// # Note
///
/// Assumes that the transaction is always "replaceable" by default and has a locktime of 0.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CreateRawTransactionArguments {
    pub inputs: Vec<CreateRawTransactionInput>,
    pub outputs: Vec<CreateRawTransactionOutput>,
}

/// Inputs of JSON-RPC method `createrawtransaction`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct CreateRawTransactionInput {
    pub txid: String,
    pub vout: u32,
}

/// Transaction outputs for Bitcoin RPC methods.
///
/// Used by various RPC methods such as `createrawtransaction`, `psbtbumpfee`,
/// and `walletcreatefundedpsbt`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CreateRawTransactionOutput {
    /// A pair of an address and an amount in BTC float.
    AddressAmount {
        /// Address to sent to.
        address: String,
        /// Amount to send in BTC float
        amount: f64,
    },
    /// A payload such as in `OP_RETURN` transactions.
    Data {
        /// The payload.
        data: String,
    },
}

/// Result of JSON-RPC method `testmempoolaccept`.
///
/// > testmempoolaccept ["rawtxs"] ( allowhighfees )
/// >
/// > Returns if raw transaction (serialized, hex-encoded) would be accepted by mempool.
/// >
/// > This checks if the transaction violates the consensus or policy rules.
/// >
/// > See sendrawtransaction call.
/// >
/// > Arguments:
/// > 1. ["rawtxs"]       (array, required) An array of hex strings of raw transactions.
/// >                                         Length must be one for now.
/// > 2. allowhighfees    (boolean, optional, default=false) Allow high fees
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct TestMempoolAccept(pub Vec<MempoolAcceptance>);

/// A single mempool acceptance test result. Part of `testmempoolaccept`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct MempoolAcceptance {
    /// The transaction hash in hex.
    pub txid: String,
    /// The transaction witness hash in hex.
    pub wtxid: String,
    /// If the mempool allows this tx to be inserted.
    pub allowed: bool,
    /// Virtual transaction size as defined in BIP 141. This is different from actual serialized size for witness transactions as witness data is discounted (only present when 'allowed' is true).
    pub vsize: Option<i64>,
    /// Transaction fees (only present if 'allowed' is true).
    pub fees: Option<MempoolAcceptanceFees>,
    /// Rejection string (only present when 'allowed' is false).
    #[serde(rename = "reject-reason")]
    pub reject_reason: Option<String>,
    /// Rejection details (only present when 'allowed' is false and rejection details exist)
    #[serde(rename = "reject-details")]
    pub reject_details: Option<String>,
}

/// Wrapper for the fees field. Part of `testmempoolaccept`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct MempoolAcceptanceFees {
    /// Transaction fee in BTC.
    pub base: f64,
    /// The effective feerate in BTC per KvB. May differ from the base feerate if, for example, there
    /// are modified fees from `prioritisetransaction` or a package feerate was used.
    #[serde(rename = "effective-feerate", default)]
    pub effective_feerate: Option<f64>,
    /// Transactions whose fees and vsizes are included in `effective_feerate`.
    #[serde(rename = "effective-includes", default)]
    pub effective_includes: Vec<String>,
}
