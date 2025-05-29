//! Constants, error types, and functions for validating [`Message`] fields.
//!
//! [`Message`]: twilight_model::channel::Message

use crate::{
    component::{ComponentValidationErrorType, COMPONENT_COUNT},
    embed::{chars as embed_chars, EmbedValidationErrorType, EMBED_TOTAL_LENGTH},
    request::ValidationError,
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::{
    channel::message::{Component, Embed},
    http::attachment::Attachment,
    id::{marker::StickerMarker, Id},
};

/// Maximum length of an attachment's description.
pub const ATTACHMENT_DESCIPTION_LENGTH_MAX: usize = 1024;

/// Maximum number of embeds that a message may have.
pub const EMBED_COUNT_LIMIT: usize = 10;

/// Maximum length of message content.
pub const MESSAGE_CONTENT_LENGTH_MAX: usize = 2000;

/// Maximum amount of stickers.
pub const STICKER_MAX: usize = 3;

/// ASCII dash.
const DASH: char = '-';

/// ASCII dot.
const DOT: char = '.';

/// ASCII underscore.
const UNDERSCORE: char = '_';

/// A message is not valid.
#[derive(Debug)]
pub struct MessageValidationError {
    /// Type of error that occurred.
    kind: MessageValidationErrorType,
    /// Source of the error, if any.
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl MessageValidationError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &MessageValidationErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        self.source
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(
        self,
    ) -> (
        MessageValidationErrorType,
        Option<Box<dyn Error + Send + Sync>>,
    ) {
        (self.kind, self.source)
    }

    /// Create a [`MessageValidationError`] from a [`ValidationError`].
    #[must_use = "has no effect if unused"]
    pub fn from_validation_error(
        kind: MessageValidationErrorType,
        source: ValidationError,
    ) -> Self {
        Self {
            kind,
            source: Some(Box::new(source)),
        }
    }
}

impl Display for MessageValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            MessageValidationErrorType::AttachmentDescriptionTooLarge { chars } => {
                f.write_str("the attachment description is ")?;
                Display::fmt(chars, f)?;
                f.write_str(" characters long, but the max is ")?;

                Display::fmt(&ATTACHMENT_DESCIPTION_LENGTH_MAX, f)
            }
            MessageValidationErrorType::AttachmentFilename { filename } => {
                f.write_str("attachment filename `")?;
                Display::fmt(filename, f)?;

                f.write_str("`is invalid")
            }
            MessageValidationErrorType::ComponentCount { count } => {
                Display::fmt(count, f)?;
                f.write_str(" components were provided, but only ")?;
                Display::fmt(&COMPONENT_COUNT, f)?;

                f.write_str(" root components are allowed")
            }
            MessageValidationErrorType::ComponentInvalid { .. } => {
                f.write_str("a provided component is invalid")
            }
            MessageValidationErrorType::ContentInvalid => f.write_str("message content is invalid"),
            MessageValidationErrorType::EmbedInvalid { idx, .. } => {
                f.write_str("embed at index ")?;
                Display::fmt(idx, f)?;

                f.write_str(" is invalid")
            }
            MessageValidationErrorType::StickersInvalid { len } => {
                f.write_str("amount of stickers provided is ")?;
                Display::fmt(len, f)?;
                f.write_str(" but it must be at most ")?;

                Display::fmt(&STICKER_MAX, f)
            }
            MessageValidationErrorType::TooManyEmbeds => f.write_str("message has too many embeds"),
            MessageValidationErrorType::WebhookUsername => {
                if let Some(source) = self.source() {
                    Display::fmt(&source, f)
                } else {
                    f.write_str("webhook username is invalid")
                }
            }
        }
    }
}

impl Error for MessageValidationError {}

