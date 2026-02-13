//! TLS connection handling functionality - supports both `rustls` and `native-tls` backends.
//! When both features are enabled, rustls takes precedence.

#[cfg(feature = "rustls")]
use alloc::sync::Arc;
#[cfg(feature = "rustls")]
use core::convert::TryFrom;
use std::io;
use std::net::TcpStream;
use std::sync::OnceLock;

#[cfg(feature = "rustls")]
use crate::client::ClientConfig as CustomClientConfig;
#[cfg(all(feature = "native-tls", not(feature = "rustls")))]
use native_tls::{HandshakeError, TlsConnector, TlsStream};
#[cfg(feature = "rustls")]
use rustls::{self, ClientConfig, ClientConnection, RootCertStore, ServerName, StreamOwned};
#[cfg(all(feature = "native-tls", not(feature = "rustls"), feature = "tokio-native-tls"))]
use tokio_native_tls::TlsConnector as AsyncTlsConnector;
#[cfg(feature = "tokio-rustls")]
use tokio_rustls::{client::TlsStream, TlsConnector};
#[cfg(feature = "rustls-webpki")]
use webpki_roots::TLS_SERVER_ROOTS;

#[cfg(feature = "tokio-rustls")]
use super::{AsyncHttpStream, AsyncTcpStream};
#[cfg(all(feature = "native-tls", not(feature = "rustls"), feature = "tokio-native-tls"))]
use super::{AsyncHttpStream, AsyncTcpStream};
use crate::Error;

#[derive(Clone)]
pub(crate) struct Certificates {
    pub(crate) inner: RootCertStore,
}

impl Certificates {
    pub(crate) fn new(certificate: Option<&Vec<u8>>) -> Result<Self, Error> {
        let certificates = Self { inner: RootCertStore::empty() };

        let result = if let Some(certificate) = certificate {
            certificates.append_certificate(certificate)
        } else {
            Ok(certificates)
        };
        result
    }

    #[cfg(feature = "rustls")]
    pub(crate) fn append_certificate(mut self, certificate: &Vec<u8>) -> Result<Self, Error> {
        let mut certificates = self.inner;
        certificates
            .add(&rustls::Certificate(certificate.clone()))
            .map_err(Error::RustlsCreateConnection)?;
        self.inner = certificates;
        Ok(self)
    }

    #[cfg(feature = "rustls")]
    fn with_root_certificates(mut self) -> Self {
        let mut root_certificates = self.inner;

        // Try to load native certs
        #[cfg(feature = "https-rustls-probe")]
        if let Ok(os_roots) = rustls_native_certs::load_native_certs() {
            for root_cert in os_roots {
                // Ignore erroneous OS certificates, there's nothing
                // to do differently in that situation anyways.
                let _ = root_certificates.add(&rustls::Certificate(root_cert.0));
            }
        }

        #[cfg(feature = "rustls-webpki")]
        {
            #[allow(deprecated)]
            // Need to use add_server_trust_anchors to compile with rustls 0.21.1
            root_certificates.add_server_trust_anchors(TLS_SERVER_ROOTS.iter().map(|ta| {
                rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
                    ta.subject,
                    ta.spki,
                    ta.name_constraints,
                )
            }));
        }
        self.inner = root_certificates;
        self
    }
}

#[cfg(feature = "rustls")]
pub type SecuredStream = StreamOwned<ClientConnection, TcpStream>;

#[cfg(feature = "rustls")]
static CONFIG: OnceLock<Arc<ClientConfig>> = OnceLock::new();

