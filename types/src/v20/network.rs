// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v0.17` - network.
//!
//! Types for methods found under the `== Network ==` section of the API docs.
//!
/// These types do not implement `into_model` because no `rust-bitcoin` types needed yet.
use serde::Deserialize;

/// Result of JSON-RPC method `setnetworkactive`.
///
/// > setnetworkactive
/// >
/// > Returns null (json null)
/// >
/// > Arguments:
/// > 1. state (boolean, required) true to enable networking, false to disable
/// >
/// > Returns true|false (boolean) The value that was passed in
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetNetworkActive(pub bool);
