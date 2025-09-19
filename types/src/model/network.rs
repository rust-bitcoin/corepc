// SPDX-License-Identifier: CC0-1.0

//! Types for methods found under the `== Network ==` section of the API docs.
//!
//! These structs model the types returned by the JSON-RPC API but have concrete types
//! and are not specific to a specific version of Bitcoin Core.

use bitcoin::FeeRate;
use serde::{Deserialize, Serialize};

/// Models the result of JSON-RPC method `getnetworkinfo`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetNetworkInfo {
    /// The server version.
    pub version: usize,
    /// The server subversion string.
    pub subversion: String,
    /// The protocol version.
    pub protocol_version: usize,
    /// The services we offer to the network (hex string).
    pub local_services: String,
    /// The services we offer to the network. v0.19 and later only.
    pub local_services_names: Option<Vec<String>>,
    /// `true` if transaction relay is requested from peers.
    pub local_relay: bool,
    /// The time offset.
    pub time_offset: isize,
    /// The total number of connections.
    pub connections: usize,
    /// The number of inbound connections. v28 and later only.
    pub connections_in: Option<usize>,
    /// The number of outbound connections. v28 and later only.
    pub connections_out: Option<usize>,
    /// Whether p2p networking is enabled.
    pub network_active: bool,
    /// Information per network.
    pub networks: Vec<GetNetworkInfoNetwork>,
    /// Minimum relay fee rate for transactions.
    pub relay_fee: Option<FeeRate>, // `Some` if parsing succeeds.
    /// Minimum fee rate increment for mempool limiting or replacement.
    pub incremental_fee: Option<FeeRate>, // `Some` if parsing succeeds.
    /// List of local addresses.
    pub local_addresses: Vec<GetNetworkInfoAddress>,
    /// Any network and blockchain warnings.
    pub warnings: Vec<String>,
}

/// Information per network. Part of `getnetworkinfo`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
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

/// Local address info. Part of `getnetworkinfo`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetNetworkInfoAddress {
    /// Network address.
    pub address: String,
    /// Network port.
    pub port: u16,
    /// Relative score.
    pub score: u32,
}
