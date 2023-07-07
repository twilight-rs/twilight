//! Utilities for creating Websocket connections.

use crate::{
    error::{ReceiveMessageError, ReceiveMessageErrorType},
    API_VERSION,
};
use std::fmt::{Display, Formatter, Result as FmtResult};
use tokio::net::TcpStream;
use tokio_websockets::{ClientBuilder, Connector, Limits, MaybeTlsStream, WebsocketStream};

/// Query argument with zlib-stream enabled.
#[cfg(any(feature = "zlib-stock", feature = "zlib-simd"))]
const COMPRESSION_FEATURES: &str = "&compress=zlib-stream";

/// No query arguments due to compression being disabled.
#[cfg(not(any(feature = "zlib-stock", feature = "zlib-simd")))]
const COMPRESSION_FEATURES: &str = "";

/// URL of the Discord gateway.
const GATEWAY_URL: &str = "wss://gateway.discord.gg";

/// [`tokio_websockets`] library Websocket connection.
///
/// Connections are used by [`Shard`]s when reconnecting.
///
/// [`Shard`]: crate::Shard
pub type Connection = WebsocketStream<MaybeTlsStream<TcpStream>>;

/// Formatter for a gateway URL, with the API version and compression features
/// specified.
struct ConnectionUrl<'a> {
    /// Gateway URL configured by the URL via [`ConfigBuilder::gateway_url`].
    ///
    /// [`ConfigBuilder::gateway_url`]: crate::ConfigBuilder::gateway_url
    configured_url: Option<&'a str>,
}

impl<'a> ConnectionUrl<'a> {
    /// Initialize a new gateway URL formatter with the user's configured gateway
    /// URL.
    const fn new(maybe_configured_url: Option<&'a str>) -> Self {
        Self {
            configured_url: maybe_configured_url,
        }
    }
}

impl Display for ConnectionUrl<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let base_url = self.configured_url.unwrap_or(GATEWAY_URL);
        f.write_str(base_url)?;

        // Discord's documentation states:
        //
        // "Generally, it is a good idea to explicitly pass the gateway version
        // and encoding".
        //
        // <https://discord.com/developers/docs/topics/gateway#connecting-gateway-url-query-string-params>
        f.write_str("/?v=")?;
        Display::fmt(&API_VERSION, f)?;

        f.write_str("&encoding=json")?;
        f.write_str(COMPRESSION_FEATURES)
    }
}

/// Connect to the gateway for a given URL, defaulting if not present.
///
/// If a URL isn't provided then [`GATEWAY_URL`] is used. The Shard ID is used
/// only for tracing logs.
///
/// # Errors
///
/// Returns a [`ReceiveMessageErrorType::Reconnect`] error type if the
/// connection with the Discord gateway could not be established, such as
/// due to network or TLS errors.
///
/// [`ReceiveMessageErrorType::Reconnect`]: crate::error::ReceiveMessageErrorType::Reconnect
#[tracing::instrument(skip_all)]
pub async fn connect(
    maybe_gateway_url: Option<&str>,
    tls: &Connector,
) -> Result<Connection, ReceiveMessageError> {
    let url = ConnectionUrl::new(maybe_gateway_url).to_string();

    // Limits to impose on Websocket connections.
    //
    // `max_payload_len` limit is disabled because Discord is not a malicious
    // actor and having a limit has caused problems on large `GuildCreate`
    // payloads.
    let limits = Limits::default().max_payload_len(None);

    tracing::debug!(?url, "shaking hands with gateway");

    let (stream, _) = ClientBuilder::new()
        .uri(&url)
        .expect("Gateway URL must be valid")
        .limits(limits)
        .connector(tls)
        .connect()
        .await
        .map_err(|source| ReceiveMessageError {
            kind: ReceiveMessageErrorType::Reconnect,
            source: Some(Box::new(source)),
        })?;

    Ok(stream)
}

#[cfg(test)]
mod tests {
    use super::{ConnectionUrl, COMPRESSION_FEATURES, GATEWAY_URL};
    use crate::API_VERSION;

    /// Test that [`ConnectionUrl`] formats the default URL as expected.
    #[test]
    fn default_url() {
        let url = ConnectionUrl::new(None).to_string();
        assert_eq!(
            url,
            format!("{GATEWAY_URL}/?v={API_VERSION}&encoding=json{COMPRESSION_FEATURES}"),
        );
    }

    /// Test that [`ConnectionUrl`] formats a provided URL as expected.
    #[test]
    fn user_provided_url() {
        /// URL provided by the user in the shard's configuration.
        const USER_URL: &str = "ws://localhost:1312";

        let valid_url = ConnectionUrl::new(Some(USER_URL));
        assert_eq!(
            valid_url.to_string(),
            format!("{USER_URL}/?v={API_VERSION}&encoding=json{COMPRESSION_FEATURES}"),
        );
    }
}