#[cfg(feature = "rustls")]
fn build_client_config() -> Arc<ClientConfig> {
    let mut root_certificates = RootCertStore::empty();

    // Try to load native certs
    #[cfg(feature = "https-rustls-probe")]
    if let Ok(os_roots) = rustls_native_certs::load_native_certs() {
        for root_cert in os_roots {
            // Ignore erroneous OS certificates, there's nothing
            // to do differently in that situation anyways.
            let _ = root_certificates.add(&rustls::Certificate(root_cert.0));
        }
    }

    #[cfg(feature = "rustls-webpki")]
    #[allow(deprecated)] // Need to use add_server_trust_anchors to compile with rustls 0.21.1
    root_certificates.add_server_trust_anchors(TLS_SERVER_ROOTS.iter().map(|ta| {
        rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
            ta.subject,
            ta.spki,
            ta.name_constraints,
        )
    }));

    let config = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_certificates)
        .with_no_client_auth();
    Arc::new(config)
}

#[cfg(feature = "rustls")]
fn build_root_certificates() -> RootCertStore {
    let mut root_certificates = RootCertStore::empty();

    // Try to load native certs
    #[cfg(feature = "https-rustls-probe")]
    if let Ok(os_roots) = rustls_native_certs::load_native_certs() {
        for root_cert in os_roots {
            // Ignore erroneous OS certificates, there's nothing
            // to do differently in that situation anyways.
            let _ = root_certificates.add(&rustls::Certificate(root_cert.0));
        }
    }

    #[cfg(feature = "rustls-webpki")]
    {
        #[allow(deprecated)] // Need to use add_server_trust_anchors to compile with rustls 0.21.1
        root_certificates.add_server_trust_anchors(TLS_SERVER_ROOTS.iter().map(|ta| {
            rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
                ta.subject,
                ta.spki,
                ta.name_constraints,
            )
        }));
    }
    root_certificates
}

#[cfg(feature = "rustls")]
fn append_certificate(mut certificates: RootCertStore, certificate: Vec<u8>) -> RootCertStore {
    match certificates.add(&rustls::Certificate(certificate)) {
        Ok(_) => println!("Certificate added successfully"),
        Err(e) => println!("Failed to add certificate: {:?}", e),
    }
    certificates
}

#[cfg(feature = "rustls")]
fn build_rustls_client_config(certificates: RootCertStore) -> Arc<ClientConfig> {
    let config = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(certificates)
        .with_no_client_auth();
    Arc::new(config)
}

#[cfg(feature = "rustls")]
pub(super) fn wrap_stream(tcp: TcpStream, host: &str) -> Result<SecuredStream, Error> {
    #[cfg(feature = "log")]
    log::trace!("Setting up TLS parameters for {host}.");
    let dns_name = match ServerName::try_from(host) {
        Ok(result) => result,
        Err(err) => return Err(Error::IoError(io::Error::new(io::ErrorKind::Other, err))),
    };
    let sess = ClientConnection::new(CONFIG.get_or_init(build_client_config).clone(), dns_name)
        .map_err(Error::RustlsCreateConnection)?;

    #[cfg(feature = "log")]
    log::trace!("Establishing TLS session to {host}.");
    Ok(StreamOwned::new(sess, tcp))
}

// Async rustls TLS implementation

#[cfg(all(feature = "rustls", feature = "tokio-rustls"))]
pub type AsyncSecuredStream = TlsStream<tokio::net::TcpStream>;

#[cfg(all(feature = "rustls", feature = "tokio-rustls"))]
pub(super) async fn wrap_async_stream(
    tcp: AsyncTcpStream,
    host: &str,
) -> Result<AsyncHttpStream, Error> {
    #[cfg(feature = "log")]
    log::trace!("Setting up TLS parameters for {host}.");
    let dns_name = match ServerName::try_from(host) {
        Ok(result) => result,
        Err(err) => return Err(Error::IoError(io::Error::new(io::ErrorKind::Other, err))),
    };

    let connector = TlsConnector::from(CONFIG.get_or_init(build_client_config).clone());

    #[cfg(feature = "log")]
    log::trace!("Establishing TLS session to {host}.");

    let tls = connector.connect(dns_name, tcp).await.map_err(Error::IoError)?;

    Ok(AsyncHttpStream::Secured(Box::new(tls)))
}

