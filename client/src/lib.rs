// SPDX-License-Identifier: CC0-1.0

//! Support for connecting to Bitcoin Core via JSON-RPC.

/// Re-export the `rust-bitcoin` crate.
pub extern crate bitcoin;

/// Re-export the `corepc-types` crate.
pub extern crate types;

#[cfg(feature = "client-sync")]
#[macro_use]
pub mod client_sync;

/// Helper to log an RPC response.
#[cfg(any(feature = "client-sync", feature = "client-async"))]
pub(crate) fn log_response<E: std::fmt::Debug>(
    method: &str,
    resp: &std::result::Result<jsonrpc::Response, E>,
) {
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
                    if let Ok(def) =
                        serde_json::value::to_raw_value(&serde_json::value::Value::Null)
                    {
                        let result = resp.result.as_ref().unwrap_or(&def);
                        log::trace!(target: "corepc", "response for {}: {}", method, result);
                    }
                },
        }
    }
}

/// Shorthand for converting a variable into a `serde_json::Value`.
#[cfg(any(feature = "client-sync", feature = "client-async"))]
pub(crate) fn into_json<T>(val: T) -> Result<serde_json::Value, serde_json::Error>
where
    T: serde::ser::Serialize,
{
    serde_json::to_value(val)
}
