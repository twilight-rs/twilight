//! The error type of why errors occur in the shard module.

use flate2::DecompressError;
use futures_channel::mpsc::TrySendError;
use serde_json::Error as JsonError;
use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
    result::Result as StdResult,
};
use tokio_tungstenite::tungstenite::{Error as TungsteniteError, Message as TungsteniteMessage};
use url::ParseError;

/// A result enum with the error type being the shard's [`Error`] type.
///
/// [`Error`]: enum.Error.html
pub type Result<T, E = Error> = StdResult<T, E>;

/// Error type representing the possible reasons for errors to occur in the
/// shard.
#[derive(Debug)]
pub enum Error {
    /// An error happened while trying to connect to the gateway.
    Connecting {
        /// The error from the WebSocket client.
        source: TungsteniteError,
    },
    /// The shard ID was larger than the total number of shards.
    IdTooLarge {
        /// The ID of the shard that was too large.
        id: u64,
        /// The total number of shards in use.
        total: u64,
    },
    /// The "large threshold" value was too large or too small.
    LargeThresholdInvalid {
        /// The value that is invalid.
        value: u64,
    },
    /// Parsing the URL to connect to the gateway failed due to an invalid URL.
    ParsingUrl {
        /// The reason for the parse failing.
        source: ParseError,
        /// The URL that couldn't be parsed.
        url: String,
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

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Connecting {
                ..
            } => f.write_str("An issue occurred connecting to the gateway"),
            Self::IdTooLarge {
                id,
                total,
            } => write!(f, "The shard ID {} is larger than the total, {}", id, total),
            Self::LargeThresholdInvalid {
                value,
            } => write!(
                f,
                "The large threshold given, {}, is not in the accepted range",
                value
            ),
            Self::ParsingUrl {
                url, ..
            } => write!(f, "The gateway URL {:?} is invalid", url),
            Self::PayloadSerialization {
                ..
            } => f.write_str("Deserializing or serializing a payload failed"),
            Self::SendingMessage {
                ..
            } => f.write_str("The message couldn't be sent because the receiver half dropped"),
            Self::Decompressing {
                ..
            } => f.write_str("A frame could not be decompressed"),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Connecting {
                source,
            } => Some(source),
            Self::ParsingUrl {
                source, ..
            } => Some(source),
            Self::PayloadSerialization {
                source,
            } => Some(source),
            Self::SendingMessage {
                source,
            } => Some(source),
            Self::Decompressing {
                source,
            } => Some(source),
            Self::IdTooLarge {
                ..
            }
            | Self::LargeThresholdInvalid {
                ..
            } => None,
        }
    }
}
