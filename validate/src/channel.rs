//! Constants, error types, and functions for validating channel fields.

use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::channel::ChannelType;

/// Maximum length of a channel's name.
pub const CHANNEL_NAME_LENGTH_MAX: usize = 100;

/// Minimum length of a channel's name.
pub const CHANNEL_NAME_LENGTH_MIN: usize = 1;

/// Maximum length of a channel's rate limit per user.
pub const CHANNEL_RATE_LIMIT_PER_USER_MAX: u16 = 21_600;

/// Maximum length of a channel's topic.
pub const CHANNEL_TOPIC_LENGTH_MAX: usize = 1024;

/// Returned when the channel can not be updated as configured.
#[derive(Debug)]
pub struct ChannelValidationError {
    /// Type of error that occurred.
    kind: ChannelValidationErrorType,
}

impl ChannelValidationError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &ChannelValidationErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[allow(clippy::unused_self)]
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        None
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(
        self,
    ) -> (
        ChannelValidationErrorType,
        Option<Box<dyn Error + Send + Sync>>,
    ) {
        (self.kind, None)
    }
}

impl Display for ChannelValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            ChannelValidationErrorType::NameInvalid => {
                f.write_str("the length of the name is invalid")
            }
            ChannelValidationErrorType::RateLimitPerUserInvalid { .. } => {
                f.write_str("the rate limit per user is invalid")
            }
            ChannelValidationErrorType::TopicInvalid => f.write_str("the topic is invalid"),
            &ChannelValidationErrorType::TypeInvalid { kind } => {
                Display::fmt(kind.name(), f)?;

                f.write_str(" is not a thread")
            }
        }
    }
}

impl Error for ChannelValidationError {}

/// Type of [`ChannelValidationError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum ChannelValidationErrorType {
    /// The length of the name is either fewer than 1 UTF-16 characters or
    /// more than 100 UTF-16 characters.
    NameInvalid,
    /// The seconds of the rate limit per user is more than 21600.
    RateLimitPerUserInvalid {
        /// Provided ratelimit is invalid.
        rate_limit_per_user: u16,
    },
    /// The length of the topic is more than 1024 UTF-16 characters.
    TopicInvalid,
    /// Provided type was not a thread.
    TypeInvalid {
        /// Provided type.
        kind: ChannelType,
    },
}

/// Ensure a channel is a thread.
///
/// # Errors
///
/// Returns an error of type [`ChannelValidationErrorType::TypeInvalid`] if the
/// channel is not a thread.
pub const fn is_thread(kind: ChannelType) -> Result<(), ChannelValidationError> {
    if matches!(
        kind,
        ChannelType::GuildNewsThread
            | ChannelType::GuildPublicThread
            | ChannelType::GuildPrivateThread
    ) {
        Ok(())
    } else {
        Err(ChannelValidationError {
            kind: ChannelValidationErrorType::TypeInvalid { kind },
        })
    }
}

/// Ensure a channel's name's length is correct.
///
/// The length must be less than [`CHANNEL_NAME_LENGTH_MIN`] and at most
/// [`CHANNEL_NAME_LENGTH_MAX`]. This is based on [this documentation entry].
///
/// # Errors
///
/// Returns an error of type [`NameInvalid`] if the channel's name's length is
/// incorrect.
///
/// [`NameInvalid`]: ChannelValidationErrorType::NameInvalid
/// [this documentation entry]: https://discord.com/developers/docs/resources/channel#channels-resource
pub fn name(value: impl AsRef<str>) -> Result<(), ChannelValidationError> {
    let len = value.as_ref().chars().count();

    if (CHANNEL_NAME_LENGTH_MIN..=CHANNEL_NAME_LENGTH_MAX).contains(&len) {
        Ok(())
    } else {
        Err(ChannelValidationError {
            kind: ChannelValidationErrorType::NameInvalid,
        })
    }
}

/// Ensure a channel's rate limit per user is correct.
///
/// The value must be at most [`CHANNEL_RATE_LIMIT_PER_USER_MAX`]. This is based
/// on [this documentation entry].
///
/// # Errors
///
/// Returns an error of type [`RateLimitPerUserInvalid`] if the rate limit is
/// invalid.
///
/// [`RateLimitPerUserInvalid`]: ChannelValidationErrorType::RateLimitPerUserInvalid
/// [this documentation entry]: https://discord.com/developers/docs/resources/channel#channels-resource
pub const fn rate_limit_per_user(value: u16) -> Result<(), ChannelValidationError> {
    if value <= CHANNEL_RATE_LIMIT_PER_USER_MAX {
        Ok(())
    } else {
        Err(ChannelValidationError {
            kind: ChannelValidationErrorType::RateLimitPerUserInvalid {
                rate_limit_per_user: value,
            },
        })
    }
}

/// Ensure a channel's topic's length is correct.
///
/// # Errors
///
/// Returns an error of type [`TopicInvalid`] if the
/// topic is invalid.
///
/// [`TopicInvalid`]: ChannelValidationErrorType::TopicInvalid
pub fn topic(value: impl AsRef<str>) -> Result<(), ChannelValidationError> {
    let count = value.as_ref().chars().count();

    if count <= CHANNEL_TOPIC_LENGTH_MAX {
        Ok(())
    } else {
        Err(ChannelValidationError {
            kind: ChannelValidationErrorType::TopicInvalid,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_thread() {
        assert!(is_thread(ChannelType::GuildNewsThread).is_ok());
        assert!(is_thread(ChannelType::GuildPrivateThread).is_ok());
        assert!(is_thread(ChannelType::GuildPublicThread).is_ok());

        assert!(is_thread(ChannelType::Group).is_err());
    }

    #[test]
    fn test_channel_name() {
        assert!(name("a").is_ok());
        assert!(name("a".repeat(100)).is_ok());

        assert!(name("").is_err());
        assert!(name("a".repeat(101)).is_err());
    }

    #[test]
    fn test_rate_limit_per_user() {
        assert!(rate_limit_per_user(0).is_ok());
        assert!(rate_limit_per_user(21_600).is_ok());

        assert!(rate_limit_per_user(21_601).is_err());
    }

    #[test]
    fn test_topic() {
        assert!(topic("").is_ok());
        assert!(topic("a").is_ok());
        assert!(topic("a".repeat(1_024)).is_ok());

        assert!(topic("a".repeat(1_025)).is_err());
    }
}
