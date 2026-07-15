// SPDX-License-Identifier: CC0-1.0

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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Rpc(ref e) => write!(f, "JSON-RPC call failed: {}", e),
            Self::ModelV29(ref e) => write!(f, "conversion to the model type from v29 type failed: {}", e),
            Self::ModelV25(ref e) => write!(f, "conversion to the model type from v25 type failed: {}", e),
        }
    }
}

impl std::error::Error for GetBlockHeaderVerboseError {
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
            fn from(e: serde_json::error::Error) -> Self { Self::Rpc(Error::Json(e)) }
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
            fn from(e: serde_json::error::Error) -> Self { Self::Rpc(Error::Json(e)) }
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
            fn from(e: serde_json::error::Error) -> Self { Self::Rpc(Error::Json(e)) }
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

define_method_error! {
    /// Error returned by [`Client::get_block_verbose`](crate::client_async::Client::get_block_verbose).
    GetBlockVerboseError => boxed
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

define_method_error! {
    /// Error returned by
    /// [`Client::get_blockchain_info`](crate::client_async::Client::get_blockchain_info).
    GetBlockchainInfoError => boxed
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
