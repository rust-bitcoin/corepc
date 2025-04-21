// SPDX-License-Identifier: CC0-1.0

//! JSON-RPC clients for testing against specific versions of Bitcoin Core.

mod error;
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

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use bitcoin::Txid;
use serde::{Deserialize, Serialize};

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

/// Defines a `jsonrpc::Client` using `minreq`.
#[macro_export]
macro_rules! define_jsonrpc_minreq_client {
    ($version:literal) => {
        use std::fmt;

        use $crate::client_sync::{log_response, Auth, Result};
        use $crate::client_sync::error::Error;

        /// Client implements a JSON-RPC client for the Bitcoin Core daemon or compatible APIs.
        pub struct Client {
            inner: jsonrpc::client::Client,
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
                let transport = jsonrpc::http::minreq_http::Builder::new()
                    .url(url)
                    .expect("jsonrpc v0.18, this function does not error")
                    .build();
                let inner = jsonrpc::client::Client::with_transport(transport);

                Self { inner }
            }

            /// Creates a client to a bitcoind JSON-RPC server with authentication.
            pub fn new_with_auth(url: &str, auth: Auth) -> Result<Self> {
                if matches!(auth, Auth::None) {
                    return Err(Error::MissingUserPassword);
                }
                let (user, pass) = auth.get_user_pass()?;

                let transport = jsonrpc::http::minreq_http::Builder::new()
                    .url(url)
                    .expect("jsonrpc v0.18, this function does not error")
                    .basic_auth(user.unwrap(), pass)
                    .build();
                let inner = jsonrpc::client::Client::with_transport(transport);

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

/// Shorthand for converting an `Option` into an `Option<serde_json::Value>`.
#[allow(dead_code)] // TODO: Remove this if unused still when we are done.
fn opt_into_json<T>(opt: Option<T>) -> Result<serde_json::Value>
where
    T: serde::ser::Serialize,
{
    match opt {
        Some(val) => Ok(into_json(val)?),
        None => Ok(serde_json::Value::Null),
    }
}

/// Shorthand for `serde_json::Value::Null`.
#[allow(dead_code)] // TODO: Remove this if unused still when we are done.
fn null() -> serde_json::Value { serde_json::Value::Null }

/// Shorthand for an empty `serde_json::Value` array.
#[allow(dead_code)] // TODO: Remove this if unused still when we are done.
fn empty_arr() -> serde_json::Value { serde_json::Value::Array(vec![]) }

/// Shorthand for an empty `serde_json` object.
#[allow(dead_code)] // TODO: Remove this if unused still when we are done.
fn empty_obj() -> serde_json::Value { serde_json::Value::Object(Default::default()) }

/// Convert a possible-null result into an `Option`.
#[allow(dead_code)] // TODO: Remove this if unused still when we are done.
fn opt_result<T: for<'a> serde::de::Deserialize<'a>>(
    result: serde_json::Value,
) -> Result<Option<T>> {
    if result == serde_json::Value::Null {
        Ok(None)
    } else {
        Ok(serde_json::from_value(result)?)
    }
}

/// Helper to log an RPC response.
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

/// Input used as parameter to `create_raw_transaction`.
#[derive(Debug, Serialize)]
pub struct Input {
    /// The txid of the transaction that contains the UTXO.
    pub txid: bitcoin::Txid,
    /// The vout for the UTXO.
    pub vout: u64,
    /// Sequence number if needed.
    pub sequence: Option<bitcoin::Sequence>,
}

/// An element in the `inputs` argument of method `walletcreatefundedpsbt`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct WalletCreateFundedPsbtInput {
    txid: Txid,
    vout: u32,
}

/// Arg for the `getblocktemplate` method.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TemplateRequest {
    /// A list of strings.
    pub rules: Vec<TemplateRules>,
}

/// Client side supported softfork deployment.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TemplateRules {
    /// SegWit v0 supported.
    Segwit,
    /// Signet supported.
    Signet,
    /// CSV supported.
    Csv,
    /// Taproot supported.
    Taproot,
}

/// Args for the `addnode` method
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AddNodeCommand {
    Add,
    Remove,
    OneTry,
}

/// Args for the `setban` method
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SetBanCommand {
    Add,
    Remove,
}

/// Args for the `lockunspent` method
#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct LockUnspentOutput {
    /// The transaction id
    pub txid: Txid,
    /// The output number
    pub vout: u32,
}

/// Args for the `scantxoutset`
///
/// Represents the action for the `scantxoutset` RPC call.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ScanAction {
    Start,
    Abort,
    Status,
}

/// Represents the range for HD descriptor scanning (handles n or [n, n]).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum ScanRange {
    /// Represents the end index (beginning is 0).
    Single(u64),
    /// Array represents [begin, end] indexes
    Range([u64; 2]),
}

// Helper function for serde default
fn default_scan_range() -> ScanRange {
    // Default range is 1000 as per Bitcoin Core docs
    ScanRange::Single(1000)
}

/// Represents a scan object for scantxoutset (descriptor string or object).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum ScanObject {
    /// Plain descriptor string
    Descriptor(String),
    /// Object containing descriptor and optional range
    WithRange {
        desc: String,
        #[serde(default = "default_scan_range")]
        range: ScanRange,
    },
}

/// Args for the `importmulti`
///
/// Represents the scriptPubKey field in an importmulti request.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum ImportMultiScriptPubKey {
    /// Script hex string
    Script(String),
    /// Address object
    Address { address: String },
}

/// Represents the timestamp field in an importmulti request.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum ImportMultiTimestamp {
    /// Use current blockchain time
    Now(String),
    /// Specific UNIX epoch time
    Time(u64),
}

impl Default for ImportMultiTimestamp {
    fn default() -> Self { ImportMultiTimestamp::Now("now".to_string()) }
}

/// Represents a single request object within the importmulti call.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct ImportMultiRequest {
    /// Descriptor to import (optional, mutually exclusive with scriptPubKey/address etc.) (v18+)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// ScriptPubKey or address object (required unless desc is provided)
    #[serde(rename = "scriptPubKey", skip_serializing_if = "Option::is_none")]
    pub script_pub_key: Option<ImportMultiScriptPubKey>,
    /// Creation time of the key
    pub timestamp: ImportMultiTimestamp,
    /// Redeem script (P2SH/P2SH-P2WSH only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redeemscript: Option<String>,
    /// Witness script (P2WSH/P2SH-P2WSH only) (v18+)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witnessscript: Option<String>,
    /// Pubkeys to import (cannot be used with descriptor)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pubkeys: Vec<String>,
    /// Private keys to import (WIF format)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keys: Vec<String>,
    /// Range for ranged descriptors (v18+)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<ScanRange>,
    /// Treat matching outputs as change
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal: Option<bool>,
    /// Treat matching outputs as watchonly
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watchonly: Option<bool>,
    /// Label for address (use "" for default)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Add pubkeys to keypool (only when private keys disabled) (v18+)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keypool: Option<bool>,
}

/// Represents the optional options object for importmulti
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
/// Rescan after import (default true)
pub struct ImportMultiOptions {
    /// Rescan after import (default true)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rescan: Option<bool>,
}
