//! TLS manager to reuse connections between shards.

#[cfg(any(feature = "rustls-native-roots", feature = "rustls-webpki-roots"))]
use std::sync::Arc;

use std::{
    error::Error,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
};

#[cfg(all(
    feature = "native",
    not(any(feature = "rustls-native-roots", feature = "rustls-webpki-roots"))
))]
use native_tls::TlsConnector as NativeTlsConnector;

#[cfg(any(feature = "rustls-native-roots", feature = "rustls-webpki-roots"))]
use rustls_tls::ClientConfig;

#[cfg(feature = "rustls-webpki-roots")]
use rustls_tls::OwnedTrustAnchor;

use tokio_tungstenite::Connector;

#[cfg(all(
    feature = "native",
    not(any(feature = "rustls-native-roots", feature = "rustls-webpki-roots"))
))]
pub type TlsConnector = NativeTlsConnector;

#[cfg(any(feature = "rustls-native-roots", feature = "rustls-webpki-roots"))]
pub type TlsConnector = Arc<ClientConfig>;

#[derive(Debug)]
pub struct TlsError {
    /// Type of error.
    kind: TlsErrorType,
    /// Source error if available.
    source: Option<Box<dyn Error + Send + Sync>>,
}

#[allow(dead_code)]
impl TlsError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &TlsErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        self.source
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(self) -> (TlsErrorType, Option<Box<dyn Error + Send + Sync>>) {
        (self.kind, self.source)
    }
}

impl Display for TlsError {
    // If TlsErrorType is an empty type f is not used.
    #[cfg_attr(feature = "rustls-webpki-roots", allow(unused))]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.kind {
            #[cfg(all(
                feature = "native",
                not(any(feature = "rustls-native-roots", feature = "rustls-webpki-roots"))
            ))]
            TlsErrorType::NativeTls => {
                f.write_str("construction of the native-tls connector failed")
            }
            #[cfg(feature = "rustls-native-roots")]
            TlsErrorType::NativeCerts => f.write_str("could not load native certificates"),
        }
    }
}

impl Error for TlsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn Error + 'static))
    }
}

/// Type of [`TlsError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum TlsErrorType {
    /// Construction of the nativetls connector failed.
    #[cfg(all(
        feature = "native",
        not(any(feature = "rustls-native-roots", feature = "rustls-webpki-roots"))
    ))]
    NativeTls,
    /// Could not load native certificates.
    #[cfg(feature = "rustls-native-roots")]
    NativeCerts,
}

#[derive(Clone)]
#[cfg_attr(
    all(
        feature = "native",
        not(any(feature = "rustls-native-roots", feature = "rustls-webpki-roots")),
    ),
    derive(Debug)
)]
pub struct TlsContainer {
    tls: TlsConnector,
}

#[cfg(any(feature = "rustls-native-roots", feature = "rustls-webpki-roots"))]
impl Debug for TlsContainer {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("TlsContainer").finish()
    }
}

impl TlsContainer {
    #[cfg(all(
        feature = "native",
        not(any(feature = "rustls-native-roots", feature = "rustls-webpki-roots"))
    ))]
    pub fn new() -> Result<Self, TlsError> {
        let native_connector = TlsConnector::new().map_err(|err| TlsError {
            kind: TlsErrorType::NativeTls,
            source: Some(Box::new(err)),
        })?;

        Ok(TlsContainer {
            tls: native_connector,
        })
    }

    #[cfg(any(feature = "rustls-native-roots", feature = "rustls-webpki-roots"))]
    pub fn new() -> Result<Self, TlsError> {
        let mut roots = rustls_tls::RootCertStore::empty();

        #[cfg(feature = "rustls-native-roots")]
        {
            let certs = rustls_native_certs::load_native_certs().map_err(|err| TlsError {
                kind: TlsErrorType::NativeCerts,
                source: Some(Box::new(err)),
            })?;

            for cert in certs {
                roots
                    .add(&rustls_tls::Certificate(cert.0))
                    .map_err(|err| TlsError {
                        kind: TlsErrorType::NativeCerts,
                        source: Some(Box::new(err)),
                    })?;
            }
        }

        #[cfg(feature = "rustls-webpki-roots")]
        {
            roots.add_server_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.0.iter().map(|ta| {
                OwnedTrustAnchor::from_subject_spki_name_constraints(
                    ta.subject,
                    ta.spki,
                    ta.name_constraints,
                )
            }));
        };

        let config = ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(roots)
            .with_no_client_auth();

        Ok(TlsContainer {
            tls: Arc::new(config),
        })
    }

    pub fn connector(&self) -> Connector {
        #[cfg(all(
            feature = "native",
            not(any(feature = "rustls-native-roots", feature = "rustls-webpki-roots"))
        ))]
        return Connector::NativeTls(self.tls.clone());

        #[cfg(any(feature = "rustls-native-roots", feature = "rustls-webpki-roots"))]
        return Connector::Rustls(Arc::clone(&self.tls));
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::TlsContainer;
    static_assertions::assert_impl_all!(TlsContainer: Debug, Clone, Send, Sync);
}
