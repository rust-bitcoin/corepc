// SPDX-License-Identifier: CC0-1.0

//! Auto-generated types for Bitcoin Core `30` - control.
//!
//! Produced by `codegen`. Do not edit by hand, re-run
//! `just codegen` to regenerate. Holds the RPC return types for this section; the
//! `*Options` request structs live with the call surface in `corepc-client`.

#![allow(non_camel_case_types, non_snake_case, clippy::large_enum_variant)]

use serde::{Deserialize, Serialize};

/// Result of the JSON-RPC method `getmemoryinfo`.
///
/// > getmemoryinfo
/// >
/// > Returns an object containing information about memory usage.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum GetMemoryInfo {
    Object(GetMemoryInfoVariant0),
    Text(String),
}

/// mode "stats"
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMemoryInfoVariant0 {
    /// Information about locked memory manager
    pub locked: GetMemoryInfoVariant0Locked,
}

/// Information about locked memory manager
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMemoryInfoVariant0Locked {
    /// Number unused chunks
    pub chunks_free: u64,
    /// Number allocated chunks
    pub chunks_used: u64,
    /// Number of bytes available in current arenas
    pub free: u64,
    /// Amount of bytes that succeeded locking. If this number is smaller than total, locking pages failed at some point and key data could be swapped to disk.
    pub locked: u64,
    /// Total number of bytes managed
    pub total: u64,
    /// Number of bytes used
    pub used: u64,
}

/// Returns an OpenRPC document for currently available RPC commands.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetOpenRpcInfo {
    /// Metadata about this JSON-RPC interface.
    pub info: GetOpenRpcInfoInfo,
    /// Documented RPC methods.
    pub methods: Vec<GetOpenRpcInfoMethodsItem>,
    /// OpenRPC specification version.
    #[serde(rename = "openrpc")]
    pub open_rpc: String,
}

/// Metadata about this JSON-RPC interface.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetOpenRpcInfoInfo {
    /// API description.
    pub description: String,
    /// API title.
    pub title: String,
    /// Bitcoin Core version string.
    pub version: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetOpenRpcInfoMethodsItem {
    /// Method description.
    pub description: String,
    /// Method name.
    pub name: String,
    /// Method parameters.
    pub params: Vec<GetOpenRpcInfoMethodsItemParamsItem>,
    /// Method result.
    pub result: GetOpenRpcInfoMethodsItemResult,
    /// RPC category.
    #[serde(rename = "x-bitcoin-category")]
    pub x_bitcoin_category: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetOpenRpcInfoMethodsItemParamsItem {
    /// Parameter description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Parameter name.
    pub name: String,
    /// Whether the parameter is required.
    pub required: bool,
    /// JSON Schema for the parameter.
    pub schema: serde_json::Value,
    /// Alternative parameter names.
    #[serde(rename = "x-bitcoin-aliases", skip_serializing_if = "Option::is_none")]
    pub x_bitcoin_aliases: Option<Vec<String>>,
    /// Whether the parameter can also be passed positionally.
    #[serde(rename = "x-bitcoin-also-positional", skip_serializing_if = "Option::is_none")]
    pub x_bitcoin_also_positional: Option<bool>,
    /// Whether the parameter is retained only for compatibility.
    #[serde(rename = "x-bitcoin-placeholder", skip_serializing_if = "Option::is_none")]
    pub x_bitcoin_placeholder: Option<bool>,
}

/// Method result.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetOpenRpcInfoMethodsItemResult {
    /// Result name.
    pub name: String,
    /// JSON Schema for the result.
    pub schema: serde_json::Value,
}

/// Returns details of the RPC server.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRpcInfo {
    /// All active commands
    pub active_commands: Vec<GetRpcInfoActiveCommandsItem>,
    /// The complete file path to the debug log
    #[serde(rename = "logpath")]
    pub log_path: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRpcInfoActiveCommandsItem {
    /// The running time in microseconds
    pub duration: i64,
    /// The name of the RPC command
    pub method: String,
}

/// Result of the JSON-RPC method `help`.
///
/// > help
/// >
/// > List all commands, or get help for a specified command.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Help(pub String);

impl std::ops::Deref for Help {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.0 }
}

/// Result of the JSON-RPC method `logging`.
///
/// > logging
/// >
/// > Gets and sets the logging configuration.
/// > When called without an argument, returns the list of categories with status that are currently being debug logged or not.
/// > When called with arguments, adds or removes categories from debug logging and return the lists above.
/// > The arguments are evaluated in order "include", "exclude".
/// > If an item is both included and excluded, it will thus end up being excluded.
/// > The valid logging categories are: addrman, bench, blockstorage, cmpctblock, coindb, estimatefee, http, i2p, ipc, leveldb, libevent, mempool, mempoolrej, net, proxy, prune, qt, rand, reindex, rpc, scan, selectcoins, tor, txpackages, txreconciliation, validation, walletdb, zmq
/// > In addition, the following are available as category names with special meanings:
/// >   - "all",  "1" : represent all logging categories.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Logging(
    /// Map entries
    pub std::collections::BTreeMap<String, bool>,
);

/// Result of the JSON-RPC method `stop`.
///
/// > stop
/// >
/// > Request a graceful shutdown of Bitcoin Core.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Stop(pub String);

impl std::ops::Deref for Stop {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.0 }
}

/// Result of the JSON-RPC method `uptime`.
///
/// > uptime
/// >
/// > Returns the total uptime of the server.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Uptime(pub i64);

impl std::ops::Deref for Uptime {
    type Target = i64;
    fn deref(&self) -> &Self::Target { &self.0 }
}
