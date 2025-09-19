// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v0.21` - util.
//!
//! Types for methods found under the `== Util ==` section of the API docs.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Result of JSON-RPC method `getindexinfo`.
///
/// > Returns the status of one or all available indices currently running in the node.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetIndexInfo(pub BTreeMap<String, GetIndexInfoName>);

/// Index info details. Part of `getindexinfo`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetIndexInfoName {
    /// Whether the index is synced or not.
    pub synced: bool,
    /// The block height to which the index is synced.
    pub best_block_height: u32,
}
