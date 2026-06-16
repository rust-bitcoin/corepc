// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v31` - raw transactions.
//!
//! Types for methods found under the `== Rawtransactions ==` section of the API docs.

mod into;

use serde::{Deserialize, Serialize};

/// Result of JSON-RPC method `abortprivatebroadcast`.
///
/// > abortprivatebroadcast "id"
/// >
/// > Abort private broadcast attempts for a transaction currently being privately broadcast.
/// > The transaction will be removed from the private broadcast queue.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct AbortPrivateBroadcast {
    /// The removed transactions.
    pub removed_transactions: Vec<RemovedTransaction>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct RemovedTransaction {
    /// The transaction hash in hex.
    pub txid: String,
    /// The transaction witness hash in hex.
    pub wtxid: String,
    /// The serialized, hex-encoded transaction data.
    pub hex: String,
}

/// Result of JSON-RPC method `getprivatebroadcastinfo`.
///
/// > getprivatebroadcastinfo
/// >
/// > Returns information about transactions that are currently being privately broadcast.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetPrivateBroadcastInfo {
    /// The transactions currently being broadcast.
    pub transactions: Vec<PrivateBroadcastTransaction>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct PrivateBroadcastTransaction {
    /// The transaction hash in hex.
    pub txid: String,
    /// The transaction witness hash in hex.
    pub wtxid: String,
    /// The serialized, hex-encoded transaction data.
    pub hex: String,
    /// Per-peer send and acknowledgment information for this transaction.
    pub peers: Vec<PrivateBroadcastPeer>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct PrivateBroadcastPeer {
    /// The address of the peer to which the transaction was sent.
    pub address: String,
    /// The time this transaction was picked for sending to this peer via private
    /// broadcast (seconds since epoch).
    pub sent: i64,
    /// The time this peer acknowledged reception of the transaction (seconds since
    /// epoch).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received: Option<i64>,
}
