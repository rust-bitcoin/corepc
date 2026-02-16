#[cfg(feature = "rustls")]
use rustls::RootCertStore;
#[cfg(feature = "rustls-webpki")]
use webpki_roots::TLS_SERVER_ROOTS;

use crate::Error;

#[derive(Clone)]
pub(crate) struct Certificates {
    pub(crate) inner: RootCertStore,
}

impl Certificates {
    pub(crate) fn new(certificate: Option<&Vec<u8>>) -> Result<Self, Error> {
        let certificates = Self { inner: RootCertStore::empty() };

        if let Some(certificate) = certificate {
            certificates.append_certificate(certificate)
        } else {
            Ok(certificates)
        }
    }

    #[cfg(feature = "rustls")]
    pub(crate) fn append_certificate(mut self, certificate: &[u8]) -> Result<Self, Error> {
        let mut certificates = self.inner;
        certificates
            .add(&rustls::Certificate(certificate.to_owned()))
            .map_err(Error::RustlsAppendCert)?;
        self.inner = certificates;
        Ok(self)
    }

    #[cfg(feature = "rustls")]
    pub(crate) fn with_root_certificates(mut self) -> Self {
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
