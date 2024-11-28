// SPDX-License-Identifier: CC0-1.0

//! Types for methods found under the `== Rawtransactions ==` section of the API docs.
//!
//! These structs model the types returned by the JSON-RPC API but have concrete types
//! and are not specific to a specific version of Bitcoin Core.

use std::collections::BTreeMap;

use bitcoin::{Amount, FeeRate, Txid, Wtxid};
use serde::{Deserialize, Serialize};

/// Models the result of JSON-RPC method `sendrawtransaction`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SendRawTransaction(pub Txid);

/// Models the result of JSON-RPC method `submitpackage`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SubmitPackage {
    /// The transaction package result message. "success" indicates all transactions were accepted into or are already in the mempool.
    pub package_msg: String,
    /// Transaction results keyed by [`Wtxid`].
    pub tx_results: BTreeMap<Wtxid, SubmitPackageTxResult>,
    /// List of txids of replaced transactions.
    pub replaced_transactions: Vec<Txid>,
}

/// Models the per-transaction result included in the JSON-RPC method `submitpackage`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SubmitPackageTxResult {
    /// The transaction id.
    pub txid: Txid,
    /// The [`Wtxid`] of a different transaction with the same [`Txid`] but different witness found in the mempool.
    ///
    /// If set, this means the submitted transaction was ignored.
    pub other_wtxid: Option<Wtxid>,
    /// Sigops-adjusted virtual transaction size.
    pub vsize: Option<u32>,
    /// Transaction fees.
    pub fees: Option<SubmitPackageTxResultFees>,
    /// The transaction error string, if it was rejected by the mempool
    pub error: Option<String>,
}

/// Models the fees included in the per-transaction result of the JSON-RPC method `submitpackage`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SubmitPackageTxResultFees {
    /// Transaction fee.
    pub base_fee: Amount,
    /// The effective feerate.
    ///
    /// Will be `None` if the transaction was already in the mempool. For example, the package
    /// feerate and/or feerate with modified fees from the `prioritisetransaction` JSON-RPC method.
    pub effective_fee_rate: Option<FeeRate>,
    /// If [`Self::effective_fee_rate`] is provided, this holds the [`Wtxid`]s of the transactions
    /// whose fees and vsizes are included in effective-feerate.
    pub effective_includes: Vec<Wtxid>,
}
