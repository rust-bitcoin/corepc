#[cfg(any(feature = "rustls", feature = "native-tls"))]
use std::sync::Arc;

#[cfg(all(feature = "native-tls", not(feature = "rustls")))]
use native_tls::{Certificate, TlsConnector, TlsConnectorBuilder};
#[cfg(feature = "rustls")]
use rustls::RootCertStore;
#[cfg(all(feature = "native-tls", not(feature = "rustls"), feature = "tokio-native-tls"))]
use tokio_native_tls::TlsConnector as AsyncTlsConnector;
#[cfg(feature = "rustls-webpki")]
use webpki_roots::TLS_SERVER_ROOTS;

use crate::Error;

#[cfg(all(feature = "rustls", feature = "tokio-rustls"))]
pub(crate) struct CertificatesBuilder {
    pub(crate) inner: RootCertStore,
    pub(crate) disable_default: bool,
}

#[cfg(all(feature = "native-tls", not(feature = "rustls"), feature = "tokio-native-tls"))]
pub(crate) struct CertificatesBuilder {
    pub(crate) inner: TlsConnectorBuilder,
}

impl CertificatesBuilder {
    #[cfg(all(feature = "rustls", feature = "tokio-rustls"))]
    pub(crate) fn new(cert_der: Option<Vec<u8>>) -> Result<Self, Error> {
        let mut certificates = Self { inner: RootCertStore::empty(), disable_default: false };

        if let Some(cert_der) = cert_der {
            certificates.append_certificate(cert_der)?;
        }

        Ok(certificates)
    }

    #[cfg(all(feature = "native-tls", not(feature = "rustls"), feature = "tokio-native-tls"))]
    pub(crate) fn new(cert_der: Option<Vec<u8>>) -> Result<Self, Error> {
        let builder = TlsConnector::builder();
        let mut certificates = Self { inner: builder };

        if let Some(cert_der) = cert_der {
            certificates.append_certificate(cert_der)?;
        }

        Ok(certificates)
    }

    #[cfg(all(feature = "rustls", feature = "tokio-rustls"))]
    pub(crate) fn append_certificate(&mut self, cert_der: Vec<u8>) -> Result<&mut Self, Error> {
        self.inner.add(&rustls::Certificate(cert_der)).map_err(Error::RustlsAppendCert)?;

        Ok(self)
    }

    #[cfg(all(feature = "native-tls", not(feature = "rustls"), feature = "tokio-native-tls"))]
    pub(crate) fn append_certificate(&mut self, cert_der: Vec<u8>) -> Result<&mut Self, Error> {
        let certificate = Certificate::from_der(&cert_der)?;
        self.inner.add_root_certificate(certificate);

        Ok(self)
    }

    #[cfg(all(feature = "native-tls", not(feature = "rustls"), feature = "tokio-native-tls"))]
    pub(crate) fn build(self) -> Result<Certificates, Error> {
        let connector = self.inner.build()?;
        let async_connector = AsyncTlsConnector::from(connector);

        Ok(Certificates(Arc::new(async_connector)))
    }

    #[cfg(all(feature = "rustls", feature = "tokio-rustls"))]
    pub(crate) fn build(mut self) -> Result<Certificates, Error> {
        if !self.disable_default {
            self.with_root_certificates();
        }

        Ok(Certificates(Arc::new(self.inner)))
    }

    #[cfg(all(feature = "rustls", feature = "tokio-rustls"))]
    fn with_root_certificates(&mut self) -> &mut Self {
        // Try to load native certs
        #[cfg(feature = "https-rustls-probe")]
        if let Ok(os_roots) = rustls_native_certs::load_native_certs() {
            for root_cert in os_roots {
                // Ignore erroneous OS certificates, there's nothing
                // to do differently in that situation anyways.
                let _ = self.inner.add(&rustls::Certificate(root_cert.0));
            }
        }

        #[cfg(feature = "rustls-webpki")]
        {
            #[allow(deprecated)]
            // Need to use add_server_trust_anchors to compile with rustls 0.21.1
            self.inner.add_server_trust_anchors(TLS_SERVER_ROOTS.iter().map(|ta| {
                rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
                    ta.subject,
                    ta.spki,
                    ta.name_constraints,
                )
            }));
        }
        self
    }

    #[cfg(all(feature = "rustls", feature = "tokio-rustls"))]
    pub(crate) fn disable_default(&mut self) -> Result<&mut Self, Error> {
        self.disable_default = true;
        Ok(self)
    }

    #[cfg(all(feature = "native-tls", not(feature = "rustls"), feature = "tokio-native-tls"))]
    pub(crate) fn disable_default(&mut self) -> Result<&mut Self, Error> {
        self.inner.disable_built_in_roots(true);
        Ok(self)
    }
}

#[derive(Clone)]
#[cfg(all(feature = "rustls", feature = "tokio-rustls"))]
pub(crate) struct Certificates(pub(crate) Arc<RootCertStore>);

#[derive(Clone)]
#[cfg(all(feature = "native-tls", not(feature = "rustls"), feature = "tokio-native-tls"))]
pub(crate) struct Certificates(pub(crate) Arc<AsyncTlsConnector>);
