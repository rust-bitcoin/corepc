// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v0.20` - control.
//!
//! Types for methods found under the `== Control ==` section of the API docs.

use serde::{Deserialize, Serialize};

/// Result of JSON-RPC method `logging`.
///
/// > logging ( `<include>` `<exclude>` )
///
/// > Gets and sets the logging configuration.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Logging {
    pub addrman: bool,
    pub bench: bool,
    pub cmpctblock: bool,
    pub coindb: bool,
    pub estimatefee: bool,
    pub http: bool,
    pub leveldb: bool,
    pub libevent: bool,
    pub mempool: bool,
    pub mempoolrej: bool,
    pub net: bool,
    pub prune: bool,
    pub proxy: bool,
    pub qt: bool,
    pub rand: bool,
    pub reindex: bool,
    pub rpc: bool,
    pub selectcoins: bool,
    pub tor: bool,
    pub validation: bool,
    pub walletdb: bool,
    pub zmq: bool,
}
