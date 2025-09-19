// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v0.19` - network.
//!
//! Types for methods found under the `== Network ==` section of the API docs.

mod into;

use alloc::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::{GetNetworkInfoAddress, GetNetworkInfoError, GetNetworkInfoNetwork};

/// Result of the JSON-RPC method `getnetworkinfo`.
///
/// > getnetworkinfo
///
/// > Returns an object containing various state info regarding P2P networking.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetNetworkInfo {
    /// The server version.
    pub version: usize,
    /// The server subversion string.
    pub subversion: String,
    /// The protocol version.
    #[serde(rename = "protocolversion")]
    pub protocol_version: usize,
    /// The services we offer to the network (hex string).
    #[serde(rename = "localservices")]
    pub local_services: String,
    /// The services we offer to the network.
    #[serde(rename = "localservicesnames")]
    pub local_services_names: Vec<String>,
    /// `true` if transaction relay is requested from peers.
    #[serde(rename = "localrelay")]
    pub local_relay: bool,
    /// The time offset.
    #[serde(rename = "timeoffset")]
    pub time_offset: isize,
    /// The total number of connections.
    pub connections: usize,
    /// Whether p2p networking is enabled.
    #[serde(rename = "networkactive")]
    pub network_active: bool,
    /// Information per network.
    pub networks: Vec<GetNetworkInfoNetwork>,
    /// Minimum relay fee rate for transactions in BTC/kB.
    #[serde(rename = "relayfee")]
    pub relay_fee: f64,
    /// Minimum fee rate increment for mempool limiting or replacement in BTC/kB.
    #[serde(rename = "incrementalfee")]
    pub incremental_fee: f64,
    /// List of local addresses.
    #[serde(rename = "localaddresses")]
    pub local_addresses: Vec<GetNetworkInfoAddress>,
    /// Any network and blockchain warnings.
    pub warnings: String,
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
    /// Whether connection was due to addnode/-connect or if it was an automatic/inbound connection.
    #[serde(rename = "addnode")]
    pub add_node: Option<bool>,
    /// The starting height (block) of the peer.
    #[serde(rename = "startingheight")]
    pub starting_height: i64,
    /// The ban score.
    #[serde(rename = "banscore")]
    pub ban_score: Option<i64>,
    /// The last header we have in common with this peer.
    pub synced_headers: i64,
    /// The last block we have in common with this peer.
    pub synced_blocks: i64,
    /// The heights of blocks we're currently asking from this peer.
    pub inflight: Vec<u64>,
    /// Any special permissions that have been granted to this peer. v0.19 and later only.
    pub permissions: Vec<String>,
    /// Whether the peer is whitelisted.
    pub whitelisted: Option<bool>,
    /// The minimum fee rate for transactions this peer accepts.
    #[serde(rename = "minfeefilter")]
    pub min_fee_filter: f64,
    /// The total bytes sent aggregated by message type.
    #[serde(rename = "bytessent_per_msg")]
    pub bytes_sent_per_message: BTreeMap<String, u64>,
    /// The total bytes received aggregated by message type.
    #[serde(rename = "bytesrecv_per_msg")]
    pub bytes_received_per_message: BTreeMap<String, u64>,
}
