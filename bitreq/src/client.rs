//! Connection pooling [`Client`] for HTTP requests.
//!
//! The [`Client`] caches connections to avoid repeated TCP handshakes and TLS negotiations.
//!
//! When the `async` feature is enabled, the client uses async connections via `tokio`.
//! Otherwise a blocking client backed by `std::net::TcpStream` is provided.

use std::collections::{hash_map, HashMap, VecDeque};
use std::sync::{Arc, Mutex};

// ---------------------------------------------------------------------------
// Async Client (feature = "async")
// ---------------------------------------------------------------------------
#[cfg(feature = "async")]
use crate::connection::AsyncConnection;
use crate::request::{OwnedConnectionParams as ConnectionKey, ParsedRequest};
use crate::{Error, Request, Response};

/// A client that caches connections for reuse.
///
/// The client maintains a pool of up to `capacity` connections, evicting
/// the least recently used connection when the cache is full.
///
/// # Example
///
/// ```no_run
/// # async fn request() {
/// use bitreq::{Client, RequestExt};
///
/// let client = Client::new(10); // Cache up to 10 connections
/// let response = bitreq::get("https://example.com")
///     .send_async_with_client(&client)
///     .await;
/// # }
/// ```
#[cfg(feature = "async")]
#[derive(Clone)]
pub struct Client {
    r#async: Arc<Mutex<AsyncClientState>>,
}

#[cfg(feature = "async")]
struct AsyncClientState {
    connections: HashMap<ConnectionKey, Arc<AsyncConnection>>,
    lru_order: VecDeque<ConnectionKey>,
    capacity: usize,
}

#[cfg(feature = "async")]
impl Client {
    /// Creates a new `Client` with the specified connection cache capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - Maximum number of cached connections. When this limit is
    ///   reached, the least recently used connection is evicted.
    pub fn new(capacity: usize) -> Self {
        Client {
            r#async: Arc::new(Mutex::new(AsyncClientState {
                connections: HashMap::new(),
                lru_order: VecDeque::new(),
                capacity,
            })),
        }
    }

    /// Sends a request asynchronously using a cached connection if available.
    pub async fn send_async(&self, request: Request) -> Result<Response, Error> {
        let parsed_request = ParsedRequest::new(request)?;
        let key = parsed_request.connection_params();
        let owned_key = key.into();

        // Try to get cached connection
        let conn_opt = {
            let state = self.r#async.lock().unwrap();

            if let Some(conn) = state.connections.get(&owned_key) {
                Some(Arc::clone(conn))
            } else {
                None
            }
        };
        let conn = if let Some(conn) = conn_opt {
            conn
        } else {
            let connection = AsyncConnection::new(key, parsed_request.timeout_at).await?;
            let connection = Arc::new(connection);

            let mut state = self.r#async.lock().unwrap();
            if let hash_map::Entry::Vacant(entry) = state.connections.entry(owned_key) {
                entry.insert(Arc::clone(&connection));
                state.lru_order.push_back(key.into());
                if state.connections.len() > state.capacity {
                    if let Some(oldest_key) = state.lru_order.pop_front() {
                        state.connections.remove(&oldest_key);
                    }
                }
            }
            connection
        };

        // Send the request
        conn.send(parsed_request).await
    }
}

/// Extension trait for [`Request`] to use with [`Client`].
#[cfg(feature = "async")]
pub trait RequestExt {
    /// Sends this request asynchronously using the provided client's connection pool.
    fn send_async_with_client(
        self,
        client: &Client,
    ) -> impl std::future::Future<Output = Result<Response, Error>>;
}

#[cfg(feature = "async")]
impl RequestExt for Request {
    fn send_async_with_client(
        self,
        client: &Client,
    ) -> impl std::future::Future<Output = Result<Response, Error>> {
        client.send_async(self)
    }
}

// ---------------------------------------------------------------------------
// Blocking Client (no "async" feature)
// ---------------------------------------------------------------------------

#[cfg(not(feature = "async"))]
use core::time::Duration;
#[cfg(not(feature = "async"))]
use std::time::Instant;

#[cfg(not(feature = "async"))]
use crate::connection::{Connection, HttpStream};
#[cfg(not(feature = "async"))]
use crate::Method;

#[cfg(not(feature = "async"))]
struct PoolEntry {
    stream: HttpStream,
    expires_at: Instant,
}

