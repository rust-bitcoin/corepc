// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v0.18` - blockchain.
//!
//! Types for methods found under the `== Blockchain ==` section of the API docs.

mod into;

use serde::{Deserialize, Serialize};

use super::{MempoolEntryError, MempoolEntryFees, ScanTxOutSetError};

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
    /// This is different from actual serialized size for witness transactions as witness data is discounted.
    pub size: i64,
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
    /// (No docs in Core v0.17.)
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

/// Result of JSON-RPC method `scantxoutset`.
///
/// > scantxoutset "action" ( [scanobjects,...] )
/// >
/// > Arguments:
/// > 1. "action"                       (string, required) The action to execute
/// > 2. "scanobjects"                  (array, required) Array of scan objects
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ScanTxOutSetStart {
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
    /// A specialized descriptor for the matched scriptPubKey
    #[serde(rename = "desc")]
    pub descriptor: String,
    /// The total amount in BTC of unspent output
    pub amount: f64,
    /// Height of the unspent transaction output
    pub height: u64,
}
