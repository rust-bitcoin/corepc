// SPDX-License-Identifier: CC0-1.0

//! JSON-RPC async client support.

use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic;

use serde_json::value::RawValue;

use super::{Error, Request, Response};

/// Boxed future type used by async transports.
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// An interface for an async transport over which to use the JSONRPC protocol.
pub trait AsyncTransport: Send + Sync + 'static {
    /// Sends an RPC request over the transport.
    fn send_request<'a>(&'a self, req: Request<'a>) -> BoxFuture<'a, Result<Response, Error>>;
    /// Formats the target of this transport. I.e. the URL/socket/...
    fn fmt_target(&self, f: &mut fmt::Formatter) -> fmt::Result;
}

/// An async JSON-RPC client.
pub struct AsyncClient {
    pub(crate) transport: Box<dyn AsyncTransport>,
    nonce: atomic::AtomicUsize,
}

impl AsyncClient {
    /// Creates a new client with the given transport.
    pub fn with_transport<T: AsyncTransport>(transport: T) -> AsyncClient {
        AsyncClient { transport: Box::new(transport), nonce: atomic::AtomicUsize::new(1) }
    }

    /// Builds a request.
    pub fn build_request<'a>(&self, method: &'a str, params: Option<&'a RawValue>) -> Request<'a> {
        let nonce = self.nonce.fetch_add(1, atomic::Ordering::Relaxed);
        Request { method, params, id: serde_json::Value::from(nonce), jsonrpc: Some("2.0") }
    }

    /// Sends a request to a client.
    pub async fn send_request(&self, request: Request<'_>) -> Result<Response, Error> {
        self.transport.send_request(request).await
    }
}

impl fmt::Debug for AsyncClient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "jsonrpc::AsyncClient(")?;
        self.transport.fmt_target(f)?;
        write!(f, ")")
    }
}

impl<T: AsyncTransport> From<T> for AsyncClient {
    fn from(t: T) -> AsyncClient { AsyncClient::with_transport(t) }
}
