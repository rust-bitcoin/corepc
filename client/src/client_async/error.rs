// SPDX-License-Identifier: CC0-1.0

use std::{error, fmt, io};

use bitcoin::hex;

/// The error type for errors produced in this library.
#[derive(Debug)]
pub enum Error {
    JsonRpc(jsonrpc::error::Error),
    HexToArray(hex::HexToArrayError),
    HexToBytes(hex::HexToBytesError),
    Json(serde_json::error::Error),
    BitcoinSerialization(bitcoin::consensus::encode::FromHexError),
    Io(io::Error),
    InvalidCookieFile,
    /// The JSON result had an unexpected structure.
    UnexpectedStructure,
    /// The daemon returned an error string.
    Returned(String),
    /// A model conversion error.
    Model(IntoModelError),
    /// The server version did not match what was expected.
    ServerVersion(UnexpectedServerVersionError),
    /// Missing user/password.
    MissingUserPassword,
}

impl From<jsonrpc::error::Error> for Error {
    fn from(e: jsonrpc::error::Error) -> Error { Error::JsonRpc(e) }
}

impl From<hex::HexToArrayError> for Error {
    fn from(e: hex::HexToArrayError) -> Self { Self::HexToArray(e) }
}

impl From<hex::HexToBytesError> for Error {
    fn from(e: hex::HexToBytesError) -> Self { Self::HexToBytes(e) }
}

impl From<serde_json::error::Error> for Error {
    fn from(e: serde_json::error::Error) -> Error { Error::Json(e) }
}

impl From<bitcoin::consensus::encode::FromHexError> for Error {
    fn from(e: bitcoin::consensus::encode::FromHexError) -> Error { Error::BitcoinSerialization(e) }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error { Error::Io(e) }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;

        match *self {
            JsonRpc(ref e) => write!(f, "JSON-RPC error: {}", e),
            HexToArray(ref e) => write!(f, "hex to array decode error: {}", e),
            HexToBytes(ref e) => write!(f, "hex to bytes decode error: {}", e),
            Json(ref e) => write!(f, "JSON error: {}", e),
            BitcoinSerialization(ref e) => write!(f, "Bitcoin serialization error: {}", e),
            Io(ref e) => write!(f, "I/O error: {}", e),
            InvalidCookieFile => write!(f, "invalid cookie file"),
            UnexpectedStructure => write!(f, "the JSON result had an unexpected structure"),
            Returned(ref s) => write!(f, "the daemon returned an error string: {}", s),
            Model(ref e) => write!(f, "model conversion error: {e}"),
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
            HexToArray(ref e) => Some(e),
            HexToBytes(ref e) => Some(e),
            Json(ref e) => Some(e),
            BitcoinSerialization(ref e) => Some(e),
            Io(ref e) => Some(e),
            ServerVersion(ref e) => Some(e),
            Model(ref e) => Some(e),
            InvalidCookieFile | UnexpectedStructure | Returned(_) | MissingUserPassword => None,
        }
    }
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

/// Error returned when converting an RPC response into a model type fails.
#[derive(Debug)]
pub struct IntoModelError {
    context: &'static str,
    source: Box<dyn error::Error + Send + Sync + 'static>,
}

impl IntoModelError {
    /// Creates a new model conversion error with caller-provided context.
    pub fn new<E>(context: &'static str, source: E) -> Self
    where
        E: error::Error + Send + Sync + 'static,
    {
        Self { context, source: Box::new(source) }
    }

    /// Returns the context for the failed conversion.
    pub fn context(&self) -> &'static str { self.context }
}

impl fmt::Display for IntoModelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "conversion of {} into a model type failed", self.context)
    }
}

impl error::Error for IntoModelError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> { Some(&*self.source) }
}

impl From<IntoModelError> for Error {
    fn from(e: IntoModelError) -> Self { Self::Model(e) }
}
