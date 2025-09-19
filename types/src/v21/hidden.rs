// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v21` - hidden.
//!
//! Types for methods that are excluded from the API docs by default.

use serde::{Deserialize, Serialize};

/// Result of JSON-RPC method `addpeeraddress`.
///
/// > addpeeraddress "address" port
/// >
/// > Add the address of a potential peer to the address manager. This RPC is for testing only.
/// >
/// > Arguments:
/// > 1. address    (string, required) The IP address of the peer
/// > 2. port       (numeric, required) The port of the peer
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct AddPeerAddress {
    /// Whether the peer address was successfully added to the address manager.
    pub success: bool,
}
