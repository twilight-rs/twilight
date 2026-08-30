//! Constants, error types, and functions for validating soundboard requests.

use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

/// Maximum length of a soundboard sound's name.
pub const SOUNDBOARD_SOUND_NAME_LENGTH_MAX: usize = 100;

/// Minimum length of a soundboard sound's name.
pub const SOUNDBOARD_SOUND_NAME_LENGTH_MIN: usize = 1;

/// Maximum volume of a soundboard sound.
pub const SOUNDBOARD_SOUND_VOLUME_MAX: f64 = 1.0;

/// Minimum volume of a soundboard sound.
pub const SOUNDBOARD_SOUND_VOLUME_MIN: f64 = 0.0;

/// Returned when the channel can not be updated as configured.
#[derive(Debug)]
pub struct SoundboardValidationError {
    /// Type of error that occurred.
    kind: SoundboardValidationErrorType,
}

impl SoundboardValidationError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &SoundboardValidationErrorType {
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
        SoundboardValidationErrorType,
        Option<Box<dyn Error + Send + Sync>>,
    ) {
        (self.kind, None)
    }
}

impl Display for SoundboardValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            SoundboardValidationErrorType::NameInvalid => {
                f.write_str("the length of the name is invalid")
            }
            SoundboardValidationErrorType::VolumeInvalid => f.write_str("the volume is invalid"),
        }
    }
}

impl Error for SoundboardValidationError {}

/// Type of [`SoundboardValidationError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum SoundboardValidationErrorType {
    /// The length of the name is either fewer than 2 characters or more than 32
    /// characters.
    NameInvalid,
    /// The volume can be between 0 and 1 inclusive.
    VolumeInvalid,
}

/// The name of a soundboard sound can have a length between 2 and 32 characters.
///
/// # Errors
/// Returns a [`SoundboardValidationError`] with the type
/// [`SoundboardValidationErrorType::NameInvalid`] if the name fails
/// to validate.
pub fn name(value: impl AsRef<str>) -> Result<(), SoundboardValidationError> {
    let len = value.as_ref().chars().count();

    if (SOUNDBOARD_SOUND_NAME_LENGTH_MIN..=SOUNDBOARD_SOUND_NAME_LENGTH_MAX).contains(&len) {
        Ok(())
    } else {
        Err(SoundboardValidationError {
            kind: SoundboardValidationErrorType::NameInvalid,
        })
    }
}

/// The volume of a soundboard sound must be between 0 and 1 inclusive.
///
/// # Errors
/// Returns a [`SoundboardValidationError`] with the type
/// [`SoundboardValidationErrorType::VolumeInvalid`] if the volume fails
/// to validate.
pub fn volume(value: f64) -> Result<(), SoundboardValidationError> {
    if (SOUNDBOARD_SOUND_VOLUME_MIN..=SOUNDBOARD_SOUND_VOLUME_MAX).contains(&value) {
        Ok(())
    } else {
        Err(SoundboardValidationError {
            kind: SoundboardValidationErrorType::VolumeInvalid,
        })
    }
}
