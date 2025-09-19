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
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMemoryInfoStats(pub BTreeMap<String, Locked>);

/// Information about locked memory manager. Part of `getmemoryinfo`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
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
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Logging {
    pub addrman: bool,
    pub bench: bool,
    pub cmpctblock: bool,
    pub coindb: bool,
    pub db: bool,
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
    pub zmq: bool,
}
