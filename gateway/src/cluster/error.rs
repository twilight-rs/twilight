//! The error type of why errors occur in the cluster module.

use crate::shard::Error as ShardError;
use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
    result::Result as StdResult,
};
use twilight_http::Error as HttpError;

/// A result enum with the error type being the cluster's [`Error`] type.
///
/// [`Error`]: enum.Error.html
pub type Result<T, E = Error> = StdResult<T, E>;

/// Error type representing the possible reasons for errors to occur in the
/// cluster.
#[allow(clippy::pub_enum_variant_names)]
#[derive(Debug)]
pub enum Error {
    /// An error occurred while getting the gateway information with the number
    /// of shards to use.
    GettingGatewayInfo {
        /// The reason for the error.
        source: HttpError,
    },
    /// The start of the shard range was greater than the end or total.
    IdTooLarge {
        /// The last shard in the range to manage.
        end: u64,
        /// The first shard in the range to manage.
        start: u64,
        /// The total number of shards used by the bot.
        total: u64,
    },
    /// The "large threshold" value was too large or too small.
    LargeThresholdInvalid {
        /// The error from the shard's [`ShardConfigBuilder`].
        ///
        /// [`ShardConfigBuilder`]: ../../shard/config/struct.ShardConfigBuilder.html
        source: ShardError,
    },
    /// The shard you are trying to get does not exist.
    ShardDoesNotExist {
        /// The attempted ID.
        id: u64,
    },
    ShardError {
        /// The error from the shard's [`ShardConfigBuilder`].
        ///
        /// [`ShardConfigBuilder`]: ../../shard/config/struct.ShardConfigBuilder.html
        source: ShardError,
    },
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::GettingGatewayInfo {
                ..
            } => f.write_str("Getting the gateway info failed"),
            Self::IdTooLarge {
                end,
                start,
                total,
            } => write!(
                f,
                "The shard ID pair {}-{}/{} is mismatched",
                start, end, total
            ),
            Self::LargeThresholdInvalid {
                source,
            } | Self::ShardError {
                source,
            } => write!(f, "{}", source),
            Self::ShardDoesNotExist {
                id,
            } => write!(f, "The ID: {} does not exist", id),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::GettingGatewayInfo {
                source,
            } => Some(source),
            Self::IdTooLarge {
                ..
            } | Self::ShardDoesNotExist {
                ..
            } => None,
            Self::LargeThresholdInvalid {
                source,
            } | Self::ShardError {
                source,
            } => Some(source),
        }
    }
}