/// Type of [`MessageValidationError`] that occurred.
#[derive(Debug)]
pub enum MessageValidationErrorType {
    /// Attachment filename is not valid.
    AttachmentFilename {
        /// Invalid filename.
        filename: String,
    },
    /// Attachment description is too large.
    AttachmentDescriptionTooLarge {
        /// Provided number of codepoints.
        chars: usize,
    },
    /// Too many message components were provided.
    ComponentCount {
        /// Number of components that were provided.
        count: usize,
    },
    /// An invalid message component was provided.
    ComponentInvalid {
        /// Index of the component.
        idx: usize,
        /// Additional details about the validation failure type.
        kind: ComponentValidationErrorType,
    },
    /// Returned when the content is over 2000 UTF-16 characters.
    ContentInvalid,
    /// Returned when the embed is invalid.
    EmbedInvalid {
        /// Index of the embed.
        idx: usize,
        /// Additional details about the validation failure type.
        kind: EmbedValidationErrorType,
    },
    /// Amount of stickers provided is invalid.
    StickersInvalid {
        /// Invalid length.
        len: usize,
    },
    /// Too many embeds were provided.
    ///
    /// A followup message can have up to 10 embeds.
    TooManyEmbeds,
    /// Provided webhook username was invalid.
    WebhookUsername,
}

/// Ensure an attachment is correct.
///
/// # Errors
///
/// Returns an error of type [`AttachmentDescriptionTooLarge`] if
/// the attachments's description is too large.
///
/// Returns an error of type [`AttachmentFilename`] if the
/// filename is invalid.
///
/// [`AttachmentDescriptionTooLarge`]: MessageValidationErrorType::AttachmentDescriptionTooLarge
/// [`AttachmentFilename`]: MessageValidationErrorType::AttachmentFilename
pub fn attachment(attachment: &Attachment) -> Result<(), MessageValidationError> {
    attachment_filename(&attachment.filename)?;

    if let Some(description) = &attachment.description {
        attachment_description(description)?;
    }

    Ok(())
}

/// Ensure an attachment's description is correct.
///
/// # Errors
///
/// Returns an error of type [`AttachmentDescriptionTooLarge`] if
/// the attachment's description is too large.
///
/// [`AttachmentDescriptionTooLarge`]: MessageValidationErrorType::AttachmentDescriptionTooLarge
pub fn attachment_description(description: impl AsRef<str>) -> Result<(), MessageValidationError> {
    let chars = description.as_ref().chars().count();
    if chars <= ATTACHMENT_DESCIPTION_LENGTH_MAX {
        Ok(())
    } else {
        Err(MessageValidationError {
            kind: MessageValidationErrorType::AttachmentDescriptionTooLarge { chars },
            source: None,
        })
    }
}

/// Ensure an attachment's description is correct.
///
/// The filename can contain ASCII alphanumeric characters, dots, dashes, and
/// underscores.
///
/// # Errors
///
/// Returns an error of type [`AttachmentFilename`] if the filename is invalid.
///
/// [`AttachmentFilename`]: MessageValidationErrorType::AttachmentFilename
pub fn attachment_filename(filename: impl AsRef<str>) -> Result<(), MessageValidationError> {
    if filename
        .as_ref()
        .chars()
        .all(|c| (c.is_ascii_alphanumeric() || c == DOT || c == DASH || c == UNDERSCORE))
    {
        Ok(())
    } else {
        Err(MessageValidationError {
            kind: MessageValidationErrorType::AttachmentFilename {
                filename: filename.as_ref().to_string(),
            },
            source: None,
        })
    }
}

/// Ensure a list of components is correct.
///
/// # Errors
///
/// Returns a [`ComponentValidationErrorType::ComponentCount`] if there are
/// too many components in the provided list.
///
/// Refer to the errors section of [`component`] for a list of errors that may
/// be returned as a result of validating each provided component.
///
/// [`component`]: crate::component::component
pub fn components(components: &[Component]) -> Result<(), MessageValidationError> {
    let count = components.len();

    if count > COMPONENT_COUNT {
        Err(MessageValidationError {
            kind: MessageValidationErrorType::ComponentCount { count },
            source: None,
        })
    } else {
        for (idx, component) in components.iter().enumerate() {
            crate::component::component_v2(component).map_err(|source| {
                let (kind, source) = source.into_parts();

                MessageValidationError {
                    kind: MessageValidationErrorType::ComponentInvalid { idx, kind },
                    source,
                }
            })?;
        }

        Ok(())
    }
}

