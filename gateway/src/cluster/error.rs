//! The error type of why errors occur in the cluster module.

use crate::shard::Error as ShardError;
use dawn_http::Error as HttpError;
use snafu::Snafu;
use std::result::Result as StdResult;

/// A result enum with the error type being the cluster's [`Error`] type.
///
/// [`Error`]: enum.Error.html
pub type Result<T, E = Error> = StdResult<T, E>;

/// Error type representing the possible reasons for errors to occur in the
/// cluster.
#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
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
        /// The error from the shard's [`ConfigBuilder`].
        ///
        /// [`ConfigBuilder`]: ../../shard/config/struct.ConfigBuilder.html
        source: ShardError,
    },
}
