//! Rustls-based TLS connection handling functionality.

#[cfg(feature = "rustls")]
use alloc::sync::Arc;
#[cfg(feature = "rustls")]
use std::io;
use std::net::TcpStream;
use std::sync::OnceLock;

#[cfg(feature = "rustls")]
use rustls::pki_types::ServerName;
#[cfg(feature = "rustls")]
use rustls::{self, ClientConfig, ClientConnection, RootCertStore, StreamOwned};
#[cfg(any(feature = "async-https-rustls", feature = "async-https-rustls-probe"))]
use tokio_rustls::{client::TlsStream, TlsConnector};
#[cfg(feature = "rustls-webpki")]
use webpki_roots::TLS_SERVER_ROOTS;

#[cfg(feature = "rustls")]
use super::HttpStream;
#[cfg(any(feature = "async-https-rustls", feature = "async-https-rustls-probe"))]
use super::{AsyncHttpStream, AsyncTcpStream};
use crate::Error;

// === SYNC rustls ===

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

// === ASYNC rustls ===

#[cfg(any(feature = "async-https-rustls", feature = "async-https-rustls-probe"))]
pub type AsyncSecuredStream = TlsStream<tokio::net::TcpStream>;

#[cfg(any(feature = "async-https-rustls", feature = "async-https-rustls-probe"))]
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
