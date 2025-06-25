// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v0.17` - control.
//!
//! Types for methods found under the `== Control ==` section of the API docs.

use alloc::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Result of JSON-RPC method `getmemoryinfo`.
///
/// We only support the default "stats" mode.
///
/// > getmemoryinfo ("mode")
///
/// > Returns an object containing information about memory usage.
///
/// > Arguments:
/// > 1. "mode" determines what kind of information is returned. This argument is optional, the default mode is "stats".
/// >   - "stats" returns general statistics about memory usage in the daemon.
/// >   - "mallocinfo" returns an XML string describing low-level heap state (only available if compiled with glibc 2.10+).
// This just mimics the map returned by my instance of Core `v0.17`, I don't know how
// to handle other map values or if they exist?
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetMemoryInfoStats(pub BTreeMap<String, Locked>);

/// Information about locked memory manager.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Locked {
    /// Number of bytes used.
    pub used: u64,
    /// Number of bytes available in current arenas.
    pub free: u64,
    /// Total number of bytes managed.
    pub total: u64,
    /// Amount of bytes that succeeded locking.
    ///
    /// If this number is smaller than total, locking pages failed at some point and key data could
    /// be swapped to disk.
    pub locked: u64,
    /// Number allocated chunks.
    pub chunks_used: u64,
    /// Number unused chunks.
    pub chunks_free: u64,
}

/// Result of JSON-RPC method `logging`.
///
/// > logging ( `<include>` `<exclude>` )
///
/// > Gets and sets the logging configuration.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Logging {
    pub addrman: bool,
    pub bench: bool,
    pub blockstorage: Option<bool>, // v23 and later only
    pub cmpctblock: bool,
    pub coindb: bool,
    pub db: Option<bool>, // v22 and before only
    pub estimatefee: bool,
    pub http: bool,
    pub i2p: Option<bool>, // v23 and later only
    pub ipc: Option<bool>, // v23 and later only
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
    pub scan: Option<bool>, // v25 and later only
    pub selectcoins: bool,
    pub tor: bool,
    pub txpackages: Option<bool>,       // v26 and later only
    pub txreconciliation: Option<bool>, // v25 and later only
    pub util: Option<bool>,             // v23 and later only
    pub validation: Option<bool>,       // v23 and later only
    pub walletdb: Option<bool>,         // v23 and later only
    pub zmq: bool,
}
