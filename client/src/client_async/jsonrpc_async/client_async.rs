// SPDX-License-Identifier: CC0-1.0

//! JSON-RPC client support.

use std::fmt;
use std::sync::atomic;

use serde_json::value::RawValue;

use crate::client_async::jsonrpc_async::error::Error;
use crate::client_async::jsonrpc_async::{Request, Response};

/// An interface for a transport over which to use the JSON-RPC protocol.
pub trait Transport: Send + Sync + 'static {
    /// Sends an RPC request over the transport.
    fn send_request(&self, req: Request) -> Result<Response, Error>;
    /// Formats the target of this transport. I.e. the URL/socket/...
    fn fmt_target(&self, f: &mut fmt::Formatter) -> fmt::Result;
}

/// A JSON-RPC client.
pub struct Client {
    pub(crate) transport: Box<dyn Transport>,
    nonce: atomic::AtomicUsize,
}

impl Client {
    /// Creates a new client with the given transport.
    pub fn with_transport<T: Transport>(transport: T) -> Client {
        Client { transport: Box::new(transport), nonce: atomic::AtomicUsize::new(1) }
    }

    /// Builds a request.
    pub fn build_request<'a>(&self, method: &'a str, params: Option<&'a RawValue>) -> Request<'a> {
        let nonce = self.nonce.fetch_add(1, atomic::Ordering::Relaxed);
        Request { method, params, id: serde_json::Value::from(nonce), jsonrpc: Some("2.0") }
    }

    /// Sends a request to a client.
    pub fn send_request(&self, request: Request) -> Result<Response, Error> {
        self.transport.send_request(request)
    }

}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "jsonrpc::Client(")?;
        self.transport.fmt_target(f)?;
        write!(f, ")")
    }
}

impl<T: Transport> From<T> for Client {
    fn from(t: T) -> Client { Client::with_transport(t) }
}
