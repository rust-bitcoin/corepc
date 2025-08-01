// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v27` - network.
//!
//! Types for methods found under the `== Network ==` section of the API docs.

mod into;

use serde::{Deserialize, Serialize};

/// Result of JSON-RPC method `getnodeaddresses`.
///
/// > getnodeaddresses ( count "network" )
/// >
/// > Return known addresses, after filtering for quality and recency.
/// > These can potentially be used to find new peers in the network.
/// > The total number of addresses known to the node may be higher.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GetNodeAddresses(pub Vec<NodeAddress>);

/// An item from the list returned by the JSON-RPC method `getnodeaddresses`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
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
