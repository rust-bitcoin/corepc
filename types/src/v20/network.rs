// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v0.20.2` - network.
//!
//! Types for methods found under the `== Network ==` section of the API docs.

use serde::{Deserialize, Serialize};

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
    /// The UNIX epoch time the ban was expires.
    pub banned_until: u32,
    /// The UNIX epoch time the ban was created.
    pub ban_created: u32,
}
