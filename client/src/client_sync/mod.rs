// SPDX-License-Identifier: CC0-1.0

//! JSON-RPC clients for testing against specific versions of Bitcoin Core.

mod error;
mod jsonrpc_sync;
pub mod v17;
pub mod v18;
pub mod v19;
pub mod v20;
pub mod v21;
pub mod v22;
pub mod v23;
pub mod v24;
pub mod v25;
pub mod v26;
pub mod v27;
pub mod v28;
pub mod v29;
pub mod v30;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub use crate::client_sync::error::Error;

/// Crate-specific Result type.
///
/// Shorthand for `std::result::Result` with our crate-specific [`Error`] type.
pub type Result<T> = std::result::Result<T, Error>;

/// The different authentication methods for the client.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Auth {
    None,
    UserPass(String, String),
    CookieFile(PathBuf),
}

impl Auth {
    /// Convert into the arguments that the JSON-RPC client needs.
    pub fn get_user_pass(self) -> Result<(Option<String>, Option<String>)> {
        match self {
            Auth::None => Ok((None, None)),
            Auth::UserPass(u, p) => Ok((Some(u), Some(p))),
            Auth::CookieFile(path) => {
                let line = BufReader::new(File::open(path)?)
                    .lines()
                    .next()
                    .ok_or(Error::InvalidCookieFile)??;
                let colon = line.find(':').ok_or(Error::InvalidCookieFile)?;
                Ok((Some(line[..colon].into()), Some(line[colon + 1..].into())))
            }
        }
    }
}

/// Defines a JSON-RPC client using `bitreq`.
#[macro_export]
macro_rules! define_jsonrpc_bitreq_client {
    ($version:literal) => {
        use std::fmt;

        use $crate::client_sync::{log_response, Auth, Result};
        use $crate::client_sync::error::Error;
        use $crate::client_sync::jsonrpc_sync;

        /// Client implements a JSON-RPC client for the Bitcoin Core daemon or compatible APIs.
        pub struct Client {
            inner: jsonrpc_sync::Client,
        }

        impl fmt::Debug for Client {
            fn fmt(&self, f: &mut fmt::Formatter) -> core::fmt::Result {
                write!(
                    f,
                    "corepc_client::client_sync::{}::Client({:?})", $version, self.inner
                )
            }
        }

        impl Client {
            /// Creates a client to a bitcoind JSON-RPC server without authentication.
            pub fn new(url: &str) -> Self {
                let transport = jsonrpc_sync::http::bitreq_http::Builder::new()
                    .url(url)
                    .expect("jsonrpc v0.19, this function does not error")
                    .timeout(std::time::Duration::from_secs(60))
                    .build();
                let inner = jsonrpc_sync::Client::with_transport(transport);

                Self { inner }
            }

            /// Creates a client to a bitcoind JSON-RPC server with authentication.
            pub fn new_with_auth(url: &str, auth: Auth) -> Result<Self> {
                if matches!(auth, Auth::None) {
                    return Err(Error::MissingUserPassword);
                }
                let (user, pass) = auth.get_user_pass()?;

                let transport = jsonrpc_sync::http::bitreq_http::Builder::new()
                    .url(url)
                    .expect("jsonrpc v0.19, this function does not error")
                    .timeout(std::time::Duration::from_secs(60))
                    .basic_auth(user.unwrap(), pass)
                    .build();
                let inner = jsonrpc_sync::Client::with_transport(transport);

                Ok(Self { inner })
            }

            /// Call an RPC `method` with given `args` list.
            pub fn call<T: for<'a> serde::de::Deserialize<'a>>(
                &self,
                method: &str,
                args: &[serde_json::Value],
            ) -> Result<T> {
                let raw = serde_json::value::to_raw_value(args)?;
                let req = self.inner.build_request(&method, Some(&*raw));
                if log::log_enabled!(log::Level::Debug) {
                    log::debug!(target: "corepc", "request: {} {}", method, serde_json::Value::from(args));
                }

                let resp = self.inner.send_request(req).map_err(Error::from);
                log_response(method, &resp);
                Ok(resp?.result()?)
            }
        }
    }
}

/// Implements the `check_expected_server_version()` on `Client`.
///
/// Requires `Client` to be in scope and implement `server_version()`.
/// See and/or use `impl_client_v17__getnetworkinfo`.
///
/// # Parameters
///
/// - `$expected_versions`: An vector of expected server versions e.g., `[230100, 230200]`.
#[macro_export]
macro_rules! impl_client_check_expected_server_version {
    ($expected_versions:expr) => {
        impl Client {
            /// Checks that the JSON-RPC endpoint is for a `bitcoind` instance with the expected version.
            pub fn check_expected_server_version(&self) -> Result<()> {
                let server_version = self.server_version()?;
                if !$expected_versions.contains(&server_version) {
                    return Err($crate::client_sync::error::UnexpectedServerVersionError {
                        got: server_version,
                        expected: $expected_versions.to_vec(),
                    })?;
                }
                Ok(())
            }
        }
    };
}

/// Shorthand for converting a variable into a `serde_json::Value`.
fn into_json<T>(val: T) -> Result<serde_json::Value>
where
    T: serde::ser::Serialize,
{
    Ok(serde_json::to_value(val)?)
}

/// Helper to log an RPC response.
fn log_response(method: &str, resp: &Result<jsonrpc_sync::Response>) {
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
