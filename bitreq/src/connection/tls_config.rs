use std::sync::Arc;

#[cfg(feature = "rustls")]
use rustls::RootCertStore;
#[cfg(feature = "rustls-webpki")]
use webpki_roots::TLS_SERVER_ROOTS;

use crate::Error;

#[cfg(feature = "rustls")]
pub(crate) struct TlsConfigBuilder {
    pub(crate) inner: RootCertStore,
    pub(crate) disable_default: bool,
}

#[cfg(feature = "tokio-rustls")]
impl TlsConfigBuilder {
    pub(crate) fn new(cert_der: Option<Vec<u8>>) -> Result<Self, Error> {
        let mut tls_config = Self { inner: RootCertStore::empty(), disable_default: false };

        if let Some(cert_der) = cert_der {
            tls_config.append_certificate(cert_der)?;
        }

        Ok(tls_config)
    }

    pub(crate) fn append_certificate(&mut self, cert_der: Vec<u8>) -> Result<&mut Self, Error> {
        self.inner.add(&rustls::Certificate(cert_der)).map_err(Error::RustlsAppendCert)?;

        Ok(self)
    }

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

#[derive(Clone)]
#[cfg(feature = "rustls")]
pub(crate) struct TlsConfig {
    pub(crate) certificates: Arc<RootCertStore>,
}