/// Ensure a message's content is correct.
///
/// # Errors
///
/// Returns an error of type [`ContentInvalid`] if the message's content is
/// invalid.
///
/// [`ContentInvalid`]: MessageValidationErrorType::ContentInvalid
pub fn content(value: impl AsRef<str>) -> Result<(), MessageValidationError> {
    // <https://discordapp.com/developers/docs/resources/channel#create-message-params>
    if value.as_ref().chars().count() <= MESSAGE_CONTENT_LENGTH_MAX {
        Ok(())
    } else {
        Err(MessageValidationError {
            kind: MessageValidationErrorType::ContentInvalid,
            source: None,
        })
    }
}

/// Ensure a list of embeds is correct.
///
/// # Errors
///
/// Returns an error of type [`TooManyEmbeds`] if there are too many embeds.
///
/// Otherwise, refer to the errors section of [`embed`] for a list of errors
/// that may occur.
///
/// [`TooManyEmbeds`]: MessageValidationErrorType::TooManyEmbeds
/// [`embed`]: crate::embed::embed
pub fn embeds(embeds: &[Embed]) -> Result<(), MessageValidationError> {
    if embeds.len() > EMBED_COUNT_LIMIT {
        Err(MessageValidationError {
            kind: MessageValidationErrorType::TooManyEmbeds,
            source: None,
        })
    } else {
        let mut chars = 0;
        for (idx, embed) in embeds.iter().enumerate() {
            chars += embed_chars(embed);

            if chars > EMBED_TOTAL_LENGTH {
                return Err(MessageValidationError {
                    kind: MessageValidationErrorType::EmbedInvalid {
                        idx,
                        kind: EmbedValidationErrorType::EmbedTooLarge { chars },
                    },
                    source: None,
                });
            }

            crate::embed::embed(embed).map_err(|source| {
                let (kind, source) = source.into_parts();

                MessageValidationError {
                    kind: MessageValidationErrorType::EmbedInvalid { idx, kind },
                    source,
                }
            })?;
        }

        Ok(())
    }
}

/// Ensure that the amount of stickers in a message is correct.
///
/// There must be at most [`STICKER_MAX`] stickers. This is based on [this
/// documentation entry].
///
/// # Errors
///
/// Returns an error of type [`StickersInvalid`] if the length is invalid.
///
/// [`StickersInvalid`]: MessageValidationErrorType::StickersInvalid
/// [this documentation entry]: https://discord.com/developers/docs/resources/channel#create-message-jsonform-params
pub fn sticker_ids(sticker_ids: &[Id<StickerMarker>]) -> Result<(), MessageValidationError> {
    let len = sticker_ids.len();

    if len <= STICKER_MAX {
        Ok(())
    } else {
        Err(MessageValidationError {
            kind: MessageValidationErrorType::StickersInvalid { len },
            source: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attachment_description_limit() {
        assert!(attachment_description("").is_ok());
        assert!(attachment_description(str::repeat("a", 1024)).is_ok());

        assert!(matches!(
            attachment_description(str::repeat("a", 1025))
                .unwrap_err()
                .kind(),
            MessageValidationErrorType::AttachmentDescriptionTooLarge { chars: 1025 }
        ));
    }

    #[test]
    fn attachment_allowed_filename() {
        assert!(attachment_filename("one.jpg").is_ok());
        assert!(attachment_filename("two.png").is_ok());
        assert!(attachment_filename("three.gif").is_ok());
        assert!(attachment_filename(".dots-dashes_underscores.gif").is_ok());

        assert!(attachment_filename("????????").is_err());
    }

    #[test]
    fn content_length() {
        assert!(content("").is_ok());
        assert!(content("a".repeat(2000)).is_ok());

        assert!(content("a".repeat(2001)).is_err());
    }
}
