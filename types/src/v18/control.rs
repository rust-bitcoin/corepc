// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v0.18` - control.
//!
//! Types for methods found under the `== Control ==` section of the API docs.

use serde::{Deserialize, Serialize};

/// Result of JSON-RPC method `getrpcinfo`.
///
/// > getrpcinfo
/// >
/// > Returns details of the RPC server.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRpcInfo {
    active_commands: Vec<ActiveCommand>,
}

/// Information about an active command. Part of `getrpcinfo`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ActiveCommand {
    /// The name of the RPC command.
    pub method: String,
    /// The running time in microseconds.
    pub duration: u64,
}
