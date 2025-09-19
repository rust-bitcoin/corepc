// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v23` - network.
//!
//! Types for methods found under the `== Network ==` section of the API docs.

use alloc::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Result of JSON-RPC method `getpeerinfo`.
///
/// > getpeerinfo
/// >
/// > Returns data about each connected network node as a json array of objects.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetPeerInfo(pub Vec<PeerInfo>);

/// A peer info item. Part of `getpeerinfo`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct PeerInfo {
    /// Peer index.
    pub id: u32,
    /// The IP address and port of the peer ("host:port").
    #[serde(rename = "addr")]
    pub address: String,
    /// Bind address of the connection to the peer ("ip:port").
    #[serde(rename = "addrbind")]
    pub address_bind: Option<String>,
    /// Local address as reported by the peer.
    #[serde(rename = "addrlocal")]
    pub address_local: Option<String>,
    /// Network (ipv4, ipv6, or onion) the peer connected through.
    pub network: Option<String>,
    /// The services offered.
    pub services: String,
    /// The services offered, in human-readable form. v0.19 and later only.
    #[serde(rename = "servicesnames")]
    pub services_names: Vec<String>,
    /// Whether peer has asked us to relay transactions to it.
    #[serde(rename = "relaytxes")]
    pub relay_transactions: bool,
    /// The time in seconds since epoch (Jan 1 1970 GMT) of the last send.
    #[serde(rename = "lastsend")]
    pub last_send: i64,
    /// The time in seconds since epoch (Jan 1 1970 GMT) of the last receive.
    #[serde(rename = "lastrecv")]
    pub last_received: i64,
    /// The UNIX epoch time of the last valid transaction received from this peer. v21 and later only.
    pub last_transaction: i64,
    /// The UNIX epoch time of the last block received from this peer. v21 and later only.
    pub last_block: i64,
    /// The total bytes sent.
    #[serde(rename = "bytessent")]
    pub bytes_sent: u64,
    /// The total bytes received.
    #[serde(rename = "bytesrecv")]
    pub bytes_received: u64,
    /// The connection time in seconds since epoch (Jan 1 1970 GMT).
    #[serde(rename = "conntime")]
    pub connection_time: i64,
    /// The time offset in seconds.
    #[serde(rename = "timeoffset")]
    pub time_offset: i64,
    /// Ping time (if available).
    #[serde(rename = "pingtime")]
    pub ping_time: Option<f64>,
    /// Minimum observed ping time (if any at all).
    #[serde(rename = "minping")]
    pub minimum_ping: Option<f64>,
    /// Ping wait (if non-zero).
    #[serde(rename = "pingwait")]
    pub ping_wait: Option<f64>,
    /// The peer version, such as 70001.
    pub version: u32,
    /// The string version (e.g. "/Satoshi:0.8.5/").
    #[serde(rename = "subver")]
    pub subversion: String,
    /// Inbound (true) or Outbound (false).
    pub inbound: bool,
    /// Whether we selected peer as (compact blocks) high-bandwidth peer. v22 and later only.
    pub bip152_hb_to: bool,
    /// Whether peer selected us as (compact blocks) high-bandwidth peer. v22 and later only.
    pub bip152_hb_from: bool,
    /// Whether connection was due to addnode/-connect or if it was an automatic/inbound connection.
    #[serde(rename = "addnode")]
    pub add_node: Option<bool>,
    /// The starting height (block) of the peer.
    #[serde(rename = "startingheight")]
    pub starting_height: Option<i64>,
    /// The ban score.
    #[serde(rename = "banscore")]
    pub ban_score: Option<i64>,
    /// The last header we have in common with this peer.
    pub synced_headers: Option<i64>,
    /// The last block we have in common with this peer.
    pub synced_blocks: Option<i64>,
    /// The heights of blocks we're currently asking from this peer.
    pub inflight: Option<Vec<u64>>,
    /// Whether we participate in address relay with this peer.
    #[serde(rename = "addr_relay_enabled")]
    pub addresses_relay_enabled: Option<bool>,
    /// The total number of addresses processed, excluding those dropped due to rate limiting.
    #[serde(rename = "addr_processed")]
    pub addresses_processed: Option<usize>,
    /// The total number of addresses dropped due to rate limiting.
    #[serde(rename = "addr_rate_limited")]
    pub addresses_rate_limited: Option<usize>,
    /// Any special permissions that have been granted to this peer.
    pub permissions: Vec<String>,
    /// The minimum fee rate for transactions this peer accepts.
    #[serde(rename = "minfeefilter")]
    pub minimum_fee_filter: f64,
    /// The total bytes sent aggregated by message type.
    #[serde(rename = "bytessent_per_msg")]
    pub bytes_sent_per_message: BTreeMap<String, u64>,
    /// The total bytes received aggregated by message type.
    #[serde(rename = "bytesrecv_per_msg")]
    pub bytes_received_per_message: BTreeMap<String, u64>,
    /// Type of connection.
    pub connection_type: Option<String>,
}
