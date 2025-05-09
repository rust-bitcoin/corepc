// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v0.18` - network.
//!
//! Types for methods found under the `== Network ==` section of the API docs.
//!
/// These types do not implement `into_model` because apart from fee rate there is no additional
/// `rust-bitcoin` types needed.
use serde::{Deserialize, Serialize};

/// Result of JSON-RPC method `getnodeaddresses`.
///
/// > getnodeaddresses ( count )
/// >
/// > Return known addresses which can potentially be used to find new nodes in the network.
/// >
/// > Arguments:
/// > 1. count    (numeric, optional, default=1) How many addresses to return. Limited to the smaller of 2500 or 23% of all known addresses.
/// >
/// > Result: Array of objects with time, services, address, and port.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetNodeAddresses(pub Vec<NodeAddress>);

/// An item from the list returned by the JSON-RPC method `getnodeaddresses`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct NodeAddress {
    /// Timestamp in seconds since epoch (Jan 1 1970 GMT) when the node was last seen.
    pub time: u64,
    /// The services offered.
    pub services: u64,
    /// The address of the node.
    pub address: String,
    /// The port of the node.
    pub port: u16,
}
