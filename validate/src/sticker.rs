//! Constants, error types, and functions for validating [`Sticker`] fields.
//!
//! [`Sticker`]: twilight_model::channel::message::Sticker

use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

/// Maximum length of a sticker description.
pub const STICKER_DESCRIPTION_LENGTH_MAX: usize = 200;

/// Minimum length of a sticker description.
pub const STICKER_DESCRIPTION_LENGTH_MIN: usize = 2;

/// Maximum length of a sticker name.
pub const STICKER_NAME_LENGTH_MAX: usize = 30;

/// Minimum length of a sticker name.
pub const STICKER_NAME_LENGTH_MIN: usize = 2;

/// Maximum length of the sticker's tags.
pub const STICKER_TAGS_LENGTH_MAX: usize = 200;

/// Minimum length of the sticker's tags.
pub const STICKER_TAGS_LENGTH_MIN: usize = 2;

/// Error created if validation of a sticker field fails.
#[derive(Debug)]
pub struct StickerValidationError {
    /// Type of error that occurred.
    kind: StickerValidationErrorType,
}

impl StickerValidationError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &StickerValidationErrorType {
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
        StickerValidationErrorType,
        Option<Box<dyn Error + Send + Sync>>,
    ) {
        (self.kind, None)
    }
}

impl Display for StickerValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.kind {
            StickerValidationErrorType::DescriptionInvalid => {
                f.write_str("sticker's description is invalid")
            }
            StickerValidationErrorType::NameInvalid => f.write_str("sticker's name is invalid"),
            StickerValidationErrorType::TagsInvalid => f.write_str("sticker's tags are invalid"),
        }
    }
}

impl Error for StickerValidationError {}

/// Type of [`StickerValidationError`] that occurred.
#[derive(Debug)]
pub enum StickerValidationErrorType {
    /// Sticker's description is invalid.
    DescriptionInvalid,
    /// Sticker's name is invalid.
    NameInvalid,
    /// Sticker's tags are invalid.
    TagsInvalid,
}

/// Ensure that a sticker's description is correct.
///
/// The length must be at least [`STICKER_DESCRIPTION_LENGTH_MIN`] and at most
/// [`STICKER_DESCRIPTION_LENGTH_MAX`]. This is based on
/// [this documentation entry].
///
/// # Errors
///
/// Returns an error of type [`DescriptionInvalid`] if the length is invalid.
///
/// [`DescriptionInvalid`]: StickerValidationErrorType::DescriptionInvalid
/// [this documentation entry]: https://discord.com/developers/docs/resources/sticker#create-guild-sticker
pub fn description(value: impl AsRef<str>) -> Result<(), StickerValidationError> {
    let len = value.as_ref().chars().count();

    if (STICKER_DESCRIPTION_LENGTH_MIN..=STICKER_DESCRIPTION_LENGTH_MAX).contains(&len) {
        Ok(())
    } else {
        Err(StickerValidationError {
            kind: StickerValidationErrorType::DescriptionInvalid,
        })
    }
}

/// Ensure that a sticker's name is correct.
///
/// The length must be at least [`STICKER_NAME_LENGTH_MIN`] and at most
/// [`STICKER_NAME_LENGTH_MAX`]. This is based on [this documentation entry].
///
/// # Errors
///
/// Returns an error of type [`NameInvalid`] if the length is invalid.
///
/// [`NameInvalid`]: StickerValidationErrorType::NameInvalid
/// [this documentation entry]: https://discord.com/developers/docs/resources/sticker#create-guild-sticker
pub fn name(value: impl AsRef<str>) -> Result<(), StickerValidationError> {
    let len = value.as_ref().chars().count();

    if (STICKER_NAME_LENGTH_MIN..=STICKER_NAME_LENGTH_MAX).contains(&len) {
        Ok(())
    } else {
        Err(StickerValidationError {
            kind: StickerValidationErrorType::NameInvalid,
        })
    }
}

/// Ensure that a sticker's tags is correct.
///
/// The length must be at least [`STICKER_TAGS_LENGTH_MIN`] and at most
/// [`STICKER_TAGS_LENGTH_MAX`]. This is based on [this documentation entry].
///
/// # Errors
///
/// Returns an error of type [`TagsInvalid`] if the length is invalid.
///
/// [`TagsInvalid`]: StickerValidationErrorType::TagsInvalid
/// [this documentation entry]: https://discord.com/developers/docs/resources/sticker#create-guild-sticker
pub fn tags(value: impl AsRef<str>) -> Result<(), StickerValidationError> {
    let len = value.as_ref().chars().count();

    if (STICKER_TAGS_LENGTH_MIN..=STICKER_TAGS_LENGTH_MAX).contains(&len) {
        Ok(())
    } else {
        Err(StickerValidationError {
            kind: StickerValidationErrorType::TagsInvalid,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn description() {
        assert!(description("aa").is_ok());
        assert!(description("a".repeat(200)).is_ok());

        assert!(description("a").is_err());
        assert!(description("a".repeat(201)).is_err());
    }

    #[test]
    fn name() {
        assert!(name("aa").is_ok());
        assert!(name("a".repeat(30)).is_ok());

        assert!(name("a").is_err());
        assert!(name("a".repeat(31)).is_err());
    }

    #[test]
    fn tags() {
        assert!(tags("aa").is_ok());
        assert!(tags("a".repeat(200)).is_ok());

        assert!(tags("a").is_err());
        assert!(tags("a".repeat(201)).is_err());
    }
}
