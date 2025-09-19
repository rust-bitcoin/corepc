// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v0.18` - blockchain.
//!
//! Types for methods found under the `== Blockchain ==` section of the API docs.

mod into;

use alloc::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::{MapMempoolEntryError, MempoolEntryError, MempoolEntryFees};

/// Result of JSON-RPC method `getmempoolancestors` with verbose set to `false`.
///
/// > getmempoolancestors txid (verbose)
/// >
/// > If txid is in the mempool, returns all in-mempool ancestors.
/// >
/// > Arguments:
/// > 1. "txid"                 (string, required) The transaction id (must be in mempool)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolAncestors(pub Vec<String>);

/// Result of JSON-RPC method `getmempoolancestors` with verbose set to true.
///
/// Map of txid to `MempoolEntry` i.e., an ancestor.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolAncestorsVerbose(pub BTreeMap<String, MempoolEntry>);

/// Result of JSON-RPC method `getmempooldescendants` with verbose set to `false`.
///
/// > getmempooldescendants txid (verbose)
/// >
/// > If txid is in the mempool, returns all in-mempool descendants.
/// >
/// > Arguments:
/// > 1. "txid"                 (string, required) The transaction id (must be in mempool)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolDescendants(pub Vec<String>);

/// Result of JSON-RPC method `getmempooldescendants` with verbose set to true.
///
/// Map of txid to [`MempoolEntry`] i.e., a descendant.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolDescendantsVerbose(pub BTreeMap<String, MempoolEntry>);

/// Result of JSON-RPC method `getmempoolentry`.
///
/// > getmempoolentry txid
/// >
/// > Returns mempool data for given transaction
/// >
/// > Arguments:
/// > 1. "txid"                 (string, required) The transaction id (must be in mempool)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolEntry(pub MempoolEntry);

/// Mempool data. Part of `getmempoolentry`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
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
