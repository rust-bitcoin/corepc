//! Connection pooling client for HTTP requests.
//!
//! The `Client` caches connections to avoid repeated TCP handshakes and TLS negotiations.
//!
//! Due to std limitations, `Client` currently only supports async requests.

#![cfg(feature = "async")]

use std::collections::{hash_map, HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use crate::connection::AsyncConnection;
use crate::request::{OwnedConnectionParams as ConnectionKey, ParsedRequest};
use crate::{Error, Request, Response};

/// A client that caches connections for reuse.
///
/// The client maintains a pool of up to `capacity` connections, evicting
/// the least recently used connection when the cache is full. Pooled
/// connections are validated on every acquire: an entry whose keep-alive
/// deadline has passed — or whose underlying socket has been poisoned by
/// a previous failure — is dropped and a fresh connection is opened.
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
#[derive(Clone)]
pub struct Client {
    r#async: Arc<Mutex<ClientImpl<AsyncConnection>>>,
}

struct ClientImpl<T> {
    connections: HashMap<ConnectionKey, Arc<T>>,
    lru_order: VecDeque<ConnectionKey>,
    capacity: usize,
}

impl Client {
    /// Creates a new `Client` with the specified connection cache capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - Maximum number of cached connections. When this limit is
    ///   reached, the least recently used connection is evicted.
    pub fn new(capacity: usize) -> Self {
        Client {
            r#async: Arc::new(Mutex::new(ClientImpl {
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
        let owned_key: ConnectionKey = key.into();

        let conn = match self.acquire_pooled(&owned_key) {
            Some(conn) => conn,
            None => {
                // On a miss, pre-insert the fresh `Arc` so concurrent
                // callers arriving before this send completes can clone
                // it and share the socket for pipelining. A send failure
                // or non-keep-alive response will evict in the post-send
                // check below; the `reusable_until` probe in
                // `acquire_pooled` keeps subsequent callers from using a
                // poisoned `Arc` even during that window.
                let conn = Arc::new(AsyncConnection::new(key, parsed_request.timeout_at).await?);
                self.insert_if_vacant(owned_key.clone(), Arc::clone(&conn));
                conn
            }
        };

        let result = conn.send(parsed_request).await;

        // Evict when the send poisoned the connection — covers write /
        // read errors, `Connection: close`, and malformed `Keep-Alive`,
        // all of which `AsyncConnection::send` signals by setting
        // `next_request_id = usize::MAX`.
        if conn.reusable_until().is_none() {
            self.evict(&owned_key);
        }

        result
    }

    /// Returns a pooled connection for `key` if one is present and still
    /// reusable per its own [`AsyncConnection::reusable_until`] — no
    /// sidecar expiry needs to be tracked because the connection already
    /// refreshes its `socket_new_requests_timeout` from the server's
    /// `Keep-Alive: timeout=N` header on every successful response.
    /// Otherwise evicts the stale entry and returns `None`.
    fn acquire_pooled(&self, key: &ConnectionKey) -> Option<Arc<AsyncConnection>> {
        let mut state = self.r#async.lock().unwrap();
        let conn = state.connections.get(key)?;
        let reusable = conn.reusable_until().is_some_and(|t| t > Instant::now());
        if !reusable {
            state.connections.remove(key);
            if let Some(pos) = state.lru_order.iter().position(|k| k == key) {
                state.lru_order.remove(pos);
            }
            return None;
        }
        let connection = Arc::clone(conn);
        // Refresh LRU position so this hit is treated as the most recent use.
        if let Some(pos) = state.lru_order.iter().position(|k| k == key) {
            state.lru_order.remove(pos);
        }
        state.lru_order.push_back(key.clone());
        Some(connection)
    }

    /// Inserts `connection` under `key` only if the slot is vacant. On a
    /// pool-hit the entry is already there (we cloned the `Arc` during
    /// acquire), so this is a no-op on that path. On a pool-miss, a
    /// concurrent caller may have raced us and already placed a different
    /// `Arc` under this key — "first writer wins," and we drop ours.
    fn insert_if_vacant(&self, key: ConnectionKey, connection: Arc<AsyncConnection>) {
        let mut state = self.r#async.lock().unwrap();
        if let hash_map::Entry::Vacant(entry) = state.connections.entry(key.clone()) {
            entry.insert(connection);
            state.lru_order.push_back(key);
            while state.connections.len() > state.capacity {
                let oldest = match state.lru_order.pop_front() {
                    Some(k) => k,
                    None => break,
                };
                state.connections.remove(&oldest);
            }
        }
    }

    /// Removes any pool entry for `key`. No-op if the slot is already empty.
    fn evict(&self, key: &ConnectionKey) {
        let mut state = self.r#async.lock().unwrap();
        state.connections.remove(key);
        if let Some(pos) = state.lru_order.iter().position(|k| k == key) {
            state.lru_order.remove(pos);
        }
    }
}

/// Extension trait for `Request` to use with `Client`.
pub trait RequestExt {
    /// Sends this request asynchronously using the provided client's connection pool.
    fn send_async_with_client(
        self,
        client: &Client,
    ) -> impl std::future::Future<Output = Result<Response, Error>>;
}

impl RequestExt for Request {
    fn send_async_with_client(
        self,
        client: &Client,
    ) -> impl std::future::Future<Output = Result<Response, Error>> {
        client.send_async(self)
    }
}
