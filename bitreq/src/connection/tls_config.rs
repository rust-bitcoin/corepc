use std::sync::Arc;

#[cfg(not(feature = "rustls"))]
use native_tls::{Certificate, TlsConnector, TlsConnectorBuilder};
#[cfg(feature = "rustls")]
use rustls::pki_types::CertificateDer;
#[cfg(feature = "rustls")]
use rustls::RootCertStore;
#[cfg(not(feature = "rustls"))]
use tokio_native_tls::TlsConnector as AsyncTlsConnector;
#[cfg(feature = "rustls-webpki")]
use webpki_roots::TLS_SERVER_ROOTS;

use crate::Error;

#[cfg(feature = "rustls")]
pub(crate) struct TlsConfigBuilder {
    pub(crate) inner: RootCertStore,
    pub(crate) disable_default: bool,
}

#[cfg(not(feature = "rustls"))]
pub(crate) struct TlsConfigBuilder {
    pub(crate) inner: TlsConnectorBuilder,
}

#[cfg(feature = "rustls")]
impl TlsConfigBuilder {
    pub(crate) fn new(cert_der: Option<Vec<u8>>) -> Result<Self, Error> {
        let mut tls_config = Self { inner: RootCertStore::empty(), disable_default: false };

        if let Some(cert_der) = cert_der {
            tls_config.append_certificate(cert_der)?;
        }

        Ok(tls_config)
    }

    pub(crate) fn append_certificate(&mut self, cert_der: Vec<u8>) -> Result<&mut Self, Error> {
        self.inner.add(CertificateDer::from(cert_der)).map_err(Error::RustlsAppendCert)?;

        Ok(self)
    }

    fn with_root_certificates(&mut self) -> &mut Self {
        // Try to load native certs
        #[cfg(feature = "https-rustls-probe")]
        for cert in rustls_native_certs::load_native_certs().certs {
            let _ = self.inner.add(cert);
        }

        #[cfg(feature = "rustls-webpki")]
        {
            self.inner.extend(TLS_SERVER_ROOTS.iter().cloned());
        }
        self
    }

    pub(crate) fn disable_default_certificates(&mut self) -> Result<&mut Self, Error> {
        self.disable_default = true;
        Ok(self)
    }

    pub(crate) fn build(mut self) -> Result<TlsConfig, Error> {
        if !self.disable_default {
            self.with_root_certificates();
        }

        Ok(TlsConfig { certificates: Arc::new(self.inner) })
    }
}

#[cfg(not(feature = "rustls"))]
impl TlsConfigBuilder {
    pub(crate) fn new(cert_der: Option<Vec<u8>>) -> Result<Self, Error> {
        let builder = TlsConnector::builder();
        let mut tls_config = Self { inner: builder };

        if let Some(cert_der) = cert_der {
            tls_config.append_certificate(cert_der)?;
        }

        Ok(tls_config)
    }

    pub(crate) fn append_certificate(&mut self, cert_der: Vec<u8>) -> Result<&mut Self, Error> {
        let certificate = Certificate::from_der(&cert_der)?;
        self.inner.add_root_certificate(certificate);

        Ok(self)
    }

    pub(crate) fn disable_default_certificates(&mut self) -> Result<&mut Self, Error> {
        self.inner.disable_built_in_roots(true);
        Ok(self)
    }

    pub(crate) fn build(self) -> Result<TlsConfig, Error> {
        let connector = self.inner.build()?;
        let async_connector = AsyncTlsConnector::from(connector);

        Ok(TlsConfig { connector: Arc::new(async_connector) })
    }
}

#[derive(Clone)]
#[cfg(feature = "rustls")]
pub(crate) struct TlsConfig {
    pub(crate) certificates: Arc<RootCertStore>,
}

#[derive(Clone)]
#[cfg(not(feature = "rustls"))]
pub(crate) struct TlsConfig {
    pub(crate) connector: Arc<AsyncTlsConnector>,
}
