// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v31` - network.
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
    /// (host:port) The IP address/hostname optionally followed by :port of the peer.
    #[serde(rename = "addr")]
    pub address: String,
    /// (ip:port) Bind address of the connection to the peer.
    #[serde(rename = "addrbind")]
    pub address_bind: Option<String>,
    /// (ip:port) Local address as reported by the peer.
    #[serde(rename = "addrlocal")]
    pub address_local: Option<String>,
    /// Network (ipv4, ipv6, onion, i2p, cjdns, not_publicly_routable).
    pub network: String,
    /// Mapped AS (Autonomous System) number at the end of the BGP route to the peer, used for diversifying peer selection (only displayed if the -asmap config option is set).
    pub mapped_as: Option<u32>,
    /// The services offered.
    pub services: String,
    /// The services offered, in human-readable form.
    #[serde(rename = "servicesnames")]
    pub services_names: Vec<String>,
    /// Whether we relay transactions to this peer.
    #[serde(rename = "relaytxes")]
    pub relay_transactions: bool,
    /// The UNIX epoch time of the last send.
    #[serde(rename = "lastsend")]
    pub last_send: i64,
    /// The UNIX epoch time of the last receive.
    #[serde(rename = "lastrecv")]
    pub last_received: i64,
    /// The UNIX epoch time of the last valid transaction received from this peer.
    pub last_transaction: i64,
    /// The UNIX epoch time of the last block received from this peer.
    pub last_block: i64,
    /// The total bytes sent.
    #[serde(rename = "bytessent")]
    pub bytes_sent: u64,
    /// The total bytes received.
    #[serde(rename = "bytesrecv")]
    pub bytes_received: u64,
    /// The UNIX epoch time of the connection.
    #[serde(rename = "conntime")]
    pub connection_time: i64,
    /// The time offset in seconds.
    #[serde(rename = "timeoffset")]
    pub time_offset: i64,
    /// The last ping time in seconds, if any.
    #[serde(rename = "pingtime")]
    pub ping_time: Option<f64>,
    /// The minimum observed ping time in seconds, if any.
    #[serde(rename = "minping")]
    pub minimum_ping: Option<f64>,
    /// The duration in seconds of an outstanding ping (if non-zero).
    #[serde(rename = "pingwait")]
    pub ping_wait: Option<f64>,
    /// The peer version, such as 70001.
    pub version: u32,
    /// The string version.
    #[serde(rename = "subver")]
    pub subversion: String,
    /// Inbound (true) or Outbound (false).
    pub inbound: bool,
    /// Whether we selected peer as (compact blocks) high-bandwidth peer.
    pub bip152_hb_to: bool,
    /// Whether peer selected us as (compact blocks) high-bandwidth peer.
    pub bip152_hb_from: bool,
    /// (DEPRECATED, returned only if config option -deprecatedrpc=startingheight is passed) The starting height (block) of the peer.
    #[serde(rename = "startingheight")]
    pub starting_height: Option<i64>,
    /// The current height of header pre-synchronization with this peer, or -1 if no low-work sync is
    /// in progress.
    pub presynced_headers: Option<i64>,
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
    /// How many txs we have queued to announce to this peer.
    pub inv_to_send: u64,
    /// Mempool sequence number of this peer's last INV.
    pub last_inv_sequence: u64,
    /// Type of connection.
    pub connection_type: Option<ConnectionType>,
    /// Type of transport protocol.
    pub transport_protocol_type: TransportProtocolType,
    /// The session ID for this connection, or "" if there is none ("v2" transport protocol only).
    pub session_id: String,
}

/// Type of connection. Part of `getpeerinfo`.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ConnectionType {
    /// Default automatic connections.
    OutboundFullRelay,
    /// Does not relay transactions or addresses.
    BlockRelayOnly,
    /// Initiated by the peer.
    Inbound,
    /// Added via addnode RPC or -addnode/-connect configuration options.
    Manual,
    /// Short-lived automatic connection for soliciting addresses.
    AddrFetch,
    /// Short-lived automatic connection for testing addresses.
    Feeler,
    /// Short-lived automatic connection for broadcasting privacy-sensitive transactions.
    PrivateBroadcast,
}

/// Type of transport protocol. Part of `getpeerinfo`.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TransportProtocolType {
    /// Peer could be v1 or v2.
    Detecting,
    /// Plaintext transport protocol.
    V1,
    /// BIP324 encrypted transport protocol.
    V2,
}
