// SPDX-License-Identifier: CC0-1.0

//! Auto-generated types for Bitcoin Core `31` - network.
//!
//! Produced by `codegen`. Do not edit by hand, re-run
//! `just codegen` to regenerate. Holds the RPC return types for this section; the
//! `*Options` request structs live with the call surface in `corepc-client`.

#![allow(non_camel_case_types, non_snake_case, clippy::large_enum_variant)]

mod into;

use serde::{Deserialize, Serialize};

pub use self::into::{GetNetworkInfoAddressError, GetNetworkInfoError, GetNetworkInfoNetworkError};

/// Result of the JSON-RPC method `getaddednodeinfo`.
///
/// > getaddednodeinfo
/// >
/// > Returns information about the given added node, or all added nodes
/// > (note that onetry addnodes are not listed here)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetAddedNodeInfo(pub Vec<GetAddedNodeInfoItem>);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetAddedNodeInfoItem {
    /// The node IP address or name (as provided to addnode)
    #[serde(rename = "addednode")]
    pub added_node: String,
    /// Only when connected = true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<GetAddedNodeInfoItemAddressesItem>>,
    /// If connected
    pub connected: bool,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetAddedNodeInfoItemAddressesItem {
    /// The bitcoin server IP and port we're connected to
    pub address: String,
    /// connection, inbound or outbound
    pub connected: String,
}

/// Result of the JSON-RPC method `getaddrmaninfo`.
///
/// > getaddrmaninfo
/// >
/// > Provides information about the node's address manager by returning the number of addresses in the `new` and `tried` tables and their sum for all networks.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetAddrManInfo(
    /// Map entries
    pub std::collections::BTreeMap<String, GetAddrManInfoEntry>,
);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetAddrManInfoEntry {
    /// number of addresses in the new table, which represent potential peers the node has discovered but hasn't yet successfully connected to.
    pub new: i64,
    /// total number of addresses in both new/tried tables
    pub total: i64,
    /// number of addresses in the tried table, which represent peers the node has successfully connected to in the past.
    pub tried: i64,
}

/// Result of the JSON-RPC method `getconnectioncount`.
///
/// > getconnectioncount
/// >
/// > Returns the number of connections to other nodes.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetConnectionCount(pub u64);

impl std::ops::Deref for GetConnectionCount {
    type Target = u64;
    fn deref(&self) -> &Self::Target { &self.0 }
}

/// Returns information about network traffic, including bytes in, bytes out,
/// and current system time.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetNetTotals {
    /// Current system UNIX epoch time in milliseconds
    pub timemillis: i64,
    /// Total bytes received
    #[serde(rename = "totalbytesrecv")]
    pub total_bytesrecv: u64,
    /// Total bytes sent
    #[serde(rename = "totalbytessent")]
    pub total_bytessent: u64,
    #[serde(rename = "uploadtarget")]
    pub upload_target: GetNetTotalsUploadTarget,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetNetTotalsUploadTarget {
    /// Bytes left in current time cycle
    pub bytes_left_in_cycle: u64,
    /// True if serving historical blocks
    pub serve_historical_blocks: bool,
    /// Target in bytes
    pub target: u64,
    /// True if target is reached
    pub target_reached: bool,
    /// Seconds left in current time cycle
    pub time_left_in_cycle: i64,
    /// Length of the measuring timeframe in seconds
    pub timeframe: i64,
}

