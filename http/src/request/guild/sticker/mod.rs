mod create_guild_sticker;
mod delete_guild_sticker;
mod get_guild_sticker;
mod get_guild_stickers;
mod update_guild_sticker;

pub use self::{
    create_guild_sticker::CreateGuildSticker, delete_guild_sticker::DeleteGuildSticker,
    get_guild_sticker::GetGuildSticker, get_guild_stickers::GetGuildStickers,
    update_guild_sticker::UpdateGuildSticker,
};

use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

/// Error created if validation of a sticker field fails.
#[derive(Debug)]
pub struct StickerValidationError {
    pub(crate) kind: StickerValidationErrorType,
    pub(crate) source: Option<Box<dyn Error + Send + Sync>>,
}

impl StickerValidationError {
    /// Maximum length of a sticker description.
    pub const DESCRIPTION_MAX_LENGTH: usize = 200;

    /// Minimum length of a sticker description.
    pub const DESCRIPTION_MIN_LENGTH: usize = 2;

    /// Maximum length of a sticker name.
    pub const NAME_MAX_LENGTH: usize = 30;

    /// Minimum length of a sticker name.
    pub const NAME_MIN_LENGTH: usize = 2;

    /// Maximum length of the sticker's tags.
    pub const TAGS_MAX_LENGTH: usize = 200;

    /// Minimum length of the sticker's tags.
    pub const TAGS_MIN_LENGTH: usize = 2;

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
