// SPDX-License-Identifier: CC0-1.0

//! Auto-generated method wrappers for Bitcoin Core `30` - control.
//!
//! Produced by `codegen`. Do not edit by hand, re-run
//! `just codegen` to regenerate. Defines the `*Options` request structs these methods
//! consume; the response types live in the `corepc-types` crate
//! (`types::v30::generated`).

#![allow(unused_imports, clippy::needless_pass_by_value, clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use serde_json::json;

use types::v30::generated::{
    GetMemoryInfo,
    GetOpenRpcInfo,
    GetRpcInfo,
    Help,
    Logging,
    Stop,
    Uptime,
};

use crate::client_async::error::Result;
use crate::client_async::Client;

/// Optional parameters for the `getmemoryinfo` JSON-RPC method (consumed by `Client::get_memory_info_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMemoryInfoOptions {
    /// determines what kind of information is returned.
    ///   - "stats" returns general statistics about memory usage in the daemon.
    ///   - "mallocinfo" returns an XML string describing low-level heap state (only available if compiled with glibc).
    pub mode: Option<String>,
}

/// Optional parameters for the `help` JSON-RPC method (consumed by `Client::help_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HelpOptions {
    /// The command to get help on
    pub command: Option<String>,
}

/// Optional parameters for the `logging` JSON-RPC method (consumed by `Client::logging_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoggingOptions {
    /// The categories to add to debug logging
    pub include: Option<Vec<String>>,
    /// The categories to remove from debug logging
    pub exclude: Option<Vec<String>>,
}

impl Client {
    /// `getmemoryinfo` with required arguments only.
    ///
    /// Returns an object containing information about memory usage.
    pub async fn get_memory_info(&self) -> Result<GetMemoryInfo> {
        self.call_raw("getmemoryinfo", &[(); 0] as &[()]).await
    }

    /// `getmemoryinfo` with all optional arguments via [`GetMemoryInfoOptions`].
    ///
    /// Returns an object containing information about memory usage.
    pub async fn get_memory_info_with(&self, opts: GetMemoryInfoOptions) -> Result<GetMemoryInfo> {
        self.call_raw("getmemoryinfo", &[json!(opts.mode)]).await
    }

    /// `getopenrpcinfo` with required arguments only.
    ///
    /// Returns an OpenRPC document for currently available RPC commands.
    pub async fn get_open_rpc_info(&self) -> Result<GetOpenRpcInfo> {
        self.call_raw("getopenrpcinfo", &[(); 0] as &[()]).await
    }

    /// `getrpcinfo` with required arguments only.
    ///
    /// Returns details of the RPC server.
    pub async fn get_rpc_info(&self) -> Result<GetRpcInfo> {
        self.call_raw("getrpcinfo", &[(); 0] as &[()]).await
    }

    /// `help` with required arguments only.
    ///
    /// List all commands, or get help for a specified command.
    pub async fn help(&self) -> Result<Help> {
        self.call_raw("help", &[(); 0] as &[()]).await
    }

    /// `help` with all optional arguments via [`HelpOptions`].
    ///
    /// List all commands, or get help for a specified command.
    pub async fn help_with(&self, opts: HelpOptions) -> Result<Help> {
        self.call_raw("help", &[json!(opts.command)]).await
    }

    /// `logging` with required arguments only.
    ///
    /// Gets and sets the logging configuration.
    /// When called without an argument, returns the list of categories with status that are currently being debug logged or not.
    /// When called with arguments, adds or removes categories from debug logging and return the lists above.
    /// The arguments are evaluated in order "include", "exclude".
    /// If an item is both included and excluded, it will thus end up being excluded.
    /// The valid logging categories are: addrman, bench, blockstorage, cmpctblock, coindb, estimatefee, http, i2p, ipc, leveldb, libevent, mempool, mempoolrej, net, proxy, prune, qt, rand, reindex, rpc, scan, selectcoins, tor, txpackages, txreconciliation, validation, walletdb, zmq
    /// In addition, the following are available as category names with special meanings:
    ///   - "all",  "1" : represent all logging categories.
    pub async fn logging(&self) -> Result<Logging> {
        self.call_raw("logging", &[(); 0] as &[()]).await
    }

    /// `logging` with all optional arguments via [`LoggingOptions`].
    ///
    /// Gets and sets the logging configuration.
    /// When called without an argument, returns the list of categories with status that are currently being debug logged or not.
    /// When called with arguments, adds or removes categories from debug logging and return the lists above.
    /// The arguments are evaluated in order "include", "exclude".
    /// If an item is both included and excluded, it will thus end up being excluded.
    /// The valid logging categories are: addrman, bench, blockstorage, cmpctblock, coindb, estimatefee, http, i2p, ipc, leveldb, libevent, mempool, mempoolrej, net, proxy, prune, qt, rand, reindex, rpc, scan, selectcoins, tor, txpackages, txreconciliation, validation, walletdb, zmq
    /// In addition, the following are available as category names with special meanings:
    ///   - "all",  "1" : represent all logging categories.
    pub async fn logging_with(&self, opts: LoggingOptions) -> Result<Logging> {
        self.call_raw("logging", &[json!(opts.include), json!(opts.exclude)]).await
    }

    /// `stop` with required arguments only.
    ///
    /// Request a graceful shutdown of Bitcoin Core.
    pub async fn stop(&self) -> Result<Stop> {
        self.call_raw("stop", &[(); 0] as &[()]).await
    }

    /// `uptime` with required arguments only.
    ///
    /// Returns the total uptime of the server.
    pub async fn uptime(&self) -> Result<Uptime> {
        self.call_raw("uptime", &[(); 0] as &[()]).await
    }

}
