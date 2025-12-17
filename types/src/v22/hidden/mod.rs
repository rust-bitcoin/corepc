// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v22` - hidden.
//!
//! Types for methods that are excluded from the API docs by default.

mod into;

use serde::{Deserialize, Serialize};

/// Result of JSON-RPC method `addconnection`.
///
/// > addconnection "address" "connection_type"
/// >
/// > Open an outbound connection to a specified node.
/// > This RPC is for testing only.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct AddConnection {
    /// The address of the newly added connection.
    pub address: String,
    /// Type of connection.
    pub connection_type: String,
}