/// A client that caches connections for reuse.
///
/// The client maintains a pool of up to `capacity` connections, evicting
/// the least recently used connection when the cache is full. A cached
/// connection is reused when the server indicated `Connection: keep-alive`
/// and the keep-alive timeout has not yet expired.
///
/// # Example
///
/// ```no_run
/// # fn main() -> Result<(), bitreq::Error> {
/// use bitreq::{Client, RequestExt};
///
/// let client = Client::new(10); // Cache up to 10 connections
/// let response = bitreq::get("http://example.com")
///     .send_with_client(&client)?;
/// # Ok(()) }
/// ```
#[cfg(not(feature = "async"))]
#[derive(Clone)]
pub struct Client {
    state: Arc<Mutex<BlockingClientState>>,
}

#[cfg(not(feature = "async"))]
struct BlockingClientState {
    connections: HashMap<ConnectionKey, PoolEntry>,
    lru_order: VecDeque<ConnectionKey>,
    capacity: usize,
}

#[cfg(not(feature = "async"))]
impl Client {
    /// Creates a new `Client` with the specified connection cache capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - Maximum number of cached connections. When this limit is
    ///   reached, the least recently used connection is evicted.
    pub fn new(capacity: usize) -> Self {
        Client {
            state: Arc::new(Mutex::new(BlockingClientState {
                connections: HashMap::new(),
                lru_order: VecDeque::new(),
                capacity,
            })),
        }
    }

    /// Sends a request using a cached connection if available.
    pub fn send(&self, request: Request) -> Result<Response, Error> {
        let parsed_request = ParsedRequest::new(request)?;
        self.send_inner(parsed_request)
    }

    fn send_inner(&self, mut request: ParsedRequest) -> Result<Response, Error> {
        loop {
            let key: ConnectionKey = request.connection_params().into();

            // Get cached stream or create new connection
            let connection = match self.take_stream(&key) {
                Some(stream) => Connection::from_stream(stream),
                None => Connection::new(request.connection_params(), request.timeout_at)?,
            };

            let (response, stream, req) = connection.send_for_pool(request)?;
            request = req;

            // Cache stream if keep-alive, with expiry
            if let Some(stream) = stream {
                let expires_at = Self::parse_keep_alive_timeout(response.headers.get("keep-alive"));
                self.put_stream(key, stream, expires_at);
            }

            // Handle redirects
            match response.status_code {
                301 | 302 | 303 | 307 => {
                    let location = response
                        .headers
                        .get("location")
                        .ok_or(Error::RedirectLocationMissing)?
                        .clone();
                    request.redirect_to(&location)?;
                    if response.status_code == 303 {
                        match request.config.method {
                            Method::Post | Method::Put | Method::Delete => {
                                request.config.method = Method::Get;
                            }
                            _ => {}
                        }
                    }
                    continue;
                }
                _ => return Ok(response),
            }
        }
    }

    fn take_stream(&self, key: &ConnectionKey) -> Option<HttpStream> {
        let mut state = self.state.lock().unwrap();
        if let Some(entry) = state.connections.remove(key) {
            // Remove from LRU order
            if let Some(pos) = state.lru_order.iter().position(|k| k == key) {
                state.lru_order.remove(pos);
            }
            if entry.expires_at > Instant::now() {
                return Some(entry.stream);
            }
        }
        None
    }

    fn put_stream(&self, key: ConnectionKey, stream: HttpStream, expires_at: Instant) {
        let mut state = self.state.lock().unwrap();
        if let hash_map::Entry::Vacant(entry) = state.connections.entry(key.clone()) {
            entry.insert(PoolEntry { stream, expires_at });
            state.lru_order.push_back(key);
            if state.connections.len() > state.capacity {
                if let Some(oldest_key) = state.lru_order.pop_front() {
                    state.connections.remove(&oldest_key);
                }
            }
        }
    }

    fn parse_keep_alive_timeout(keep_alive_header: Option<&String>) -> Instant {
        let default_timeout = Instant::now() + Duration::from_secs(60);
        if let Some(header) = keep_alive_header {
            for param in header.split(',') {
                if let Some((k, v)) = param.trim().split_once('=') {
                    if k.trim() == "timeout" {
                        if let Ok(secs) = v.parse::<u64>() {
                            return Instant::now() + Duration::from_secs(secs.saturating_sub(1));
                        }
                    }
                }
            }
        }
        default_timeout
    }
}

/// Extension trait for [`Request`] to use with [`Client`].
#[cfg(not(feature = "async"))]
pub trait RequestExt {
    /// Sends this request using the provided client's connection pool.
    fn send_with_client(self, client: &Client) -> Result<Response, Error>;
}

#[cfg(not(feature = "async"))]
impl RequestExt for Request {
    fn send_with_client(self, client: &Client) -> Result<Response, Error> { client.send(self) }
}
