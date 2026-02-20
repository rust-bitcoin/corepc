// SPDX-License-Identifier: CC0-1.0

//! Error handling for JSON-RPC.

use std::{error, fmt};

use serde::{Deserialize, Serialize};

/// A library error.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// A transport error.
    Transport(Box<dyn error::Error + Send + Sync>),
    /// Json error.
    Json(serde_json::Error),
    /// Error response.
    Rpc(RpcError),
    /// Response to a request did not have the expected nonce.
    NonceMismatch,
    /// Response to a request had a jsonrpc field other than "2.0".
    VersionMismatch,
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error { Error::Json(e) }
}

impl From<RpcError> for Error {
    fn from(e: RpcError) -> Error { Error::Rpc(e) }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;

        match *self {
            Transport(ref e) => write!(f, "transport error: {}", e),
            Json(ref e) => write!(f, "JSON decode error: {}", e),
            Rpc(ref r) => write!(f, "RPC error response: {:?}", r),
            NonceMismatch => write!(f, "nonce of response did not match nonce of request"),
            VersionMismatch => write!(f, "`jsonrpc` field set to non-\"2.0\""),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use self::Error::*;

        match *self {
            Rpc(_) | NonceMismatch | VersionMismatch => None,
            Transport(ref e) => Some(&**e),
            Json(ref e) => Some(e),
        }
    }
}

/// A JSON-RPC error object.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RpcError {
    /// The integer identifier of the error.
    pub code: i32,
    /// A string describing the error.
    pub message: String,
    /// Additional data specific to the error.
    pub data: Option<Box<serde_json::value::RawValue>>,
}
