// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v0.19` - control.
//!
//! Types for methods found under the `== Control ==` section of the API docs.

use serde::{Deserialize, Serialize};

use super::ActiveCommand;

/// Result of JSON-RPC method `getrpcinfo`.
///
/// > getrpcinfo
/// >
/// > Returns details of the RPC server.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRpcInfo {
    /// All active commands
    pub active_commands: Vec<ActiveCommand>,
    /// The complete file path to the debug log
    #[serde(rename = "logpath")]
    pub log_path: String,
}
