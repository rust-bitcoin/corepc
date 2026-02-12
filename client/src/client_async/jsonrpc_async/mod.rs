// SPDX-License-Identifier: CC0-1.0

//! Minimal JSON-RPC support for the sync client.

pub(crate) mod client;
pub(crate) mod error;
pub(crate) mod http;

use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

pub(crate) use self::client_async::Client;
pub(crate) use self::error::Error;

/// A JSON-RPC request object.
#[derive(Debug, Clone, Serialize)]
pub struct Request<'a> {
    /// The name of the RPC call.
    pub method: &'a str,
    /// Parameters to the RPC call.
    pub params: Option<&'a RawValue>,
    /// Identifier for this request, which should appear in the response.
    pub id: serde_json::Value,
    /// jsonrpc field, MUST be "2.0".
    pub jsonrpc: Option<&'a str>,
}

/// A JSON-RPC response object.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Response {
    /// A result if there is one, or [`None`].
    pub result: Option<Box<RawValue>>,
    /// An error if there is one, or [`None`].
    pub error: Option<error::RpcError>,
    /// Identifier for this response, which should match that of the request.
    pub id: serde_json::Value,
    /// jsonrpc field, MUST be "2.0".
    pub jsonrpc: Option<String>,
}

impl Response {
    /// Extracts the result from a response.
    pub fn result<T: for<'a> serde::de::Deserialize<'a>>(&self) -> Result<T, Error> {
        if let Some(ref e) = self.error {
            return Err(Error::Rpc(e.clone()));
        }

        if let Some(ref res) = self.result {
            serde_json::from_str(res.get()).map_err(Error::Json)
        } else {
            serde_json::from_value(serde_json::Value::Null).map_err(Error::Json)
        }
    }

}
