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

/// Native TLS connector for use with [`TlsContainer`].
#[cfg(all(
    feature = "native",
    not(any(feature = "rustls-native-roots", feature = "rustls-webpki-roots"))
))]
pub(crate) type TlsConnector = NativeTlsConnector;

/// Rustls client configuration for use with [`TlsContainer`].
#[cfg(any(feature = "rustls-native-roots", feature = "rustls-webpki-roots"))]
pub(crate) type TlsConnector = Arc<ClientConfig>;

/// Creating a TLS connector failed, possibly due to loading certificates.
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
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.kind {
            TlsErrorType::Loading => {
                f.write_str("failed to load the tls connector or its certificates")
            }
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
    /// Loading the TLS connector or its certificates failed.
    Loading,
}

/// Wrapper over a native or Rustls TLS connector.
#[derive(Clone)]
pub struct TlsContainer {
    /// TLS connector, which won't be present if no TLS feature is enabled.
    tls: Option<TlsConnector>,
}

impl Debug for TlsContainer {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut debugger = f.debug_struct("TlsContainer");

        #[cfg(all(
            feature = "native",
            not(any(feature = "rustls-native-roots", feature = "rustls-webpki-roots")),
        ))]
        debugger.field("tls", &self.tls);

        debugger.finish()
    }
}

impl TlsContainer {
    /// Create a new TLS connector.
    ///
    /// # Errors
    ///
    /// Never returns an error, and only returns a Result to reach parity when
    /// TLS features are enabled.
    #[cfg(not(any(
        feature = "native",
        feature = "rustls-native-roots",
        feature = "rustls-webpki-roots"
    )))]
    pub fn new() -> Result<Self, TlsError> {
        Ok(Self { tls: None })
    }

    /// Create a new TLS connector.
    ///
    /// # Errors
    ///
    /// Returns a [`TlsErrorType::Loading`] error type if the TLS connector
    /// couldn't be initialized.
    #[cfg(all(
        feature = "native",
        not(any(feature = "rustls-native-roots", feature = "rustls-webpki-roots"))
    ))]
    pub fn new() -> Result<Self, TlsError> {
        let native_connector = TlsConnector::new().map_err(|err| TlsError {
            kind: TlsErrorType::Loading,
            source: Some(Box::new(err)),
        })?;

        Ok(TlsContainer {
            tls: native_connector,
        })
    }

    /// Create a new TLS connector.
    ///
    /// # Errors
    ///
    /// Returns a [`TlsErrorType::Loading`] error type if the TLS connector
    /// couldn't be initialized.
    #[cfg(any(feature = "rustls-native-roots", feature = "rustls-webpki-roots"))]
    pub fn new() -> Result<Self, TlsError> {
        let mut roots = rustls_tls::RootCertStore::empty();

        #[cfg(feature = "rustls-native-roots")]
        {
            let certs = rustls_native_certs::load_native_certs().map_err(|err| TlsError {
                kind: TlsErrorType::Loading,
                source: Some(Box::new(err)),
            })?;

            for cert in certs {
                roots
                    .add(&rustls_tls::Certificate(cert.0))
                    .map_err(|err| TlsError {
                        kind: TlsErrorType::Loading,
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
            tls: Some(Arc::new(config)),
        })
    }

    /// Clone of a reference to the connector.
    pub(crate) fn connector(&self) -> Option<Connector> {
        #[cfg(not(any(
            feature = "native",
            feature = "rustls-native-roots",
            feature = "rustls-webpki-roots"
        )))]
        return None;

        #[cfg(all(
            feature = "native",
            not(any(feature = "rustls-native-roots", feature = "rustls-webpki-roots"))
        ))]
        return self
            .tls
            .as_ref()
            .map(|tls| Connector::NativeTls(Arc::clone(tls)));

        #[cfg(any(feature = "rustls-native-roots", feature = "rustls-webpki-roots"))]
        return self
            .tls
            .as_ref()
            .map(|tls| Connector::Rustls(Arc::clone(tls)));
    }
}

#[cfg(test)]
mod tests {
    use super::TlsContainer;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(TlsContainer: Debug, Clone, Send, Sync);
}
