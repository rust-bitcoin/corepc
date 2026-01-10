//! TLS connection handling functionality when using the `rustls` crate for
//! handling TLS.

use alloc::sync::Arc;
use core::convert::TryFrom;
use std::io;
use std::net::TcpStream;
use std::sync::OnceLock;

use rustls::{self, ClientConfig, ClientConnection, RootCertStore, ServerName, StreamOwned};
#[cfg(feature = "async-https")]
use tokio_rustls::{client::TlsStream, TlsConnector};
#[cfg(feature = "rustls-webpki")]
use webpki_roots::TLS_SERVER_ROOTS;

#[cfg(feature = "async-https")]
use super::{AsyncHttpStream, AsyncTcpStream};
use crate::Error;

pub type SecuredStream = StreamOwned<ClientConnection, TcpStream>;

static CONFIG: OnceLock<Arc<ClientConfig>> = OnceLock::new();

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

// Async TLS implementation

#[cfg(feature = "async-https")]
pub type AsyncSecuredStream = TlsStream<tokio::net::TcpStream>;

#[cfg(feature = "async-https")]
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

    let tls = connector
        .connect(dns_name, tcp)
        .await
        .map_err(|e| Error::IoError(io::Error::new(io::ErrorKind::Other, e)))?;

    Ok(AsyncHttpStream::Secured(Box::new(tls)))
}
