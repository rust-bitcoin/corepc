// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v0.25` - blockchain.
//!
//! Types for methods found under the `== Blockchain ==` section of the API docs.

use serde::{Deserialize, Serialize};

use crate::v22::ScanTxOutSetStatus;

/// Result of JSON-RPC method `scantxoutset`.
///
/// > scantxoutset "action" ( [scanobjects,...] )
/// >
/// > Arguments:
/// > 1. action                        (string, required) The action to execute
/// >   "start" for starting a scan
/// >   "abort" for aborting the current scan (returns true when abort was successful)
/// >   "status" for progress report (in %) of the current scan
/// 2. scanobjects                   (json array, required) Array of scan objects
/// > Every scan object is either a string descriptor or an object:
/// > [
/// > "descriptor",             (string) An output descriptor
/// > {                         (json object) An object with output descriptor and metadata
/// > "desc": "str",          (string, required) An output descriptor
/// > "range": n or \[n,n\],   (numeric or array, optional, default=1000) The range of HD chain indexes to explore (either end or \[begin,end\])
/// > },
/// > ...
/// > ]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ScanTxOutSetStart {
    /// Whether the scan is completed
    pub success: bool,
    /// The number of unspent transaction outputs scanned
    pub txouts: u64,
    /// The current block height (index)
    pub height: u64,
    /// The hash of the block at the tip of the chain
    pub bestblock: String,
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
    pub script_pub_key: String,
    /// An output descriptor
    pub desc: String,
    /// The total amount in BTC of unspent output
    pub amount: f64,
    /// Whether this is a coinbase output
    pub coinbase: bool,
    /// Height of the unspent transaction output
    pub height: u64,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ScanTxOutSet {
    Start(ScanTxOutSetStart),
    Abort(bool),
    Status(Option<ScanTxOutSetStatus>),
}
