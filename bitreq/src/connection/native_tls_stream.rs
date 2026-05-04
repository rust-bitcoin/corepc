//! Native-TLS connection handling functionality.
//! This module is only compiled when a native-tls HTTPS feature is enabled
//! AND no rustls feature is enabled (mutual exclusion enforced at module level).

use std::io;
use std::net::TcpStream;
use std::sync::OnceLock;

use native_tls::{HandshakeError, TlsConnector, TlsStream};
#[cfg(feature = "async-https-native-tls")]
use tokio_native_tls::TlsConnector as AsyncTlsConnector;

use super::HttpStream;
#[cfg(feature = "async-https-native-tls")]
use super::{AsyncHttpStream, AsyncTcpStream};
use crate::Error;

// === SYNC native-tls ===

pub type SecuredStream = TlsStream<TcpStream>;

static CONNECTOR: OnceLock<Result<TlsConnector, Error>> = OnceLock::new();

fn native_tls_err<S>(e: HandshakeError<S>) -> Error {
    match e {
        HandshakeError::Failure(err) => Error::NativeTlsCreateConnection(err),
        HandshakeError::WouldBlock(_) => {
            debug_assert!(false, "We shouldn't hit a blocking error");
            Error::Other("Got a WouldBlock error from native-tls")
        }
    }
}

fn build_tls_connector() -> Result<TlsConnector, Error> {
    TlsConnector::builder().build().map_err(Error::from)
}

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

// === ASYNC native-tls ===

#[cfg(feature = "async-https-native-tls")]
pub type AsyncSecuredStream = tokio_native_tls::TlsStream<tokio::net::TcpStream>;

#[cfg(feature = "async-https-native-tls")]
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
