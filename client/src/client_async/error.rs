// SPDX-License-Identifier: CC0-1.0

//! Async client error types.
//!
//! Async RPC methods return per-method typed errors (for example, [`GetBlockError`])
//! so callers can match method-specific `Model` conversion failures directly.
//!
//! Every method error exposes inherent helpers (`is_not_found_error`, `is_transport_error`,
//! `rpc_code`, `as_client_error`) for common JSON-RPC and transport inspection.

use core::convert::Infallible;
use std::{error, fmt, io};

/// The general error type for the async client.
///
/// This covers connecting, authenticating, and performing a JSON-RPC call. Each RPC method returns
/// its own error type (e.g. [`GetBlockError`]) which wraps this type in its `Rpc` variant.
#[derive(Debug)]
pub enum Error {
    /// A JSON-RPC error occurred (transport error or the node returned an error).
    JsonRpc(jsonrpc::error::Error),
    /// Serializing an argument or deserializing the response failed.
    Json(serde_json::error::Error),
    /// An I/O error occurred (e.g. reading the cookie file).
    Io(io::Error),
    /// The cookie file was invalid.
    InvalidCookieFile,
    /// The server version did not match what was expected.
    ServerVersion(UnexpectedServerVersionError),
    /// Missing user/password.
    MissingUserPassword,
}

impl From<jsonrpc::error::Error> for Error {
    fn from(e: jsonrpc::error::Error) -> Error { Error::JsonRpc(e) }
}

impl From<serde_json::error::Error> for Error {
    fn from(e: serde_json::error::Error) -> Error { Error::Json(e) }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error { Error::Io(e) }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;

        match *self {
            JsonRpc(ref e) => write!(f, "JSON-RPC error: {}", e),
            Json(ref e) => write!(f, "JSON error: {}", e),
            Io(ref e) => write!(f, "I/O error: {}", e),
            InvalidCookieFile => write!(f, "invalid cookie file"),
            ServerVersion(ref e) => write!(f, "server version: {}", e),
            MissingUserPassword => write!(f, "missing user and/or password"),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use Error::*;

        match *self {
            JsonRpc(ref e) => Some(e),
            Json(ref e) => Some(e),
            Io(ref e) => Some(e),
            ServerVersion(ref e) => Some(e),
            InvalidCookieFile | MissingUserPassword => None,
        }
    }
}

fn rpc_code_from_client_error(err: &Error) -> Option<i64> {
    match err {
        Error::JsonRpc(jsonrpc::error::Error::Rpc(rpc)) => Some(i64::from(rpc.code)),
        _ => None,
    }
}

fn is_jsonrpc_not_found_error(err: &Error) -> bool {
    matches!(rpc_code_from_client_error(err), Some(-5))
}

fn is_jsonrpc_transport_error(err: &Error) -> bool {
    matches!(err, Error::JsonRpc(jsonrpc::error::Error::Transport(_)))
}

