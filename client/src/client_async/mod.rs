// SPDX-License-Identifier: CC0-1.0

//! Async JSON-RPC client for Bitcoin Core v25 to v30.

mod error;
mod rpcs;

use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub use error::{Error, IntoModelError, UnexpectedServerVersionError};
pub use rpcs::BitcoinRpcs;

pub(crate) use crate::{into_json, log_response};

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
    /// Convert into the arguments that jsonrpc::Client needs.
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

/// Client implements an async JSON-RPC client for the Bitcoin Core daemon or compatible APIs.
pub struct Client {
    inner: jsonrpc::client_async::Client,
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> core::fmt::Result {
        write!(f, "corepc_client::client_async::Client({:?})", self.inner)
    }
}

impl Client {
    /// Creates a client to a bitcoind JSON-RPC server without authentication.
    pub fn new(url: &str) -> Self {
        let transport = jsonrpc::bitreq_http_async::Builder::new()
            .url(url)
            .expect("this function does not error")
            .timeout(std::time::Duration::from_secs(60))
            .build();
        let inner = jsonrpc::client_async::Client::with_transport(transport);

        Self { inner }
    }

    /// Creates a client to a bitcoind JSON-RPC server with authentication.
    pub fn new_with_auth(url: &str, auth: Auth) -> Result<Self> {
        if matches!(auth, Auth::None) {
            return Err(Error::MissingUserPassword);
        }
        let (user, pass) = auth.get_user_pass()?;
        let user = user.ok_or(Error::MissingUserPassword)?;
        let transport = jsonrpc::bitreq_http_async::Builder::new()
            .url(url)
            .expect("this function does not error")
            .timeout(std::time::Duration::from_secs(60))
            .basic_auth(user, pass)
            .build();
        let inner = jsonrpc::client_async::Client::with_transport(transport);

        Ok(Self { inner })
    }

    /// Call an RPC `method` with given `args` list.
    pub async fn call<T: for<'a> serde::de::Deserialize<'a>>(
        &self,
        method: &str,
        args: &[serde_json::Value],
    ) -> Result<T> {
        let raw = serde_json::value::to_raw_value(args)?;
        let req = self.inner.build_request(method, Some(&*raw));
        if log::log_enabled!(log::Level::Debug) {
            log::debug!(target: "corepc", "request: {} {}", method, serde_json::Value::from(args));
        }

        let resp = self.inner.send_request(req).await.map_err(Error::from);
        log_response(method, &resp);
        Ok(resp?.result()?)
    }
}
