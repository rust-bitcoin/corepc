// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v0.18` - zmq.
//!
//! Types for methods found under the `== Zmq ==` section of the API docs.

use serde::{Deserialize, Serialize};

/// Result of JSON-RPC method `getzmqnotifications`.
///
///> getzmqnotifications
///>
///> Returns information about the active ZeroMQ notifications.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetZmqNotifications {
    /// Type of notification.
    #[serde(rename = "type")]
    pub type_: String,
    /// Address of the publisher.
    pub address: String,
    /// Outbound message high water mark.
    pub hwm: u64,
}