/// Defines the error type returned by a single RPC method on the async `Client`.
///
/// Every method error has an `Rpc` variant, holding the [`Error`] from making the call. Methods
/// that convert the response into a model type additionally have a `Model` variant holding the
/// conversion error. There are three forms:
///
/// - `Name => Type`: `Model` holds the concrete conversion error `Type`.
/// - `Name => boxed`: `Model` holds a boxed error. Used by methods that select a version specific
///   type at runtime and so have no single concrete conversion error type.
/// - `Name`: no `Model` variant, for methods whose response conversion cannot fail.
macro_rules! define_method_error {
    // Version-agnostic (boxed) model conversion error.
    ($(#[$doc:meta])* $name:ident => boxed) => {
        $(#[$doc])*
        #[derive(Debug)]
        pub enum $name {
            /// Making the JSON-RPC call failed.
            Rpc(Error),
            /// Converting the returned JSON into the model type failed.
            Model(Box<dyn std::error::Error + Send + Sync + 'static>),
        }

        impl From<Error> for $name {
            fn from(e: Error) -> Self { Self::Rpc(e) }
        }

        impl From<serde_json::error::Error> for $name {
            fn from(e: serde_json::error::Error) -> Self { Error::Json(e).into() }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self {
                    Self::Rpc(ref e) => write!(f, "JSON-RPC call failed: {}", e),
                    Self::Model(ref e) => write!(f, "conversion to the model type failed: {}", e),
                }
            }
        }

        impl error::Error for $name {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match *self {
                    Self::Rpc(ref e) => Some(e),
                    Self::Model(ref e) => Some(&**e),
                }
            }
        }

        impl $name {
            /// Returns the top-level async client error if this method error wraps one.
            pub fn as_client_error(&self) -> Option<&Error> {
                match self {
                    Self::Rpc(e) => Some(e),
                    Self::Model(_) => None,
                }
            }

            /// Returns `true` when this error is an RPC error with JSON-RPC code `-5`.
            pub fn is_not_found_error(&self) -> bool {
                self.as_client_error().is_some_and(is_jsonrpc_not_found_error)
            }

            /// Returns the JSON-RPC error code when this error wraps an RPC error.
            pub fn rpc_code(&self) -> Option<i64> {
                self.as_client_error().and_then(rpc_code_from_client_error)
            }

            /// Returns `true` when this error wraps a transport-layer failure.
            pub fn is_transport_error(&self) -> bool {
                self.as_client_error().is_some_and(is_jsonrpc_transport_error)
            }
        }
    };
    // Strongly typed model conversion error.
    ($(#[$doc:meta])* $name:ident => $model:ty) => {
        $(#[$doc])*
        #[derive(Debug)]
        pub enum $name {
            /// Making the JSON-RPC call failed.
            Rpc(Error),
            /// Converting the returned JSON into the model type failed.
            Model($model),
        }

        impl From<Error> for $name {
            fn from(e: Error) -> Self { Self::Rpc(e) }
        }

        impl From<serde_json::error::Error> for $name {
            fn from(e: serde_json::error::Error) -> Self { Error::Json(e).into() }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self {
                    Self::Rpc(ref e) => write!(f, "JSON-RPC call failed: {}", e),
                    Self::Model(ref e) => write!(f, "conversion to the model type failed: {}", e),
                }
            }
        }

        impl error::Error for $name {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match *self {
                    Self::Rpc(ref e) => Some(e),
                    Self::Model(ref e) => Some(e),
                }
            }
        }

        impl $name {
            /// Returns the top-level async client error if this method error wraps one.
            pub fn as_client_error(&self) -> Option<&Error> {
                match self {
                    Self::Rpc(e) => Some(e),
                    Self::Model(_) => None,
                }
            }

            /// Returns `true` when this error is an RPC error with JSON-RPC code `-5`.
            pub fn is_not_found_error(&self) -> bool {
                self.as_client_error().is_some_and(is_jsonrpc_not_found_error)
            }

            /// Returns the JSON-RPC error code when this error wraps an RPC error.
            pub fn rpc_code(&self) -> Option<i64> {
                self.as_client_error().and_then(rpc_code_from_client_error)
            }

            /// Returns `true` when this error wraps a transport-layer failure.
            pub fn is_transport_error(&self) -> bool {
                self.as_client_error().is_some_and(is_jsonrpc_transport_error)
            }
        }
    };
    // RPC failure only (response conversion cannot fail).
    ($(#[$doc:meta])* $name:ident) => {
        $(#[$doc])*
        #[derive(Debug)]
        pub enum $name {
            /// Making the JSON-RPC call failed.
            Rpc(Error),
        }

        impl From<Error> for $name {
            fn from(e: Error) -> Self { Self::Rpc(e) }
        }

        impl From<serde_json::error::Error> for $name {
            fn from(e: serde_json::error::Error) -> Self { Error::Json(e).into() }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self {
                    Self::Rpc(ref e) => write!(f, "JSON-RPC call failed: {}", e),
                }
            }
        }

        impl error::Error for $name {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match *self {
                    Self::Rpc(ref e) => Some(e),
                }
            }
        }

        impl $name {
            /// Returns the top-level async client error if this method error wraps one.
            pub fn as_client_error(&self) -> Option<&Error> {
                match self {
                    Self::Rpc(e) => Some(e),
                }
            }

            /// Returns `true` when this error is an RPC error with JSON-RPC code `-5`.
            pub fn is_not_found_error(&self) -> bool {
                self.as_client_error().is_some_and(is_jsonrpc_not_found_error)
            }

            /// Returns the JSON-RPC error code when this error wraps an RPC error.
            pub fn rpc_code(&self) -> Option<i64> {
                self.as_client_error().and_then(rpc_code_from_client_error)
            }

            /// Returns `true` when this error wraps a transport-layer failure.
            pub fn is_transport_error(&self) -> bool {
                self.as_client_error().is_some_and(is_jsonrpc_transport_error)
            }
        }
    };
}

define_method_error! {
    /// Error returned by [`Client::get_block`](crate::client_async::Client::get_block).
    GetBlockError => bitcoin::consensus::encode::FromHexError
}

define_method_error! {
    /// Error returned by [`Client::get_block_count`](crate::client_async::Client::get_block_count).
    GetBlockCountError
}

define_method_error! {
    /// Error returned by [`Client::server_version`](crate::client_async::Client::server_version).
    ServerVersionError
}

define_method_error! {
    /// Error returned by [`Client::get_block_hash`](crate::client_async::Client::get_block_hash).
    GetBlockHashError => bitcoin::hex::HexToArrayError
}

define_method_error! {
    /// Error returned by
    /// [`Client::get_best_block_hash`](crate::client_async::Client::get_best_block_hash).
    GetBestBlockHashError => bitcoin::hex::HexToArrayError
}

define_method_error! {
    /// Error returned by [`Client::get_block_header`](crate::client_async::Client::get_block_header).
    GetBlockHeaderError => types::v17::GetBlockHeaderError
}

/// Error returned by
/// [`Client::get_block_header_verbose`](crate::client_async::Client::get_block_header_verbose).
///
/// The response shape depends on the Core version, so conversion failures are reported per version.
#[derive(Debug)]
pub enum GetBlockHeaderVerboseError {
    /// Making the JSON-RPC call failed.
    Rpc(Error),
    /// Converting the returned v29 JSON into the model type failed.
    ModelV29(types::v29::GetBlockHeaderVerboseError),
    /// Converting the returned v25 JSON into the model type failed.
    ModelV25(types::v25::GetBlockHeaderVerboseError),
}

impl fmt::Display for GetBlockHeaderVerboseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Rpc(ref e) => write!(f, "JSON-RPC call failed: {}", e),
            Self::ModelV29(ref e) =>
                write!(f, "conversion to the model type from the v29 type failed: {}", e),
            Self::ModelV25(ref e) =>
                write!(f, "conversion to the model type from the v25 type failed: {}", e),
        }
    }
}

impl error::Error for GetBlockHeaderVerboseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Rpc(ref e) => Some(e),
            Self::ModelV29(ref e) => Some(e),
            Self::ModelV25(ref e) => Some(e),
        }
    }
}

impl From<Infallible> for GetBlockHeaderVerboseError {
    fn from(never: Infallible) -> Self { match never {} }
}

impl From<Error> for GetBlockHeaderVerboseError {
    fn from(e: Error) -> Self { Self::Rpc(e) }
}

impl From<serde_json::error::Error> for GetBlockHeaderVerboseError {
    fn from(e: serde_json::error::Error) -> Self { Self::Rpc(Error::Json(e)) }
}

impl GetBlockHeaderVerboseError {
    /// Returns the top-level async client error if this method error wraps one.
    pub fn as_client_error(&self) -> Option<&Error> {
        match self {
            Self::Rpc(e) => Some(e),
            Self::ModelV29(_) | Self::ModelV25(_) => None,
        }
    }

    /// Returns `true` when this error is an RPC error with JSON-RPC code `-5`.
    pub fn is_not_found_error(&self) -> bool {
        self.as_client_error().is_some_and(is_jsonrpc_not_found_error)
    }

    /// Returns the JSON-RPC error code when this error wraps an RPC error.
    pub fn rpc_code(&self) -> Option<i64> {
        self.as_client_error().and_then(rpc_code_from_client_error)
    }

    /// Returns `true` when this error wraps a transport-layer failure.
    pub fn is_transport_error(&self) -> bool {
        self.as_client_error().is_some_and(is_jsonrpc_transport_error)
    }
}

/// Error returned by [`Client::get_block_verbose`](crate::client_async::Client::get_block_verbose).
///
/// The response shape depends on the Core version, so conversion failures are reported per version.
#[derive(Debug)]
pub enum GetBlockVerboseError {
    /// Making the JSON-RPC call failed.
    Rpc(Error),
    /// Converting the returned v31 JSON into the model type failed.
    ModelV31(types::v31::GetBlockVerboseOneError),
    /// Converting the returned v29 JSON into the model type failed.
    ModelV29(types::v29::GetBlockVerboseOneError),
    /// Converting the returned v25 JSON into the model type failed.
    ModelV25(types::v25::GetBlockVerboseOneError),
}

