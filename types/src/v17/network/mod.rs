// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v0.17` - network.
//!
//! Types for methods found under the `== Network ==` section of the API docs.
//!
/// These types do not implement `into_model` because apart from fee rate there is no additional
/// `rust-bitcoin` types needed.
mod error;
mod into;

use alloc::collections::BTreeMap;

use serde::{Deserialize, Serialize};

// TODO: Remove wildcard, use explicit types.
pub use self::error::*;

/// Result of JSON-RPC method `getaddednodeinfo`.
///
/// > getaddednodeinfo ( "node" )
/// >
/// > Returns information about the given added node, or all added nodes
/// > (note that onetry addnodes are not listed here)
/// >
/// > Arguments:
/// > 1. "node"   (string, optional) If provided, return information about this specific node, otherwise all nodes are returned.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetAddedNodeInfo(pub Vec<AddedNode>);

/// An item from the list returned by the JSON-RPC method `getaddednodeinfo`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AddedNode {
    /// The node IP address or name (as provided to addnode).
    #[serde(rename = "addednode")]
    pub added_node: String,
    /// If connected.
    pub connected: bool,
    /// Only when connected = true.
    pub addresses: Vec<AddedNodeAddress>,
}

/// An address returned as part of the JSON-RPC method `getaddednodeinfo`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AddedNodeAddress {
    /// The bitcoin server IP and port we're connected to.
    pub address: String,
    /// Connection, inbound or outbound.
    pub connected: String,
}

/// Result of JSON-RPC method `getnettotals`.
///
/// > getnettotals
/// >
/// > Returns information about network traffic, including bytes in, bytes out,
/// > and current time.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetNetTotals {
    /// Total bytes received.
    #[serde(rename = "totalbytesrecv")]
    pub total_bytes_recieved: u64,
    /// Total bytes sent.
    #[serde(rename = "totalbytessent")]
    pub total_bytes_sent: u64,
    /// Current UNIX time in milliseconds.
    #[serde(rename = "timemillis")]
    pub time_millis: u64,
    /// Upload target totals.
    #[serde(rename = "uploadtarget")]
    pub upload_target: UploadTarget,
}

/// The `upload_target` field from the result of JSON-RPC method `getnettotals`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UploadTarget {
    /// Length of the measuring timeframe in seconds.
    pub timeframe: u64,
    /// Target in bytes.
    pub target: u64,
    /// True if target is reached.
    pub target_reached: bool,
    /// True if serving historical blocks.
    pub serve_historical_blocks: bool,
    /// Bytes left in current time cycle.
    pub bytes_left_in_cycle: u64,
    /// Seconds left in current time cycle.
    pub time_left_in_cycle: u64,
}

/// Result of the JSON-RPC method `getnetworkinfo`.
///
/// > getnetworkinfo
///
/// > Returns an object containing various state info regarding P2P networking.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
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

/// Part of the result of the JSON-RPC method `getnetworkinfo` (information per network).
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetNetworkInfoNetwork {
    /// Network (ipv4, ipv6, onion, i2p, cjdns).
    pub name: String,
    /// Is the network limited using -onlynet?.
    pub limited: bool,
    /// Is the network reachable?
    pub reachable: bool,
    /// ("host:port"): The proxy that is used for this network, or empty if none.
    pub proxy: String,
    /// Whether randomized credentials are used.
    pub proxy_randomize_credentials: bool,
}

/// Part of the result of the JSON-RPC method `getnetworkinfo` (local address info).
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetNetworkInfoAddress {
    /// Network address.
    pub address: String,
    /// Network port.
    pub port: u16,
    /// Relative score.
    pub score: u32,
}

/// Result of JSON-RPC method `getpeerinfo`.
///
/// > getpeerinfo
/// >
/// > Returns data about each connected network node as a json array of objects.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetPeerInfo(pub Vec<PeerInfo>);

/// An item from the list returned by the JSON-RPC method `getpeerinfo`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
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
    /// Whether the peer is whitelisted (deprecated in v0.21).
    pub whitelisted: Option<bool>,
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
pub struct ListBanned(pub Vec<Banned>);

/// An item from the list returned by the JSON-RPC method `listbanned`
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Banned {
    /// The IP/Subnet of the banned node (Present in all versions)
    pub address: String,
    /// The UNIX epoch time the ban was created (Present v17+)
    /// None if missing in JSON
    pub ban_created: Option<i64>,
    /// The UNIX epoch time the ban expires (Present v17+)
    pub banned_until: Option<i64>,
    /// The ban reason string (Present only in v17 - v20)
    pub ban_reason: Option<String>,
    /// The ban duraction, in seconds (Present v22+)
    pub ban_duration: Option<i64>,
    /// The time remaining until ban expires, in seconds (Present v22+)
    pub time_remaining: Option<i64>,
}

/// Result of JSON-RPC method `addnode`.
///
/// > addnode
/// >
/// > Returns null (json null)
/// >
/// > Arguments:
/// > 1. node           (string, required) The address of the peer to connect to
/// > 2. command        (string, required) 'add' to add a node to the list, 'remove' to remove a node from the list, 'onetry' to try a connection to the node once
/// > 3. v2transport    (boolean, optional, default=set by -v2transport) Attempt to connect using BIP324 v2 transport protocol (ignored for 'remove' command)
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct AddNode;

/// Result of JSON-RPC method `clearbanned`.
///
/// > clearbanned
/// >
/// > Returns null (json null)
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ClearBanned;

/// Result of JSON-RPC method `setban`.
///
/// > setban
/// >
/// > Returns null (json null)
/// >
/// > Arguments:
/// > 1. subnet      (string, required) The IP/Subnet (see getpeerinfo for nodes IP) with an optional netmask (default is /32 = single IP)
/// > 2. command     (string, required) 'add' to add an IP/Subnet to the list, 'remove' to remove an IP/Subnet from the list
/// > 3. bantime     (numeric, optional, default=0) time in seconds how long (or until when if \[absolute\] is set) the IP is banned (0 or empty means using the default time of 24h which can also be overwritten by the -bantime startup argument)
/// > 4. absolute    (boolean, optional, default=false) If set, the bantime must be an absolute timestamp expressed in UNIX epoch time
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetBan;

/// Result of JSON-RPC method `disconnectnode`.
///
/// > disconnectnode
/// >
/// > Returns null (json null)
/// >
/// > Arguments:
/// > 1. address    (string, optional, default=fallback to nodeid) The IP address/port of the node
/// > 2. nodeid     (numeric, optional, default=fallback to address) The node ID (see getpeerinfo for node IDs)
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DisconnectNode;

/// Result of JSON-RPC method `ping`.
///
/// > ping
/// >
/// > Returns null (json null)
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Ping;

/// Result of JSON-RPC method `setnetworkactive`.
///
/// > setnetworkactive
/// >
/// > Returns null (json null)
/// >
/// > Arguments:
/// > 1. state (boolean, required) true to enable networking, false to disable
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetNetworkActive;

/// Result of JSON-RPC method `importprivkey`.
///
/// > importprivkey
/// >
/// > Returns null (json null)
/// >
/// > Arguments:
/// > 1. privkey    (string, required) The private key (see dumpprivkey)
/// > 2. label      (string, optional, default=current label if address exists, otherwise "") An optional label
/// > 3. rescan     (boolean, optional, default=true) Scan the chain and mempool for wallet transactions.
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ImportPrivKey;

/// Result of JSON-RPC method `getconnectioncount`.
///
/// > getconnectioncount
/// >
/// > Returns n (numeric) The connection count
#[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GetConnectionCount(pub u64);
