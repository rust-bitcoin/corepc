//! Connection pooling [`Client`] for HTTP requests.
//!
//! The [`Client`] caches connections to avoid repeated TCP handshakes and TLS negotiations.
//!
//! A blocking connection pool is always available. When the `async` feature is enabled, an
//! additional async connection pool is exposed via [`Client::send_async`] and
//! [`RequestExt::send_async_with_client`]. Both pools share a single idle-cache budget
//! governed by a unified LRU.

use std::collections::{hash_map, HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::Instant;

#[cfg(feature = "async")]
use crate::connection::AsyncConnection;
use crate::connection::{Connection, HttpStream};
use crate::request::{OwnedConnectionParams as ConnectionKey, ParsedRequest};
use crate::{Error, Request, Response};

/// A client that caches connections for reuse.
///
/// The client maintains a cache of up to `max_idle` total connections — shared across
/// the blocking and (when enabled) async paths — evicting the least recently used
/// entry when the cache is full. A cached blocking connection is reused when the
/// server indicated `Connection: keep-alive` and the keep-alive timeout has not yet
/// expired.
///
/// # Bound applies to cached entries, not live connections
///
/// `max_idle` bounds the number of connections held in the cache, not the number of
/// connections the client may have open at any one time. Concurrent requests whose
/// cached connection is absent (or currently checked out for another in-flight
/// request) each open a fresh socket; any surplus streams simply fail to re-enter
/// the cache on put-back once it is full. To bound concurrency — rather than idle
/// reuse — a caller must arrange a separate semaphore on top.
///
/// # Examples
///
/// Blocking:
/// ```no_run
/// # fn main() -> Result<(), bitreq::Error> {
/// use bitreq::{Client, RequestExt};
///
/// let client = Client::new(10); // Cache up to 10 idle connections
/// let response = bitreq::get("http://example.com").send_with_client(&client)?;
/// # Ok(()) }
/// ```
///
/// Async (requires the `async` feature):
#[cfg_attr(feature = "async", doc = "```no_run")]
#[cfg_attr(not(feature = "async"), doc = "```ignore")]
/// # async fn request() -> Result<(), bitreq::Error> {
/// use bitreq::{Client, RequestExt};
///
/// let client = Client::new(10);
/// let response = bitreq::get("https://example.com")
///     .send_async_with_client(&client)
///     .await?;
/// # Ok(()) }
/// ```
#[derive(Clone)]
pub struct Client {
    state: Arc<Mutex<ClientState>>,
}

struct ClientState {
    blocking_connections: HashMap<ConnectionKey, PoolEntry>,
    #[cfg(feature = "async")]
    async_connections: HashMap<ConnectionKey, Arc<AsyncConnection>>,
    /// Unified LRU across both pools. The oldest entry is at the front.
    lru_order: VecDeque<LruKey>,
    max_idle: usize,
}

#[derive(Clone, PartialEq, Eq)]
enum LruKey {
    Blocking(ConnectionKey),
    #[cfg(feature = "async")]
    Async(ConnectionKey),
}

pub(crate) struct PoolEntry {
    pub(crate) stream: HttpStream,
    pub(crate) expires_at: Instant,
}

impl Client {
    /// Creates a new `Client` with the specified total idle-cache size.
    ///
    /// The cache is shared across the blocking and (when enabled) async paths. When
    /// the total number of cached connections exceeds `max_idle`, the least recently
    /// used entry is evicted regardless of which pool it lives in. See the
    /// [type-level docs](Client) for why this does not bound the number of live
    /// connections.
    pub fn new(max_idle: usize) -> Self {
        Client {
            state: Arc::new(Mutex::new(ClientState {
                blocking_connections: HashMap::new(),
                #[cfg(feature = "async")]
                async_connections: HashMap::new(),
                lru_order: VecDeque::new(),
                max_idle,
            })),
        }
    }

    /// Sends a request using a cached connection if available.
    pub fn send(&self, request: Request) -> Result<Response, Error> {
        let parsed = ParsedRequest::new(request)?;
        let key: ConnectionKey = parsed.connection_params().into();
        let connection = match self.take_connection(&key) {
            Some(conn) => conn,
            None => Connection::new(parsed.connection_params(), parsed.timeout_at)?,
        };
        connection.send_pooled(self, parsed)
    }

    /// Takes a pooled [`Connection`] for `key`, if one exists and has not expired.
    pub(crate) fn take_connection(&self, key: &ConnectionKey) -> Option<Connection> {
        let mut state = self.state.lock().unwrap();
        let entry = state.blocking_connections.remove(key)?;
        let lru_key = LruKey::Blocking(key.clone());
        if let Some(pos) = state.lru_order.iter().position(|k| k == &lru_key) {
            state.lru_order.remove(pos);
        }
        if entry.expires_at > Instant::now() {
            Some(Connection::from_stream(entry.stream))
        } else {
            None
        }
    }

    /// Puts a stream back into the pool under `key`, with the given expiry.
    pub(crate) fn put_stream(&self, key: ConnectionKey, stream: HttpStream, expires_at: Instant) {
        let mut state = self.state.lock().unwrap();
        if let hash_map::Entry::Vacant(entry) = state.blocking_connections.entry(key.clone()) {
            entry.insert(PoolEntry { stream, expires_at });
            state.lru_order.push_back(LruKey::Blocking(key));
            state.evict_if_over_capacity();
        }
    }

    /// Sends a request asynchronously using a cached connection if available.
    #[cfg(feature = "async")]
    pub async fn send_async(&self, request: Request) -> Result<Response, Error> {
        let parsed_request = ParsedRequest::new(request)?;
        let key = parsed_request.connection_params();
        let owned_key: ConnectionKey = key.into();

        let conn_opt = {
            let mut state = self.state.lock().unwrap();
            if let Some(conn) = state.async_connections.get(&owned_key) {
                let conn = Arc::clone(conn);
                // Refresh LRU position so this hit is treated as the most recent use.
                let lru_key = LruKey::Async(owned_key.clone());
                if let Some(pos) = state.lru_order.iter().position(|k| k == &lru_key) {
                    state.lru_order.remove(pos);
                    state.lru_order.push_back(lru_key);
                }
                Some(conn)
            } else {
                None
            }
        };

        let conn = if let Some(conn) = conn_opt {
            conn
        } else {
            let connection = AsyncConnection::new(key, parsed_request.timeout_at).await?;
            let connection = Arc::new(connection);

            let mut state = self.state.lock().unwrap();
            if let hash_map::Entry::Vacant(entry) = state.async_connections.entry(owned_key.clone())
            {
                entry.insert(Arc::clone(&connection));
                state.lru_order.push_back(LruKey::Async(owned_key));
                state.evict_if_over_capacity();
            }
            connection
        };

        conn.send(parsed_request).await
    }
}

impl ClientState {
    fn total_len(&self) -> usize {
        let total = self.blocking_connections.len();
        #[cfg(feature = "async")]
        let total = total + self.async_connections.len();
        total
    }

    fn evict_if_over_capacity(&mut self) {
        while self.total_len() > self.max_idle {
            let oldest = match self.lru_order.pop_front() {
                Some(k) => k,
                None => return,
            };
            match oldest {
                LruKey::Blocking(k) => {
                    self.blocking_connections.remove(&k);
                }
                #[cfg(feature = "async")]
                LruKey::Async(k) => {
                    self.async_connections.remove(&k);
                }
            }
        }
    }
}

/// Extension trait for [`Request`] to use with [`Client`].
pub trait RequestExt {
    /// Sends this request using the provided client's connection pool.
    fn send_with_client(self, client: &Client) -> Result<Response, Error>;

    /// Sends this request asynchronously using the provided client's connection pool.
    #[cfg(feature = "async")]
    fn send_async_with_client(
        self,
        client: &Client,
    ) -> impl std::future::Future<Output = Result<Response, Error>>;
}

impl RequestExt for Request {
    fn send_with_client(self, client: &Client) -> Result<Response, Error> { client.send(self) }

    #[cfg(feature = "async")]
    fn send_async_with_client(
        self,
        client: &Client,
    ) -> impl std::future::Future<Output = Result<Response, Error>> {
        client.send_async(self)
    }
}