impl fmt::Display for GetBlockVerboseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Rpc(ref e) => write!(f, "JSON-RPC call failed: {}", e),
            Self::ModelV31(ref e) =>
                write!(f, "conversion to the model type from the v31 type failed: {}", e),
            Self::ModelV29(ref e) =>
                write!(f, "conversion to the model type from the v29 type failed: {}", e),
            Self::ModelV25(ref e) =>
                write!(f, "conversion to the model type from the v25 type failed: {}", e),
        }
    }
}

impl error::Error for GetBlockVerboseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Rpc(ref e) => Some(e),
            Self::ModelV31(ref e) => Some(e),
            Self::ModelV29(ref e) => Some(e),
            Self::ModelV25(ref e) => Some(e),
        }
    }
}

impl From<Infallible> for GetBlockVerboseError {
    fn from(never: Infallible) -> Self { match never {} }
}

impl From<Error> for GetBlockVerboseError {
    fn from(e: Error) -> Self { Self::Rpc(e) }
}

impl From<serde_json::error::Error> for GetBlockVerboseError {
    fn from(e: serde_json::error::Error) -> Self { Self::Rpc(Error::Json(e)) }
}

impl GetBlockVerboseError {
    /// Returns the top-level async client error if this method error wraps one.
    pub fn as_client_error(&self) -> Option<&Error> {
        match self {
            Self::Rpc(e) => Some(e),
            Self::ModelV31(_) | Self::ModelV29(_) | Self::ModelV25(_) => None,
        }
    }

    /// Returns `true` when this error is an RPC error with JSON-RPC code `-5`.
    pub fn is_not_found_error(&self) -> bool {
        self.as_client_error().is_some_and(is_jsonrpc_not_found_error)
    }

    /// Returns the JSON-RPC error code when this error wraps an RPC error.
    pub fn rpc_code(&self) -> Option<i64> {
        self.as_client_error().and_then(rpc_code_from_client_error)
    }

    /// Returns `true` when this error wraps a transport-layer failure.
    pub fn is_transport_error(&self) -> bool {
        self.as_client_error().is_some_and(is_jsonrpc_transport_error)
    }
}

define_method_error! {
    /// Error returned by [`Client::get_block_filter`](crate::client_async::Client::get_block_filter).
    GetBlockFilterError => types::v19::GetBlockFilterError
}

define_method_error! {
    /// Error returned by [`Client::get_raw_mempool`](crate::client_async::Client::get_raw_mempool).
    GetRawMempoolError => bitcoin::hex::HexToArrayError
}

define_method_error! {
    /// Error returned by
    /// [`Client::get_raw_transaction`](crate::client_async::Client::get_raw_transaction).
    GetRawTransactionError => bitcoin::consensus::encode::FromHexError
}

/// Error returned by
/// [`Client::get_blockchain_info`](crate::client_async::Client::get_blockchain_info).
///
/// The response shape depends on the Core version, so conversion failures are reported per version.
#[derive(Debug)]
pub enum GetBlockchainInfoError {
    /// Making the JSON-RPC call failed.
    Rpc(Error),
    /// Converting the returned v29 JSON into the model type failed.
    ModelV29(types::v29::GetBlockchainInfoError),
    /// Converting the returned v28 JSON into the model type failed.
    ModelV28(types::v28::GetBlockchainInfoError),
    /// Converting the returned v25 JSON into the model type failed.
    ModelV25(types::v25::GetBlockchainInfoError),
}

impl fmt::Display for GetBlockchainInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Rpc(ref e) => write!(f, "JSON-RPC call failed: {}", e),
            Self::ModelV29(ref e) =>
                write!(f, "conversion to the model type from the v29 type failed: {}", e),
            Self::ModelV28(ref e) =>
                write!(f, "conversion to the model type from the v28 type failed: {}", e),
            Self::ModelV25(ref e) =>
                write!(f, "conversion to the model type from the v25 type failed: {}", e),
        }
    }
}

