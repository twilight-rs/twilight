//! TLS manager to reuse connections between shards.

#[cfg(not(any(
    feature = "native",
    feature = "rustls-native-roots",
    feature = "rustls-webpki-roots"
)))]
mod r#impl {
    //! Plain connections with no TLS.

    /// No connector is used when plain connections are enabled.
    pub type TlsConnector = ();

    use super::{TlsContainer, TlsError};
    use crate::{
        connection::Connection,
        error::{ShardInitializeError, ShardInitializeErrorType},
    };
    use tokio_tungstenite::{tungstenite::protocol::WebSocketConfig, Connector};

    /// Create a TLS container without a TLS connector.
    ///
    /// # Errors
    ///
    /// Never returns an error, and only returns a Result to reach parity when
    /// TLS features are enabled.
    pub fn new() -> Result<TlsContainer, TlsError> {
        Ok(TlsContainer { tls: None })
    }

    /// Connect to the provided URL without TLS.
    pub async fn connect(
        url: &str,
        maybe_config: Option<WebSocketConfig>,
        _tls: &TlsContainer,
    ) -> Result<Connection, ShardInitializeError> {
        let (stream, _) = tokio_tungstenite::connect_async_with_config(url, maybe_config)
            .await
            .map_err(|source| ShardInitializeError {
                kind: ShardInitializeErrorType::Establishing,
                source: Some(Box::new(source)),
            })?;

        Ok(stream)
    }

    /// No TLS connector.
    pub fn connector(_: &TlsContainer) -> Option<Connector> {
        None
    }
}

#[cfg(all(
    feature = "native",
    not(any(feature = "rustls-native-roots", feature = "rustls-webpki-roots"))
))]
mod r#impl {
    //! Native TLS

    pub use native_tls::TlsConnector;

    use super::{TlsContainer, TlsError, TlsErrorType};
    use crate::{
        connection::Connection,
        error::{ShardInitializeError, ShardInitializeErrorType},
    };
    use tokio_tungstenite::{tungstenite::protocol::WebSocketConfig, Connector};

    /// Create a new TLS connector.
    ///
    /// # Errors
    ///
    /// Returns a [`TlsErrorType::Loading`] error type if the TLS connector
    /// couldn't be initialized.
    pub fn new() -> Result<TlsContainer, TlsError> {
        let native_connector = TlsConnector::new().map_err(|err| TlsError {
            kind: TlsErrorType::Loading,
            source: Some(Box::new(err)),
        })?;

        Ok(TlsContainer {
            tls: Some(native_connector),
        })
    }

    /// Connect to the provided URL with the underlying TLS connector.
    pub async fn connect(
        url: &str,
        maybe_config: Option<WebSocketConfig>,
        tls: &TlsContainer,
    ) -> Result<Connection, ShardInitializeError> {
        let (stream, _) =
            tokio_tungstenite::connect_async_tls_with_config(url, maybe_config, tls.connector())
                .await
                .map_err(|source| ShardInitializeError {
                    kind: ShardInitializeErrorType::Establishing,
                    source: Some(Box::new(source)),
                })?;

        Ok(stream)
    }

    /// Clone the underlying TLS connector for native TLS.
    pub fn connector(container: &TlsContainer) -> Option<Connector> {
        container
            .tls
            .as_ref()
            .map(|tls| Connector::NativeTls(tls.clone()))
    }
}

#[cfg(any(feature = "rustls-native-roots", feature = "rustls-webpki-roots"))]
mod r#impl {
    //! Rustls

    use super::{TlsContainer, TlsError};
    use crate::{
        connection::Connection,
        error::{ShardInitializeError, ShardInitializeErrorType},
    };
    use rustls_tls::ClientConfig;
    use std::sync::Arc;
    use tokio_tungstenite::{tungstenite::protocol::WebSocketConfig, Connector};

    /// Rustls client configuration.
    pub type TlsConnector = Arc<ClientConfig>;

    /// Create a new TLS connector.
    ///
    /// # Errors
    ///
    /// Returns a `TlsErrorType::Loading` error type if the TLS connector
    /// couldn't be initialized.
    #[cfg(any(feature = "rustls-native-roots", feature = "rustls-webpki-roots"))]
    pub fn new() -> Result<TlsContainer, TlsError> {
        let mut roots = rustls_tls::RootCertStore::empty();

        #[cfg(feature = "rustls-native-roots")]
        {
            let certs = rustls_native_certs::load_native_certs().map_err(|err| TlsError {
                kind: super::TlsErrorType::Loading,
                source: Some(Box::new(err)),
            })?;

            for cert in certs {
                roots
                    .add(&rustls_tls::Certificate(cert.0))
                    .map_err(|err| TlsError {
                        kind: super::TlsErrorType::Loading,
                        source: Some(Box::new(err)),
                    })?;
            }
        }

        #[cfg(feature = "rustls-webpki-roots")]
        {
            roots.add_server_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.0.iter().map(|ta| {
                rustls_tls::OwnedTrustAnchor::from_subject_spki_name_constraints(
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

    /// Connect to the provided URL with the underlying TLS connector.
    pub async fn connect(
        url: &str,
        maybe_config: Option<WebSocketConfig>,
        tls: &TlsContainer,
    ) -> Result<Connection, ShardInitializeError> {
        let (stream, _) =
            tokio_tungstenite::connect_async_tls_with_config(url, maybe_config, tls.connector())
                .await
                .map_err(|source| ShardInitializeError {
                    kind: ShardInitializeErrorType::Establishing,
                    source: Some(Box::new(source)),
                })?;

        Ok(stream)
    }

    /// Clone the underlying TLS connector for rustls.
    pub fn connector(container: &TlsContainer) -> Option<Connector> {
        container
            .tls
            .as_ref()
            .map(|tls| Connector::Rustls(Arc::clone(tls)))
    }
}

use r#impl::TlsConnector;
use std::{
    error::Error,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
};
use tokio_tungstenite::{tungstenite::protocol::WebSocketConfig, Connector};

use crate::{connection::Connection, error::ShardInitializeError};

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
    #[allow(unused)]
    Loading,
}

/// Wrapper over a native or Rustls TLS connector.
#[derive(Clone)]
pub struct TlsContainer {
    /// TLS connector, which won't be present if no TLS feature is enabled.
    #[allow(unused)]
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
    /// For non-plain TLS, returns a [`TlsErrorType::Loading`] error type if
    /// the TLS connector couldn't be initialized.
    pub fn new() -> Result<Self, TlsError> {
        r#impl::new()
    }

    /// Connect to the provided URL with the underlying TLS connector.
    pub async fn connect(
        &self,
        url: &str,
        config: Option<WebSocketConfig>,
    ) -> Result<Connection, ShardInitializeError> {
        r#impl::connect(url, config, self).await
    }

    /// Clone of a reference to the connector.
    #[allow(unused)]
    pub(crate) fn connector(&self) -> Option<Connector> {
        r#impl::connector(self)
    }
}

#[cfg(test)]
mod tests {
    use super::TlsContainer;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(TlsContainer: Debug, Clone, Send, Sync);
}
