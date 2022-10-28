//! Constants, error types, and functions for validating channel fields.

use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::channel::ChannelType;

/// Minimum bitrate of a voice channel.
pub const CHANNEL_BITRATE_MIN: u32 = 8000;

/// Maximum length of a forum channel's topic.
pub const CHANNEL_FORUM_TOPIC_LENGTH_MAX: usize = 4096;

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
            ChannelValidationErrorType::BitrateInvalid => {
                f.write_str("bitrate is less than ")?;
                Display::fmt(&CHANNEL_BITRATE_MIN, f)
            }
            ChannelValidationErrorType::ForumTopicInvalid => {
                f.write_str("the forum topic is invalid")
            }
            ChannelValidationErrorType::NameInvalid => {
                f.write_str("the length of the name is invalid")
            }
            ChannelValidationErrorType::RateLimitPerUserInvalid { .. } => {
                f.write_str("the rate limit per user is invalid")
            }
            ChannelValidationErrorType::TopicInvalid => f.write_str("the topic is invalid"),
            ChannelValidationErrorType::TypeInvalid { kind } => {
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
    /// The bitrate is less than 8000.
    BitrateInvalid,
    /// The length of the topic is more than 4096 UTF-16 characters.
    ForumTopicInvalid,
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

/// Ensure a channel's bitrate is collect.
///
/// Must be at least 8000.
///
/// # Errors
///
/// Returns an error of type [`BitrateInvalid`] if the bitrate is invalid.
///
/// [`BitrateInvalid`]: ChannelValidationErrorType::BitrateInvalid
pub const fn bitrate(value: u32) -> Result<(), ChannelValidationError> {
    if value >= CHANNEL_BITRATE_MIN {
        Ok(())
    } else {
        Err(ChannelValidationError {
            kind: ChannelValidationErrorType::BitrateInvalid,
        })
    }
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
        ChannelType::AnnouncementThread | ChannelType::PublicThread | ChannelType::PrivateThread
    ) {
        Ok(())
    } else {
        Err(ChannelValidationError {
            kind: ChannelValidationErrorType::TypeInvalid { kind },
        })
    }
}

/// Ensure a forum channel's topic's length is correct.
///
/// # Errors
///
/// Returns an error of type [`TopicInvalid`] if the
/// topic is invalid.
///
/// [`TopicInvalid`]: ChannelValidationErrorType::TopicInvalid
pub fn forum_topic(value: impl AsRef<str>) -> Result<(), ChannelValidationError> {
    let count = value.as_ref().chars().count();

    if count <= CHANNEL_FORUM_TOPIC_LENGTH_MAX {
        Ok(())
    } else {
        Err(ChannelValidationError {
            kind: ChannelValidationErrorType::TopicInvalid,
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
    fn channel_bitrate() {
        assert!(bitrate(8000).is_ok());

        assert!(bitrate(7000).is_err());
    }

    #[test]
    fn thread_is_thread() {
        assert!(is_thread(ChannelType::AnnouncementThread).is_ok());
        assert!(is_thread(ChannelType::PrivateThread).is_ok());
        assert!(is_thread(ChannelType::PublicThread).is_ok());

        assert!(is_thread(ChannelType::Group).is_err());
    }

    #[test]
    fn channel_name() {
        assert!(name("a").is_ok());
        assert!(name("a".repeat(100)).is_ok());

        assert!(name("").is_err());
        assert!(name("a".repeat(101)).is_err());
    }

    #[test]
    fn rate_limit_per_user_value() {
        assert!(rate_limit_per_user(0).is_ok());
        assert!(rate_limit_per_user(21_600).is_ok());

        assert!(rate_limit_per_user(21_601).is_err());
    }

    #[test]
    fn topic_length() {
        assert!(topic("").is_ok());
        assert!(topic("a").is_ok());
        assert!(topic("a".repeat(1_024)).is_ok());

        assert!(topic("a".repeat(1_025)).is_err());
    }
}
