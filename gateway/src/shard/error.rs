//! The error type of why errors occur in the shard module.

use futures_channel::mpsc::TrySendError;
use serde_json::Error as JsonError;
use snafu::Snafu;
use std::result::Result as StdResult;
use tokio_tungstenite::tungstenite::{Error as TungsteniteError, Message as TungsteniteMessage};
use url::ParseError;

/// A result enum with the error type being the shard's [`Error`] type.
///
/// [`Error`]: enum.Error.html
pub type Result<T, E = Error> = StdResult<T, E>;

/// Error type representing the possible reasons for errors to occur in the
/// shard.
#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
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
}
