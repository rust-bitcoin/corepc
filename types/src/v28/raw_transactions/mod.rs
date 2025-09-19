// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v28` - raw transactions.
//!
//! Types for methods found under the `== Rawtransactions ==` section of the API docs.

mod error;
mod into;

use alloc::collections::BTreeMap;

use serde::{Deserialize, Serialize};

// TODO: Remove wildcard, use explicit types.
pub use self::error::*;
use crate::model;

/// Result of JSON-RPC method `submitpackage`.
///
/// > submitpackage ["rawtx",...] ( maxfeerate maxburnamount )
/// >
/// > Submit a package of raw transactions (serialized, hex-encoded) to local node.
/// > The package will be validated according to consensus and mempool policy rules. If any transaction passes, it will be accepted to mempool.
/// > This RPC is experimental and the interface may be unstable. Refer to doc/policy/packages.md for documentation on package policies.
/// > Warning: successful submission does not mean the transactions will propagate throughout the network.
/// >
/// > Arguments:
/// > 1. package          (json array, required) An array of raw transactions.
/// >                     The package must solely consist of a child and its parents. None of the parents may depend on each other.
/// >                     The package must be topologically sorted, with the child being the last element in the array.
/// >      [
/// >        "rawtx",     (string)
/// >        ...
/// >      ]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SubmitPackage {
    /// The transaction package result message.
    ///
    /// "success" indicates all transactions were accepted into or are already in the mempool.
    pub package_msg: String,
    /// Transaction results keyed by wtxid.
    #[serde(rename = "tx-results")]
    pub tx_results: BTreeMap<String, SubmitPackageTxResult>,
    /// List of txids of replaced transactions.
    #[serde(rename = "replaced-transactions")]
    pub replaced_transactions: Vec<String>,
}

/// The per-transaction result. Part of `submitpackage`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SubmitPackageTxResult {
    /// The transaction id.
    pub txid: String,
    /// The wtxid of a different transaction with the same txid but different witness found in the mempool.
    ///
    /// If set, this means the submitted transaction was ignored.
    #[serde(rename = "other-wtxid")]
    pub other_wtxid: Option<String>,
    /// Sigops-adjusted virtual transaction size.
    pub vsize: Option<i64>,
    /// Transaction fees.
    pub fees: Option<SubmitPackageTxResultFees>,
    /// The transaction error string, if it was rejected by the mempool.
    pub error: Option<String>,
}

/// The fees included in the per-transaction result. Part of `submitpackage`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SubmitPackageTxResultFees {
    /// Transaction fee.
    #[serde(rename = "base")]
    pub base_fee: f64,
    /// The effective feerate.
    ///
    /// Will be `None` if the transaction was already in the mempool. For example, the package
    /// feerate and/or feerate with modified fees from the `prioritisetransaction` JSON-RPC method.
    #[serde(rename = "effective-feerate")]
    pub effective_fee_rate: Option<f64>,
    /// If [`Self::effective_fee_rate`] is provided, this holds the wtxid's of the transactions
    /// whose fees and vsizes are included in effective-feerate.
    #[serde(rename = "effective-includes")]
    pub effective_includes: Vec<String>,
}
