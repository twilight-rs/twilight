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
pub(crate) type TlsConnector = NativeTlsConnector;

/// todo
#[cfg(any(feature = "rustls-native-roots", feature = "rustls-webpki-roots"))]
pub(crate) type TlsConnector = Arc<ClientConfig>;

/// todo
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

/// todo
#[derive(Clone)]
#[cfg_attr(
    all(
        feature = "native",
        not(any(feature = "rustls-native-roots", feature = "rustls-webpki-roots")),
    ),
    derive(Debug)
)]
pub struct TlsContainer {
    /// todo
    tls: Option<TlsConnector>,
}

#[cfg(any(feature = "rustls-native-roots", feature = "rustls-webpki-roots"))]
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
    #[cfg(not(any(
        feature = "native",
        feature = "rustls-native-roots",
        feature = "rustls-webpki-roots"
    )))]
    pub fn new() -> Result<Self, TlsError> {
        Ok(Self { tls: None })
    }

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

    /// todo
    ///
    /// # Errors
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

    /// todo
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
        return self.tls.as_ref().map(Arc::clone).map(Connector::NativeTls);

        #[cfg(any(feature = "rustls-native-roots", feature = "rustls-webpki-roots"))]
        return self.tls.as_ref().map(Arc::clone).map(Connector::Rustls);
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::TlsContainer;
    static_assertions::assert_impl_all!(TlsContainer: Debug, Clone, Send, Sync);
}
