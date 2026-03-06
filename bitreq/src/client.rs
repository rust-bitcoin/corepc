//! Connection pooling client for HTTP requests.
//!
//! The `Client` caches connections to avoid repeated TCP handshakes and TLS negotiations.
//!
//! Due to std limitations, `Client` currently only supports async requests.

#![cfg(feature = "async")]

use std::collections::{hash_map, HashMap, VecDeque};
use std::sync::{Arc, Mutex};

#[cfg(all(feature = "rustls", feature = "tokio-rustls"))]
use crate::connection::certificates::{Certificates, CertificatesBuilder};
use crate::connection::AsyncConnection;
use crate::request::{OwnedConnectionParams as ConnectionKey, ParsedRequest};
use crate::{Error, Request, Response};

#[derive(Clone)]
pub(crate) struct ClientConfig {
    #[cfg(all(feature = "rustls", feature = "tokio-rustls"))]
    pub(crate) tls: Option<TlsConfig>,
}

#[cfg(all(feature = "rustls", feature = "tokio-rustls"))]
#[derive(Clone)]
pub(crate) struct TlsConfig {
    pub(crate) certificates: Certificates,
}

#[cfg(all(feature = "rustls", feature = "tokio-rustls"))]
impl TlsConfig {
    fn new(certificates: Certificates) -> Self { Self { certificates } }
}

pub struct ClientBuilder {
    capacity: usize,
    #[cfg(all(feature = "rustls", feature = "tokio-rustls"))]
    certificates: Option<CertificatesBuilder>,
}

/// Builder for configuring a `Client` with custom settings.
///
/// # Example
///
/// ```no_run
/// # async fn example() -> Result<(), bitreq::Error> {
/// use bitreq::{Client, RequestExt};
///
/// let client = Client::builder().with_capacity(20).build()?;
///
/// let response = bitreq::get("https://example.com")
///     .send_async_with_client(&client)
///     .await?;
/// # Ok(())
/// # }
/// ```
impl ClientBuilder {
    /// Creates a new `ClientBuilder` with a default pool capacity of 10.
    #[cfg(all(feature = "rustls", feature = "tokio-rustls"))]
    pub fn new() -> Self { Self { capacity: 10, certificates: None } }

    /// Creates a new `ClientBuilder` with a default pool capacity of 10.
    #[cfg(not(all(feature = "rustls", feature = "tokio-rustls")))]
    pub fn new() -> Self { Self { capacity: 10 } }

    /// Sets the maximum number of connections to keep in the pool.
    pub fn with_capacity(mut self, capacity: usize) -> Self {
        self.capacity = capacity;
        self
    }

    #[cfg(all(feature = "rustls", feature = "tokio-rustls"))]
    /// Builds the `Client` with the configured settings.
    pub fn build(self) -> Result<Client, Error> {
        let build_config = if let Some(builder) = self.certificates {
            let certificates = builder.build()?;
            let tls_config = TlsConfig::new(certificates);
            Some(ClientConfig { tls: Some(tls_config) })
        } else {
            None
        };
        let client_config = build_config.map(Arc::new);

        Ok(Client {
            r#async: Arc::new(Mutex::new(ClientImpl {
                connections: HashMap::new(),
                lru_order: VecDeque::new(),
                capacity: self.capacity,
                client_config,
            })),
        })
    }

    /// Builds the `Client` with the configured settings.
    #[cfg(not(any(all(feature = "rustls", feature = "tokio-rustls"))))]
    pub fn build(self) -> Result<Client, Error> {
        Ok(Client {
            r#async: Arc::new(Mutex::new(ClientImpl {
                connections: HashMap::new(),
                lru_order: VecDeque::new(),
                capacity: self.capacity,
                client_config: None,
            })),
        })
    }

    /// Adds a custom DER-encoded root certificate for TLS verification.
    /// The certificate must be provided in DER format. This method accepts any type
    /// that can be converted into a `Vec<u8>`.
    /// The certificate is appended to the default trust store rather than replacing it.
    /// The trust store used depends on the TLS backend: system certificates for native-tls,
    /// Mozilla's root certificates(rustls-webpki) and/or system certificates(rustls-native-certs) for rustls.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bitreq::Client;
    /// # async fn example() -> Result<(), bitreq::Error> {
    /// let client = Client::builder()
    ///     .with_root_certificate(include_bytes!("../tests/test_cert.der"))?
    ///     .build()?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(all(feature = "rustls", feature = "tokio-rustls"))]
    pub fn with_root_certificate<T: Into<Vec<u8>>>(mut self, cert_der: T) -> Result<Self, Error> {
        let cert_der = cert_der.into();
        if let Some(ref mut certificates) = self.certificates {
            certificates.append_certificate(cert_der)?;

            return Ok(self);
        }

        self.certificates = Some(CertificatesBuilder::new(Some(cert_der))?);
        Ok(self)
    }

    /// Disables default root certificates for TLS connections.
    /// Returns [`Error::InvalidTlsConfig`] if TLS has not been configured.
    #[cfg(all(feature = "rustls", feature = "tokio-rustls"))]
    pub fn disable_default_certificates(mut self) -> Result<Self, Error> {
        match self.certificates {
            Some(ref mut certificates) => certificates.disable_default()?,
            None => return Err(Error::InvalidTlsConfig),
        };

        Ok(self)
    }
}

impl Default for ClientBuilder {
    fn default() -> Self { Self::new() }
}

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
    client_config: Option<Arc<ClientConfig>>,
}

impl Client {
    /// Creates a new `Client` with the specified connection pool capacity.
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
    pub fn builder() -> ClientBuilder { ClientBuilder::new() }

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
                state.client_config.as_ref().map(Arc::clone)
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
