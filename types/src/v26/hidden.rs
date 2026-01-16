// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v26` - hidden.
//!
//! Types for methods that are excluded from the API docs by default.

use alloc::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Result of JSON-RPC method `getrawaddrman`.
///
/// > getrawaddrman
/// >
/// > EXPERIMENTAL warning: this call may be changed in future releases.
/// >
/// > Returns information on all address manager entries for the new and tried tables.
///
/// This is a hidden RPC, useful for testing and development.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawAddrMan {
    /// Addresses in the "new" table (potential peers discovered but not yet connected to).
    pub new: BTreeMap<String, RawAddrManEntry>,
    /// Addresses in the "tried" table (peers successfully connected to in the past).
    pub tried: BTreeMap<String, RawAddrManEntry>,
}

/// An entry in the address manager table. Part of `getrawaddrman`.
///
/// The key in the parent map is formatted as "bucket/position" indicating the
/// location in the relevant address manager table.
///
/// Field order matches Bitcoin Core's RPC response definition.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct RawAddrManEntry {
    /// The address of the node.
    pub address: String,
    /// The port number of the node.
    pub port: u16,
    /// The network (ipv4, ipv6, onion, i2p, cjdns) of the address.
    pub network: String,
    /// The services offered by the node.
    pub services: u64,
    /// The UNIX epoch time when the node was last seen.
    pub time: i64,
    /// The address that relayed the address to us.
    pub source: String,
    /// The network (ipv4, ipv6, onion, i2p, cjdns) of the source address.
    pub source_network: String,
}
