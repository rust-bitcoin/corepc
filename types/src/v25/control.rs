// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v25` - control.
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
    pub blockstorage: bool, // v23 and later only
    pub cmpctblock: bool,
    pub coindb: bool,
    pub estimatefee: bool,
    pub http: bool,
    pub i2p: bool, // v23 and later only
    pub ipc: bool, // v23 and later only
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
    pub scan: bool, // v25 and later only
    pub selectcoins: bool,
    pub tor: bool,
    pub txreconciliation: bool, // v25 and later only
    pub util: bool,             // v23 to v27 only
    pub validation: bool,       // v23 and later only
    pub walletdb: bool,         // v23 and later only
    pub zmq: bool,
}
