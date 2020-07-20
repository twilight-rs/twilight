//! The error type of why errors occur in the shard processor module.

use std::fmt;
use std::str::Utf8Error;

use flate2::DecompressError;
use futures_channel::mpsc::TrySendError;
#[cfg(not(feature = "simd-json"))]
use serde_json::Error as JsonError;
#[cfg(feature = "simd-json")]
use simd_json::Error as JsonError;
use url::ParseError;

use async_tungstenite::tungstenite::{Error as TungsteniteError, Message as TungsteniteMessage};
use twilight_model::gateway::GatewayIntents;

/// A result enum with the error type being the shard processor's [`Error`] type.
///
/// [`Error`]: enum.Error.html
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Error type representing the possible reasons for errors to occur in the
/// shard processor.
#[derive(Debug)]
pub enum Error {
    /// The provided authorization token is invalid.
    AuthorizationInvalid { shard_id: u64, token: String },
    /// An error happened while trying to connect to the gateway.
    Connecting {
        /// The error from the WebSocket client.
        source: TungsteniteError,
    },
    /// The current user isn't allowed to use at least one of the configured
    /// intents.
    ///
    /// The intents are provided.
    IntentsDisallowed {
        /// The configured intents for the shard.
        intents: Option<GatewayIntents>,
        /// The ID of the shard.
        shard_id: u64,
    },
    /// The configured intents aren't supported by Discord's gateway.
    ///
    /// The intents are provided.
    IntentsInvalid {
        /// The configured intents for the shard.
        intents: Option<GatewayIntents>,
        /// The ID of the shard.
        shard_id: u64,
    },
    /// Parsing the URL to connect to the gateway failed due to an invalid URL.
    ParsingUrl {
        /// The reason for the parse failing.
        source: ParseError,
        /// The URL that couldn't be parsed.
        url: String,
    },
    /// The payload received from Discord was an invalid structure.
    ///
    /// The payload was either invalid JSON or did not contain the necessary
    /// "op" key in the object.
    PayloadInvalid,
    /// The binary payload received from Discord wasn't validly encoded as
    /// UTF-8.
    PayloadNotUtf8 {
        /// Source error when converting to a UTF-8 valid string.
        source: Utf8Error,
    },
    /// There was an error serializing or deserializing a payload.
    PayloadSerialization {
        /// The serialization error.
        source: JsonError,
    },
    /// A message tried to be sent but the receiving half was dropped. This
    /// typically means that the shard is shutdown.
    SendingMessage {
        /// The reason for the error.
        source: TrySendError<TungsteniteMessage>,
    },
    /// There was a error decompressing a frame from discord.
    Decompressing { source: DecompressError },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AuthorizationInvalid { shard_id, .. } => write!(
                f,
                "The authorization token for shard {} is invalid",
                shard_id
            ),
            Self::Connecting { .. } => f.write_str("An issue occurred connecting to the gateway"),
            Self::IntentsDisallowed { intents, shard_id } => write!(
                f,
                "At least one of the intents ({:?}) for shard {} are disallowed",
                intents, shard_id
            ),
            Self::IntentsInvalid { intents, shard_id } => write!(
                f,
                "At least one of the intents ({:?}) for shard {} are invalid",
                intents, shard_id
            ),
            Self::ParsingUrl { url, .. } => write!(f, "The gateway URL {:?} is invalid", url),
            Self::PayloadInvalid { .. } => write!(
                f,
                "The payload received from Discord contained an invalid data structure"
            ),
            Self::PayloadNotUtf8 { .. } => write!(f, "The payload from Discord wasn't UTF-8 valid"),
            Self::PayloadSerialization { .. } => {
                f.write_str("Deserializing or serializing a payload failed")
            }
            Self::SendingMessage { .. } => {
                f.write_str("The message couldn't be sent because the receiver half dropped")
            }
            Self::Decompressing { .. } => f.write_str("A frame could not be decompressed"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Connecting { source } => Some(source),
            Self::ParsingUrl { source, .. } => Some(source),
            Self::PayloadNotUtf8 { source } => Some(source),
            Self::PayloadSerialization { source } => Some(source),
            Self::SendingMessage { source } => Some(source),
            Self::Decompressing { source } => Some(source),
            Self::AuthorizationInvalid { .. }
            | Self::IntentsDisallowed { .. }
            | Self::IntentsInvalid { .. }
            | Self::PayloadInvalid { .. } => None,
        }
    }
}
