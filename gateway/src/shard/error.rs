//! The error type of why errors occur in the shard module.

use futures_channel::mpsc::TrySendError;
#[cfg(not(feature = "simd-json"))]
use serde_json::Error as JsonError;
#[cfg(feature = "simd-json")]
use simd_json::Error as JsonError;

use async_tungstenite::tungstenite::{Error as TungsteniteError, Message as TungsteniteMessage};
use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
    result::Result as StdResult,
};
use twilight_http::Error as HttpError;

use super::processor::Error as ProcessorError;

/// A result enum with the error type being the shard's [`Error`] type.
///
/// [`Error`]: enum.Error.html
pub type Result<T, E = Error> = StdResult<T, E>;

/// Error type representing the possible reasons for errors to occur in the
/// shard.
#[derive(Debug)]
pub enum Error {
    /// An error happened while creating a shard processor.
    Processor {
        /// The error from the shard processor.
        source: ProcessorError,
    },
    /// An error happened while trying to connect to the gateway.
    Connecting {
        /// The error from the WebSocket client.
        source: TungsteniteError,
    },
    /// Getting the gateway URL via the HTTP client failed.
    GettingGatewayUrl {
        /// The error from the `twilight_http` client.
        source: HttpError,
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
    /// The shard hasn't been started, so there is no active session.
    Stopped,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Processor { .. } => f.write_str("An issue occured creating a shard processor"),
            Self::Connecting { .. } => f.write_str("An issue occurred connecting to the gateway"),
            Self::GettingGatewayUrl { .. } => f.write_str("Getting the gateway URL failed"),
            Self::IdTooLarge { id, total } => {
                write!(f, "The shard ID {} is larger than the total, {}", id, total)
            }
            Self::LargeThresholdInvalid { value } => write!(
                f,
                "The large threshold given, {}, is not in the accepted range",
                value
            ),
            Self::PayloadSerialization { .. } => {
                f.write_str("Deserializing or serializing a payload failed")
            }
            Self::SendingMessage { .. } => {
                f.write_str("The message couldn't be sent because the receiver half dropped")
            }
            Self::Stopped { .. } => f.write_str("the shard hasn't been started yet"),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Processor { source } => Some(source),
            Self::Connecting { source } => Some(source),
            Self::GettingGatewayUrl { source } => Some(source),
            Self::PayloadSerialization { source } => Some(source),
            Self::SendingMessage { source } => Some(source),
            Self::IdTooLarge { .. } | Self::LargeThresholdInvalid { .. } | Self::Stopped { .. } => {
                None
            }
        }
    }
}
