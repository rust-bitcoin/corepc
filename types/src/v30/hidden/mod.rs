// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v29` - hidden.
//!
//! Types for methods that are excluded from the API docs by default.

mod error;
mod into;

use bitcoin::Txid;
use serde::{Deserialize, Serialize};

pub use self::error::{GetOrphanTxsVerboseOneEntryError, GetOrphanTxsVerboseTwoEntryError};

/// Result of JSON-RPC method `getorphantxs` verbosity 0.
///
/// > getorphantxs ( verbosity )
/// >
/// > Shows transactions in the tx orphanage.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetOrphanTxs(pub Vec<Txid>);

/// Result of JSON-RPC method `getorphantxs` verbosity 1.
///
/// > getorphantxs ( verbosity )
/// >
/// > Shows transactions in the tx orphanage.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetOrphanTxsVerboseOne(pub Vec<GetOrphanTxsVerboseOneEntry>);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetOrphanTxsVerboseOneEntry {
    /// The transaction hash in hex
    pub txid: String,
    /// The transaction witness hash in hex
    pub wtxid: String,
    /// The serialized transaction size in bytes
    pub bytes: u64,
    /// The virtual transaction size as defined in BIP 141. This is different from actual serialized size for witness transactions as witness data is discounted.
    pub vsize: u64,
    /// The transaction weight as defined in BIP 141.
    pub weight: u64,
    /// The entry time into the orphanage expressed in UNIX epoch time
    /// Only present in v29.
    #[serde(rename = "entry")]
    pub entry_time: Option<u32>,
    /// The orphan expiration time expressed in UNIX epoch time
    /// Only present in v29.
    #[serde(rename = "expiration")]
    pub expiration_time: Option<u32>,
    /// List of peer ids that we store this transaction for.
    pub from: Vec<u64>,
}

/// Result of JSON-RPC method `getorphantxs` verbosity 2.
///
/// > getorphantxs ( verbosity )
/// >
/// > Shows transactions in the tx orphanage.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetOrphanTxsVerboseTwo(pub Vec<GetOrphanTxsVerboseTwoEntry>);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetOrphanTxsVerboseTwoEntry {
    /// The transaction hash in hex
    pub txid: String,
    /// The transaction witness hash in hex
    pub wtxid: String,
    /// The serialized transaction size in bytes
    pub bytes: u64,
    /// The virtual transaction size as defined in BIP 141. This is different from actual serialized size for witness transactions as witness data is discounted.
    pub vsize: u64,
    /// The transaction weight as defined in BIP 141.
    pub weight: u64,
    /// List of peer ids that we store this transaction for.
    pub from: Vec<u64>,
    /// The entry time into the orphanage expressed in UNIX epoch time
    /// Only present in v29.
    pub entry_time: Option<u32>,
    /// The orphan expiration time expressed in UNIX epoch time
    /// Only present in v29.
    pub expiration_time: Option<u32>,
    /// The serialized, hex-encoded transaction data.
    pub hex: String,
}