impl error::Error for GetBlockchainInfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Rpc(ref e) => Some(e),
            Self::ModelV29(ref e) => Some(e),
            Self::ModelV28(ref e) => Some(e),
            Self::ModelV25(ref e) => Some(e),
        }
    }
}

impl From<Infallible> for GetBlockchainInfoError {
    fn from(never: Infallible) -> Self { match never {} }
}

impl From<Error> for GetBlockchainInfoError {
    fn from(e: Error) -> Self { Self::Rpc(e) }
}

impl From<serde_json::error::Error> for GetBlockchainInfoError {
    fn from(e: serde_json::error::Error) -> Self { Self::Rpc(Error::Json(e)) }
}

impl GetBlockchainInfoError {
    /// Returns the top-level async client error if this method error wraps one.
    pub fn as_client_error(&self) -> Option<&Error> {
        match self {
            Self::Rpc(e) => Some(e),
            Self::ModelV29(_) | Self::ModelV28(_) | Self::ModelV25(_) => None,
        }
    }

    /// Returns `true` when this error is an RPC error with JSON-RPC code `-5`.
    pub fn is_not_found_error(&self) -> bool {
        self.as_client_error().is_some_and(is_jsonrpc_not_found_error)
    }

    /// Returns the JSON-RPC error code when this error wraps an RPC error.
    pub fn rpc_code(&self) -> Option<i64> {
        self.as_client_error().and_then(rpc_code_from_client_error)
    }

    /// Returns `true` when this error wraps a transport-layer failure.
    pub fn is_transport_error(&self) -> bool {
        self.as_client_error().is_some_and(is_jsonrpc_transport_error)
    }
}

define_method_error! {
    /// Error returned by [`Client::get_tx_out`](crate::client_async::Client::get_tx_out).
    GetTxOutError => types::v17::GetTxOutError
}

/// Error returned when RPC client expects a different version than bitcoind reports.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnexpectedServerVersionError {
    /// Version from server.
    pub got: usize,
    /// Expected server version.
    pub expected: Vec<usize>,
}

impl fmt::Display for UnexpectedServerVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut expected = String::new();
        for version in &self.expected {
            let v = format!(" {} ", version);
            expected.push_str(&v);
        }
        write!(f, "unexpected bitcoind version, got: {} expected one of: {}", self.got, expected)
    }
}

impl error::Error for UnexpectedServerVersionError {}

impl From<UnexpectedServerVersionError> for Error {
    fn from(e: UnexpectedServerVersionError) -> Self { Self::ServerVersion(e) }
}

#[cfg(test)]
mod tests {
    use std::io;

    use super::*;

    fn rpc_client_error(code: i32) -> Error {
        Error::JsonRpc(jsonrpc::error::Error::Rpc(jsonrpc::error::RpcError {
            code,
            message: "rpc error".to_owned(),
            data: None,
        }))
    }

    fn transport_client_error() -> Error {
        Error::JsonRpc(jsonrpc::error::Error::Transport(Box::new(io::Error::other(
            "transport failed",
        ))))
    }

    #[test]
    fn rpc_minus_five_is_not_found() {
        let err = GetBlockError::Rpc(rpc_client_error(-5));
        assert!(err.is_not_found_error());
        assert_eq!(err.rpc_code(), Some(-5));
        assert!(!err.is_transport_error());
        assert!(err.as_client_error().is_some());
    }

    #[test]
    fn other_rpc_code_is_not_not_found() {
        let err = GetBlockCountError::Rpc(rpc_client_error(-8));
        assert!(!err.is_not_found_error());
        assert_eq!(err.rpc_code(), Some(-8));
        assert!(!err.is_transport_error());
    }

    #[test]
    fn transport_errors_are_detected() {
        let err = GetBlockCountError::Rpc(transport_client_error());
        assert!(err.is_transport_error());
        assert!(!err.is_not_found_error());
        assert_eq!(err.rpc_code(), None);
        assert!(err.as_client_error().is_some());
    }

    #[test]
    fn model_errors_do_not_report_rpc_details() {
        let err = GetBlockVerboseError::ModelV31(types::v31::GetBlockVerboseOneError::Numeric(
            types::NumericError::Negative { field: "height".to_owned(), value: -1 },
        ));
        assert!(!err.is_not_found_error());
        assert_eq!(err.rpc_code(), None);
        assert!(!err.is_transport_error());
        assert!(err.as_client_error().is_none());
    }
}
