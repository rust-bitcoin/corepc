//! Connection pooling client for HTTP requests.
//!
//! The `Client` caches connections to avoid repeated TCP handshakes and TLS negotiations.
//!
//! Due to std limitations, `Client` currently only supports async requests.

#![cfg(feature = "async")]

use std::collections::{hash_map, HashMap, VecDeque};
use std::sync::{Arc, Mutex};

use crate::connection::certificates::Certificates;
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
#[derive(Clone)]
pub struct Client {
    r#async: Arc<Mutex<ClientImpl<AsyncConnection>>>,
}

struct ClientImpl<T> {
    connections: HashMap<ConnectionKey, Arc<T>>,
    lru_order: VecDeque<ConnectionKey>,
    capacity: usize,
    client_config: Option<ClientConfig>,
}

pub struct ClientBuilder {
    capacity: usize,
    client_config: Option<ClientConfig>,
}

#[derive(Clone)]
pub struct ClientConfig {
    pub tls: Option<TlsConfig>,
}

#[derive(Clone)]
pub struct TlsConfig {
    pub certificates: Certificates,
}

impl TlsConfig {
    fn new(certificate: Vec<u8>) -> Self {
        let certificates =
            Certificates::new(Some(&certificate)).expect("failed to append certificate");

        Self { certificates: certificates }
    }
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self { capacity: 1, client_config: None }
    }

    pub fn with_root_certificate<T: Into<Vec<u8>>>(mut self, certificate: T) -> Self {
        let tls_config = TlsConfig::new(certificate.into());
        self.client_config = Some(ClientConfig { tls: Some(tls_config) });
        self
    }

    pub fn with_capacity(mut self, capacity: usize) -> Self {
        self.capacity = capacity;
        self
    }

    pub fn build(self) -> Client {
        Client {
            r#async: Arc::new(Mutex::new(ClientImpl {
                connections: HashMap::new(),
                lru_order: VecDeque::new(),
                capacity: self.capacity,
                client_config: self.client_config,
            })),
        }
    }
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
                client_config: None,
            })),
        }
    }

    /// Create a builder for a client
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
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
            let client_config = {
                let state = self.r#async.lock().unwrap();
                state.client_config.clone()
            };

            let connection =
                AsyncConnection::new(key, parsed_request.timeout_at, client_config).await?;
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
