//! Utilities for creating Websocket connections.

use crate::{
    compression::COMPRESSION_FEATURES, error::ReceiveMessageError, tls::TlsContainer, ShardId,
    API_VERSION,
};
use std::fmt::{Display, Formatter, Result as FmtResult};
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::protocol::WebSocketConfig, MaybeTlsStream, WebSocketStream};

/// URL of the Discord gateway.
const GATEWAY_URL: &str = "wss://gateway.discord.gg";

/// Configuration used for Websocket connections.
///
/// `max_frame_size` and `max_message_queue` limits are disabled because
/// Discord is not a malicious actor and having a limit has caused problems on
/// large [`GuildCreate`] payloads.
///
/// `accept_unmasked_frames` and `max_send_queue` are set to their
/// defaults.
///
/// [`GuildCreate`]: twilight_model::gateway::payload::incoming::GuildCreate
const WEBSOCKET_CONFIG: WebSocketConfig = WebSocketConfig {
    accept_unmasked_frames: false,
    max_frame_size: None,
    max_message_size: None,
    max_send_queue: None,
};

/// [`tokio_tungstenite`] library Websocket connection.
///
/// Connections are used by [`Shard`]s when [initially connecting] and when
/// reconnecting.
///
/// [initially connecting]: crate::Shard::with_config
/// [`Shard`]: crate::Shard
pub type Connection = WebSocketStream<MaybeTlsStream<TcpStream>>;

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
pub async fn connect(
    shard_id: ShardId,
    maybe_gateway_url: Option<&str>,
    tls: &TlsContainer,
) -> Result<Connection, ReceiveMessageError> {
    let url = ConnectionUrl::new(maybe_gateway_url).to_string();

    tracing::debug!(%shard_id, ?url, "shaking hands with remote");
    let stream = tls.connect(&url, WEBSOCKET_CONFIG).await?;
    tracing::debug!(%shard_id, "shook hands with remote");

    Ok(stream)
}

#[cfg(test)]
mod tests {
    use super::GATEWAY_URL;
    use crate::{compression::COMPRESSION_FEATURES, connection::ConnectionUrl, API_VERSION};

    /// Test that [`ConnectionUrl`] formats the default URL as expected.
    #[test]
    fn default_url() {
        let url = ConnectionUrl::new(None).to_string();
        assert_eq!(
            url,
            format!(
                "{}/?v={}&encoding=json{}",
                GATEWAY_URL, API_VERSION, COMPRESSION_FEATURES
            ),
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
