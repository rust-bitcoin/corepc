// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v22` - network.
//!
//! Types for methods found under the `== Network ==` section of the API docs.

use alloc::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Result of JSON-RPC method `getnodeaddresses`.
///
/// > getnodeaddresses ( count "network" )
/// >
/// > Return known addresses, after filtering for quality and recency.
/// > These can potentially be used to find new peers in the network.
/// > The total number of addresses known to the node may be higher.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetNodeAddresses(pub Vec<NodeAddress>);

/// An node address item. Part of `getnodeaddresses`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct NodeAddress {
    /// Timestamp in seconds since epoch (Jan 1 1970 GMT) when the node was last seen.
    pub time: u64,
    /// The services offered.
    pub services: u64,
    /// The address of the node.
    pub address: String,
    /// The port of the node.
    pub port: u16,
    /// The network (ipv4, ipv6, onion, i2p, cjdns) the node connected through.
    pub network: String,
}

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
    pub address_bind: String,
    /// Local address as reported by the peer.
    #[serde(rename = "addrlocal")]
    pub address_local: Option<String>,
    /// Network (ipv4, ipv6, or onion) the peer connected through.
    pub network: Option<String>,
    /// The services offered.
    pub services: String,
    /// The services offered, in human-readable form.
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
    /// Whether we selected peer as (compact blocks) high-bandwidth peer.
    pub bip152_hb_to: bool,
    /// Whether peer selected us as (compact blocks) high-bandwidth peer.
    pub bip152_hb_from: bool,
    /// Whether connection was due to addnode/-connect or if it was an automatic/inbound connection.
    #[serde(rename = "addnode")]
    pub add_node: Option<bool>,
    /// The starting height (block) of the peer.
    #[serde(rename = "startingheight")]
    pub starting_height: i64,
    /// The last header we have in common with this peer.
    pub synced_headers: i64,
    /// The last block we have in common with this peer.
    pub synced_blocks: i64,
    /// The heights of blocks we're currently asking from this peer.
    pub inflight: Vec<u64>,
    /// The total number of addresses processed, excluding those dropped due to rate limiting. v21 and
    /// later only.
    #[serde(rename = "addr_processed")]
    pub addresses_processed: usize,
    /// The total number of addresses dropped due to rate limiting. v21 and later only.
    #[serde(rename = "addr_rate_limited")]
    pub addresses_rate_limited: usize,
    /// Any special permissions that have been granted to this peer. v0.19 and later only.
    pub permissions: Vec<String>,
    /// Whether the peer is whitelisted.
    pub whitelisted: Option<bool>,
    /// The minimum fee rate for transactions this peer accepts.
    #[serde(rename = "minfeefilter")]
    pub min_fee_filter: Option<f64>, // Docs rekon this exists.
    /// The total bytes sent aggregated by message type.
    #[serde(rename = "bytessent_per_msg")]
    pub bytes_sent_per_message: BTreeMap<String, u64>,
    /// The total bytes received aggregated by message type.
    #[serde(rename = "bytesrecv_per_msg")]
    pub bytes_received_per_message: BTreeMap<String, u64>,
    /// Type of connection.
    pub connection_type: Option<String>,
}

/// Result of JSON-RPC method `listbanned`.
///
/// > listbanned
///
/// > List all banned IPs/Subnets.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ListBanned(pub Vec<Banned>);

/// An banned item. Part of `listbanned`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Banned {
    /// The IP/Subnet of the banned node.
    pub address: String,
    /// The UNIX epoch time the ban was created.
    pub ban_created: u32,
    /// The UNIX epoch time the ban was expires.
    pub banned_until: u32,
    /// The ban duration, in seconds.
    pub ban_duration: u32,
    /// The time remaining until the ban expires, in seconds.
    pub time_remaining: u32,
}