/// Returns an object containing various state info regarding P2P networking.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetNetworkInfo {
    /// the total number of connections
    pub connections: u64,
    /// the number of inbound connections
    pub connections_in: u64,
    /// the number of outbound connections
    pub connections_out: u64,
    /// minimum fee rate increment for mempool limiting or replacement in BTC/kvB
    #[serde(rename = "incrementalfee")]
    pub incremental_fee: f64,
    /// list of local addresses
    #[serde(rename = "localaddresses")]
    pub local_addresses: Vec<GetNetworkInfoLocalAddressesItem>,
    /// true if transaction relay is requested from peers
    #[serde(rename = "localrelay")]
    pub local_relay: bool,
    /// the services we offer to the network
    pub localservices: String,
    /// the services we offer to the network, in human-readable form
    #[serde(rename = "localservicesnames")]
    pub localservices_names: Vec<String>,
    /// whether p2p networking is enabled
    #[serde(rename = "networkactive")]
    pub network_active: bool,
    /// information per network
    pub networks: Vec<GetNetworkInfoNetworksItem>,
    /// the protocol version
    #[serde(rename = "protocolversion")]
    pub protocol_version: i64,
    /// minimum relay fee rate for transactions in BTC/kvB
    #[serde(rename = "relayfee")]
    pub relay_fee: f64,
    /// the server subversion string
    #[serde(rename = "subversion")]
    pub sub_version: String,
    /// the time offset
    #[serde(rename = "timeoffset")]
    pub time_offset: i64,
    /// the server version
    pub version: i64,
    /// any network and blockchain warnings (run with `-deprecatedrpc=warnings` to return the latest warning as a single string)
    pub warnings: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetNetworkInfoLocalAddressesItem {
    /// network address
    pub address: String,
    /// network port
    pub port: i64,
    /// relative score
    pub score: i64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetNetworkInfoNetworksItem {
    /// is the network limited using -onlynet?
    pub limited: bool,
    /// network (ipv4, ipv6, onion, i2p, cjdns)
    pub name: String,
    /// ("host:port") the proxy that is used for this network, or empty if none
    pub proxy: String,
    /// Whether randomized credentials are used
    pub proxy_randomize_credentials: bool,
    /// is the network reachable?
    pub reachable: bool,
}

/// Result of the JSON-RPC method `getnodeaddresses`.
///
/// > getnodeaddresses
/// >
/// > Return known addresses, after filtering for quality and recency.
/// > These can potentially be used to find new peers in the network.
/// > The total number of addresses known to the node may be higher.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetNodeAddresses(pub Vec<GetNodeAddressesItem>);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetNodeAddressesItem {
    /// The address of the node
    pub address: String,
    /// The network (ipv4, ipv6, onion, i2p, cjdns) the node connected through
    pub network: String,
    /// The port number of the node
    pub port: i64,
    /// The services offered by the node
    pub services: u64,
    /// The UNIX epoch time when the node was last seen
    pub time: i64,
}

