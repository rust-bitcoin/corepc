//! TLS connection handling functionality - supports both `rustls` and `native-tls` backends.
//! When both features are enabled, rustls takes precedence.

#[cfg(feature = "rustls")]
use alloc::sync::Arc;
#[cfg(any(feature = "rustls", feature = "native-tls"))]
use std::io;
use std::net::TcpStream;
use std::sync::OnceLock;

#[cfg(all(feature = "native-tls", not(feature = "rustls")))]
use native_tls::{HandshakeError, TlsConnector, TlsStream};
#[cfg(feature = "rustls")]
use rustls::pki_types::ServerName;
#[cfg(feature = "rustls")]
use rustls::{self, ClientConfig, ClientConnection, RootCertStore, StreamOwned};
#[cfg(all(feature = "native-tls", not(feature = "rustls"), feature = "tokio-native-tls"))]
use tokio_native_tls::TlsConnector as AsyncTlsConnector;
#[cfg(feature = "tokio-rustls")]
use tokio_rustls::{client::TlsStream, TlsConnector};
#[cfg(feature = "rustls-webpki")]
use webpki_roots::TLS_SERVER_ROOTS;

#[cfg(any(feature = "rustls", feature = "native-tls"))]
use super::HttpStream;
#[cfg(any(feature = "tokio-rustls", feature = "tokio-native-tls"))]
use super::{AsyncHttpStream, AsyncTcpStream};
use crate::Error;

#[cfg(feature = "rustls")]
pub type SecuredStream = StreamOwned<ClientConnection, TcpStream>;

#[cfg(feature = "rustls")]
static CONFIG: OnceLock<Arc<ClientConfig>> = OnceLock::new();

#[cfg(feature = "rustls")]
fn build_client_config() -> Arc<ClientConfig> {
    let mut root_certificates = RootCertStore::empty();

    #[cfg(feature = "https-rustls-probe")]
    for cert in rustls_native_certs::load_native_certs().certs {
        let _ = root_certificates.add(cert);
    }

    #[cfg(feature = "rustls-webpki")]
    root_certificates.extend(TLS_SERVER_ROOTS.iter().cloned());

    let config =
        ClientConfig::builder().with_root_certificates(root_certificates).with_no_client_auth();
    Arc::new(config)
}

#[cfg(feature = "rustls")]
pub(super) fn wrap_stream(tcp: TcpStream, host: &str) -> Result<HttpStream, Error> {
    #[cfg(feature = "log")]
    log::trace!("Setting up TLS parameters for {host}.");
    let dns_name = ServerName::try_from(host)
        .map(|name| name.to_owned())
        .map_err(|err| Error::IoError(io::Error::new(io::ErrorKind::Other, err)))?;
    let sess = ClientConnection::new(CONFIG.get_or_init(build_client_config).clone(), dns_name)
        .map_err(Error::RustlsCreateConnection)?;
    let tls = StreamOwned::new(sess, tcp);

    #[cfg(feature = "log")]
    log::trace!("Establishing TLS session to {host}.");

    Ok(HttpStream::Secured(Box::new(tls), None))
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
    let dns_name = ServerName::try_from(host)
        .map(|name| name.to_owned())
        .map_err(|err| Error::IoError(io::Error::new(io::ErrorKind::Other, err)))?;

    let connector = TlsConnector::from(CONFIG.get_or_init(build_client_config).clone());

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
        HandshakeError::Failure(err) => Error::NativeTlsCreateConnection(err),
        HandshakeError::WouldBlock(_) => {
            debug_assert!(false, "We shouldn't hit a blocking error");
            Error::Other("Got a WouldBlock error from native-tls")
        }
    }
}

#[cfg(all(feature = "native-tls", not(feature = "rustls")))]
fn build_tls_connector() -> Result<TlsConnector, Error> {
    TlsConnector::builder().build().map_err(Error::from)
}

#[cfg(all(feature = "native-tls", not(feature = "rustls")))]
pub(super) fn wrap_stream(tcp: TcpStream, host: &str) -> Result<HttpStream, Error> {
    #[cfg(feature = "log")]
    log::trace!("Setting up TLS parameters for {host}.");

    // TODO: Once we can `get_or_try_init`, so that instead
    // https://github.com/rust-lang/rust/issues/109737
    let connector = match CONNECTOR.get_or_init(build_tls_connector) {
        Ok(c) => c.clone(),
        Err(err) => return Err(Error::IoError(io::Error::new(io::ErrorKind::Other, err))),
    };

    #[cfg(feature = "log")]
    log::trace!("Establishing TLS session to {host}.");

    let tls = connector.connect(host, tcp).map_err(native_tls_err)?;

    Ok(HttpStream::Secured(Box::new(tls), None))
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
    let sync_connector = match CONNECTOR.get_or_init(build_tls_connector) {
        Ok(c) => c.clone(),
        Err(err) => return Err(Error::IoError(io::Error::new(io::ErrorKind::Other, err))),
    };

    let async_connector = AsyncTlsConnector::from(sync_connector);

    #[cfg(feature = "log")]
    log::trace!("Establishing TLS session to {host}.");

    let tls = async_connector.connect(host, tcp).await?;

    Ok(AsyncHttpStream::Secured(Box::new(tls)))
}
