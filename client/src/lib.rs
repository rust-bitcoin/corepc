// SPDX-License-Identifier: CC0-1.0

//! Support for connecting to Bitcoin Core via JSON-RPC.

/// Re-export the `rust-bitcoin` crate.
pub extern crate bitcoin;

/// Re-export the `corepc-types` crate.
pub extern crate types;

#[cfg(feature = "client-sync")]
#[macro_use]
pub mod client_sync;

#[cfg(feature = "client-sync")]
use crate::client_sync::Result;

/// Shorthand for converting a variable into a `serde_json::Value`.
#[cfg(feature = "client-sync")]
fn into_json<T>(val: T) -> Result<serde_json::Value>
where
    T: serde::ser::Serialize,
{
    Ok(serde_json::to_value(val)?)
}

/// Helper to log an RPC response.
#[cfg(feature = "client-sync")]
fn log_response(method: &str, resp: &Result<jsonrpc::Response>) {
    use log::Level::{Debug, Trace, Warn};

    if log::log_enabled!(Warn) || log::log_enabled!(Debug) || log::log_enabled!(Trace) {
        match resp {
            Err(ref e) =>
                if log::log_enabled!(Debug) {
                    log::debug!(target: "corepc", "error: {}: {:?}", method, e);
                },
            Ok(ref resp) =>
                if let Some(ref e) = resp.error {
                    if log::log_enabled!(Debug) {
                        log::debug!(target: "corepc", "response error for {}: {:?}", method, e);
                    }
                } else if log::log_enabled!(Trace) {
                    let def =
                        serde_json::value::to_raw_value(&serde_json::value::Value::Null).unwrap();
                    let result = resp.result.as_ref().unwrap_or(&def);
                    log::trace!(target: "corepc", "response for {}: {}", method, result);
                },
        }
    }
}
