//! Constants, error types, and functions for validating [`Message`] fields.
//!
//! [`Message`]: twilight_model::channel::Message

use crate::{
    component::{ComponentValidationErrorType, COMPONENT_COUNT},
    embed::EmbedValidationErrorType,
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::{application::component::Component, channel::embed::Embed};

/// Maximum number of embeds that a message may have.
pub const EMBED_COUNT_LIMIT: usize = 10;

/// Maximum length of message content.
pub const MESSAGE_CONTENT_LENGTH_MAX: usize = 2000;

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
}

impl Display for MessageValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
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
            MessageValidationErrorType::TooManyEmbeds { .. } => {
                f.write_str("message has too many embeds")
            }
        }
    }
}

impl Error for MessageValidationError {}

/// Type of [`MessageValidationError`] that occurred.
#[derive(Debug)]
pub enum MessageValidationErrorType {
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
    /// Too many embeds were provided.
    ///
    /// A followup message can have up to 10 embeds.
    TooManyEmbeds,
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
            crate::component::component(component).map_err(|source| {
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
        for (idx, embed) in embeds.iter().enumerate() {
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

#[cfg(test)]
mod tests {
    use super::content;

    #[test]
    fn test_content() {
        assert!(content("").is_ok());
        assert!(content("a".repeat(2000)).is_ok());

        assert!(content("a".repeat(2001)).is_err());
    }
}