/// Result of the JSON-RPC method `getpeerinfo`.
///
/// > getpeerinfo
/// >
/// > Returns data about each connected network peer as a json array of objects.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetPeerInfo(pub Vec<GetPeerInfoItem>);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetPeerInfoItem {
    /// (host:port) The IP address/hostname optionally followed by :port of the peer
    pub addr: String,
    /// The total number of addresses processed, excluding those dropped due to rate limiting
    pub addr_processed: u64,
    /// The total number of addresses dropped due to rate limiting
    pub addr_rate_limited: u64,
    /// Whether we participate in address relay with this peer
    pub addr_relay_enabled: bool,
    /// (ip:port) Bind address of the connection to the peer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addrbind: Option<String>,
    /// (ip:port) Local address as reported by the peer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addrlocal: Option<String>,
    /// Whether peer selected us as (compact blocks) high-bandwidth peer
    pub bip152_hb_from: bool,
    /// Whether we selected peer as (compact blocks) high-bandwidth peer
    pub bip152_hb_to: bool,
    /// The total bytes received
    pub bytesrecv: u64,
    pub bytesrecv_per_msg: std::collections::BTreeMap<String, u64>,
    /// The total bytes sent
    pub bytessent: u64,
    pub bytessent_per_msg: std::collections::BTreeMap<String, u64>,
    /// Type of connection:
    /// outbound-full-relay (default automatic connections),
    /// block-relay-only (does not relay transactions or addresses),
    /// inbound (initiated by the peer),
    /// manual (added via addnode RPC or -addnode/-connect configuration options),
    /// addr-fetch (short-lived automatic connection for soliciting addresses),
    /// feeler (short-lived automatic connection for testing addresses),
    /// private-broadcast (short-lived automatic connection for broadcasting privacy-sensitive transactions).
    /// Please note this output is unlikely to be stable in upcoming releases as we iterate to
    /// best capture connection behaviors.
    pub connection_type: String,
    /// The UNIX epoch time of the connection
    #[serde(rename = "conntime")]
    pub conn_time: i64,
    /// Peer index
    pub id: i64,
    /// Inbound (true) or Outbound (false)
    pub inbound: bool,
    pub inflight: Vec<i64>,
    /// How many txs we have queued to announce to this peer
    pub inv_to_send: i64,
    /// The UNIX epoch time of the last block received from this peer
    pub last_block: i64,
    /// Mempool sequence number of this peer's last INV
    pub last_inv_sequence: u64,
    /// The UNIX epoch time of the last valid transaction received from this peer
    pub last_transaction: i64,
    /// The UNIX epoch time of the last receive
    pub lastrecv: i64,
    /// The UNIX epoch time of the last send
    #[serde(rename = "lastsend")]
    pub last_send: i64,
    /// Mapped AS (Autonomous System) number at the end of the BGP route to the peer, used for diversifying
    /// peer selection (only displayed if the -asmap config option is set)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mapped_as: Option<i64>,
    /// The minimum fee rate for transactions this peer accepts
    #[serde(rename = "minfeefilter")]
    pub min_fee_filter: f64,
    /// The minimum observed ping time in seconds, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minping: Option<f64>,
    /// Network (ipv4, ipv6, onion, i2p, cjdns, not_publicly_routable)
    pub network: String,
    /// Any special permissions that have been granted to this peer
    pub permissions: Vec<String>,
    /// The last ping time in seconds, if any
    #[serde(rename = "pingtime", skip_serializing_if = "Option::is_none")]
    pub ping_time: Option<f64>,
    /// The duration in seconds of an outstanding ping (if non-zero)
    #[serde(rename = "pingwait", skip_serializing_if = "Option::is_none")]
    pub ping_wait: Option<f64>,
    /// The current height of header pre-synchronization with this peer, or -1 if no low-work sync is in progress
    pub presynced_headers: i64,
    /// Whether we relay transactions to this peer
    #[serde(rename = "relaytxes")]
    pub relay_txes: bool,
    /// The services offered
    pub services: String,
    /// the services offered, in human-readable form
    #[serde(rename = "servicesnames")]
    pub services_names: Vec<String>,
    /// The session ID for this connection, or "" if there is none ("v2" transport protocol only).
    ///
    pub session_id: String,
    /// The string version
    pub subver: String,
    /// The last block we have in common with this peer
    pub synced_blocks: i64,
    /// The last header we have in common with this peer
    pub synced_headers: i64,
    /// The time offset in seconds
    #[serde(rename = "timeoffset")]
    pub time_offset: i64,
    /// Type of transport protocol:
    /// detecting (peer could be v1 or v2),
    /// v1 (plaintext transport protocol),
    /// v2 (BIP324 encrypted transport protocol).
    ///
    pub transport_protocol_type: String,
    /// The peer version, such as 70001
    pub version: i64,
}

/// Result of the JSON-RPC method `listbanned`.
///
/// > listbanned
/// >
/// > List all manually banned IPs/Subnets.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ListBanned(pub Vec<ListBannedItem>);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ListBannedItem {
    /// The IP/Subnet of the banned node
    pub address: String,
    /// The UNIX epoch time the ban was created
    pub ban_created: i64,
    /// The ban duration, in seconds
    pub ban_duration: i64,
    /// The UNIX epoch time the ban expires
    pub banned_until: i64,
    /// The time remaining until the ban expires, in seconds
    pub time_remaining: i64,
}

/// Result of the JSON-RPC method `setnetworkactive`.
///
/// > setnetworkactive
/// >
/// > Disable/enable all p2p network activity.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SetNetworkActive(pub bool);

impl std::ops::Deref for SetNetworkActive {
    type Target = bool;
    fn deref(&self) -> &Self::Target { &self.0 }
}
