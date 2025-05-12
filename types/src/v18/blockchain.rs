// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v0.18` - blockchain.
//!
//! Types for methods found under the `== Blockchain ==` section of the API docs.
use serde::{Deserialize, Serialize};

/// Result of JSON-RPC method `scantxoutset`.
///
/// > scantxoutset "action" ( [scanobjects,...] )
/// >
/// > Arguments:
/// > 1. action                        (string, required) The action to execute
/// 2. scanobjects                   (json array, required) Array of scan objects
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ScanTxOutSet {
    /// Returns after scan completes
    Start(ScanTxOutSetStart),
    /// True (scan will be aborted), False (no scan to abort)
    Abort(bool),
    /// Scan in progress or Completed
    Status(Option<ScanTxOutSetStatus>),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ScanTxOutSetStart {
    /// The unspents
    pub unspents: Vec<ScanTxOutSetUnspent>,
    /// The total amount of all found unspent outputs in BTC
    pub total_amount: f64,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ScanTxOutSetStatus {
    /// Approximate percent complete
    pub progress: f64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ScanTxOutSetUnspent {
    /// The transaction id
    pub txid: String,
    /// The vout value
    pub vout: u32,
    /// The script key
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: String,
    /// An output descriptor
    pub desc: String,
    /// The total amount in BTC of unspent output
    pub amount: f64,
    /// Height of the unspent transaction output
    pub height: u64,
}