#[cfg(all(feature = "rustls", feature = "tokio-rustls"))]
pub(super) async fn wrap_async_stream_with_configs(
    tcp: AsyncTcpStream,
    host: &str,
    custom_client_config: CustomClientConfig,
) -> Result<AsyncHttpStream, Error> {
    #[cfg(feature = "log")]
    log::trace!("Setting up TLS parameters for {host}.");
    let dns_name = match ServerName::try_from(host) {
        Ok(result) => result,
        Err(err) => return Err(Error::IoError(io::Error::new(io::ErrorKind::Other, err))),
    };

    let mut certificates = custom_client_config.tls.unwrap().certificates;
    certificates = certificates.with_root_certificates();

    let client_config = build_rustls_client_config(certificates.inner);
    let connector = TlsConnector::from(CONFIG.get_or_init(|| client_config).clone());

    #[cfg(feature = "log")]
    log::trace!("Establishing TLS session to {host}.");

    let tls = connector.connect(dns_name, tcp).await.map_err(Error::IoError)?;

    Ok(AsyncHttpStream::Secured(Box::new(tls)))
}

#[cfg(all(feature = "native-tls", not(feature = "rustls")))]
pub type SecuredStream = TlsStream<TcpStream>;

#[cfg(all(feature = "native-tls", not(feature = "rustls")))]
static CONNECTOR: OnceLock<Result<TlsConnector, Error>> = OnceLock::new();

#[cfg(all(feature = "native-tls", not(feature = "rustls")))]
fn native_tls_err<S>(e: HandshakeError<S>) -> Error {
    match e {
        HandshakeError::Failure(e) => Error::NativeTlsError(e),
        HandshakeError::WouldBlock(_) => {
            debug_assert!(false, "We shouldn't hit a blocking error");
            Error::Other("Got a WouldBlock error from native-tls")
        }
    }
}

#[cfg(all(feature = "native-tls", not(feature = "rustls")))]
fn build_tls_connector() -> Result<TlsConnector, Error> {
    TlsConnector::builder().build().map_err(Error::NativeTlsError)
}

#[cfg(all(feature = "native-tls", not(feature = "rustls")))]
pub(super) fn wrap_stream(tcp: TcpStream, host: &str) -> Result<SecuredStream, Error> {
    #[cfg(feature = "log")]
    log::trace!("Setting up TLS parameters for {host}.");

    // TODO: Once we can `get_or_try_init`, so that instead
    // https://github.com/rust-lang/rust/issues/109737
    let connector = CONNECTOR.get_or_init(build_tls_connector)?;

    #[cfg(feature = "log")]
    log::trace!("Establishing TLS session to {host}.");

    connector.connect(host, tcp).map_err(native_tls_err)
}

#[cfg(all(feature = "native-tls", not(feature = "rustls"), feature = "tokio-native-tls"))]
pub type AsyncSecuredStream = tokio_native_tls::TlsStream<tokio::net::TcpStream>;

#[cfg(all(feature = "native-tls", not(feature = "rustls"), feature = "tokio-native-tls"))]
pub(super) async fn wrap_async_stream(
    tcp: AsyncTcpStream,
    host: &str,
) -> Result<AsyncHttpStream, Error> {
    #[cfg(feature = "log")]
    log::trace!("Setting up TLS parameters for {host}.");

    // TODO: Once we can `get_or_try_init`, so that instead
    // https://github.com/rust-lang/rust/issues/109737
    let connector = AsyncTlsConnector::from(CONNECTOR.get_or_init(build_tls_connector)?.clone());

    #[cfg(feature = "log")]
    log::trace!("Establishing TLS session to {host}.");

    let tls = connector.connect(host, tcp).await.map_err(native_tls_err)?;

    Ok(AsyncHttpStream::Secured(Box::new(tls)))
}
