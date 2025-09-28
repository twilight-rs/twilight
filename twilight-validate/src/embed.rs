//! Constants, error types, and functions for validating [`Embed`]s.

use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::channel::message::Embed;

/// The maximum embed author name length in codepoints.
pub const AUTHOR_NAME_LENGTH: usize = 256;

/// The maximum accepted color value.
pub const COLOR_MAXIMUM: u32 = 0xff_ff_ff;

/// The maximum embed description length in codepoints.
pub const DESCRIPTION_LENGTH: usize = 4096;

/// The maximum combined embed length in codepoints.
pub const EMBED_TOTAL_LENGTH: usize = 6000;

/// The maximum number of fields in an embed.
pub const FIELD_COUNT: usize = 25;

/// The maximum length of an embed field name in codepoints.
pub const FIELD_NAME_LENGTH: usize = 256;

/// The maximum length of an embed field value in codepoints.
pub const FIELD_VALUE_LENGTH: usize = 1024;

/// The maximum embed footer length in codepoints.
pub const FOOTER_TEXT_LENGTH: usize = 2048;

/// The maximum embed title length in codepoints.
pub const TITLE_LENGTH: usize = 256;

/// An embed is not valid.
///
/// Referenced values are from [Discord Docs/Embed Limits].
///
/// [Discord Docs/Embed Limits]: https://discord.com/developers/docs/resources/channel#embed-limits
#[derive(Debug)]
pub struct EmbedValidationError {
    /// Type of error that occurred.
    kind: EmbedValidationErrorType,
}

impl EmbedValidationError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &EmbedValidationErrorType {
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
        EmbedValidationErrorType,
        Option<Box<dyn Error + Send + Sync>>,
    ) {
        (self.kind, None)
    }
}

impl Display for EmbedValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            EmbedValidationErrorType::AuthorNameTooLarge { chars } => {
                f.write_str("the author name is ")?;
                Display::fmt(chars, f)?;
                f.write_str(" characters long, but the max is ")?;

                Display::fmt(&AUTHOR_NAME_LENGTH, f)
            }
            EmbedValidationErrorType::ColorNotRgb { color } => {
                f.write_str("the color is ")?;
                Display::fmt(color, f)?;
                f.write_str(", but it must be less than ")?;

                Display::fmt(&COLOR_MAXIMUM, f)
            }
            EmbedValidationErrorType::DescriptionTooLarge { chars } => {
                f.write_str("the description is ")?;
                Display::fmt(chars, f)?;
                f.write_str(" characters long, but the max is ")?;

                Display::fmt(&DESCRIPTION_LENGTH, f)
            }
            EmbedValidationErrorType::EmbedTooLarge { chars } => {
                f.write_str("the combined total length of the embed is ")?;
                Display::fmt(chars, f)?;
                f.write_str(" characters long, but the max is ")?;

                Display::fmt(&EMBED_TOTAL_LENGTH, f)
            }
            EmbedValidationErrorType::FieldNameTooLarge { chars } => {
                f.write_str("a field name is ")?;
                Display::fmt(chars, f)?;
                f.write_str(" characters long, but the max is ")?;

                Display::fmt(&FIELD_NAME_LENGTH, f)
            }
            EmbedValidationErrorType::FieldValueTooLarge { chars } => {
                f.write_str("a field value is ")?;
                Display::fmt(chars, f)?;
                f.write_str(" characters long, but the max is ")?;

                Display::fmt(&FIELD_VALUE_LENGTH, f)
            }
            EmbedValidationErrorType::FooterTextTooLarge { chars } => {
                f.write_str("the footer's text is ")?;
                Display::fmt(chars, f)?;
                f.write_str(" characters long, but the max is ")?;

                Display::fmt(&FOOTER_TEXT_LENGTH, f)
            }
            EmbedValidationErrorType::TitleTooLarge { chars } => {
                f.write_str("the title's length is ")?;
                Display::fmt(chars, f)?;
                f.write_str(" characters long, but the max is ")?;

                Display::fmt(&TITLE_LENGTH, f)
            }
            EmbedValidationErrorType::TooManyFields { amount } => {
                f.write_str("there are ")?;
                Display::fmt(amount, f)?;
                f.write_str(" fields, but the maximum amount is ")?;

                Display::fmt(&FIELD_COUNT, f)
            }
        }
    }
}

impl Error for EmbedValidationError {}

/// Type of [`EmbedValidationError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum EmbedValidationErrorType {
    /// Embed author's name is larger than [`AUTHOR_NAME_LENGTH`].
    AuthorNameTooLarge {
        /// Provided number of codepoints.
        chars: usize,
    },
    /// Color is larger than a valid RGB hexadecimal value.
    ColorNotRgb {
        /// Provided color hex value.
        color: u32,
    },
    /// Embed description is larger than [`DESCRIPTION_LENGTH`].
    DescriptionTooLarge {
        /// Provided number of codepoints.
        chars: usize,
    },
    /// Combined content of all embed fields is larger than
    /// [`EMBED_TOTAL_LENGTH`].
    ///
    /// This includes author name, description, footer, field names and values,
    /// and title.
    EmbedTooLarge {
        /// Provided number of codepoints.
        chars: usize,
    },
    /// A field's name is larger than [`FIELD_NAME_LENGTH`].
    FieldNameTooLarge {
        /// Provided number of codepoints.
        chars: usize,
    },
    /// A field's value is larger than [`FIELD_VALUE_LENGTH`].
    FieldValueTooLarge {
        /// Provided number of codepoints.
        chars: usize,
    },
    /// Footer text is larger than [`FOOTER_TEXT_LENGTH`].
    FooterTextTooLarge {
        /// Provided number of codepoints.
        chars: usize,
    },
    /// Title is larger than [`TITLE_LENGTH`].
    TitleTooLarge {
        /// Provided number of codepoints.
        chars: usize,
    },
    /// There are more than [`FIELD_COUNT`] number of fields in the embed.
    TooManyFields {
        /// Provided number of fields.
        amount: usize,
    },
}

/// Ensure an embed is correct.
///
/// # Errors
///
/// Returns an error of type [`AuthorNameTooLarge`] if
/// the author's name is too large.
///
/// Returns an error of type [`ColorNotRgb`] if if the provided color is not a
/// valid RGB integer. Refer to [`COLOR_MAXIMUM`] to know what the maximum
/// accepted value is.
///
/// Returns an error of type [`DescriptionTooLarge`] if the description is too
/// large.
///
/// Returns an error of type [`EmbedTooLarge`] if the embed in total is too
/// large.
///
/// Returns an error of type [`FieldNameTooLarge`] if
/// a field's name is too long.
///
/// Returns an error of type [`FieldValueTooLarge`] if
/// a field's value is too long.
///
/// Returns an error of type [`FooterTextTooLarge`] if
/// the footer text is too long.
///
/// Returns an error of type [`TitleTooLarge`] if the title is too long.
///
/// Returns an error of type [`TooManyFields`] if
/// there are too many fields.
///
/// [`AuthorNameTooLarge`]: EmbedValidationErrorType::AuthorNameTooLarge
/// [`ColorNotRgb`]: EmbedValidationErrorType::ColorNotRgb
/// [`DescriptionTooLarge`]: EmbedValidationErrorType::DescriptionTooLarge
/// [`EmbedTooLarge`]: EmbedValidationErrorType::EmbedTooLarge
/// [`FieldNameTooLarge`]: EmbedValidationErrorType::FieldNameTooLarge
/// [`FieldValueTooLarge`]: EmbedValidationErrorType::FieldValueTooLarge
/// [`FooterTextTooLarge`]: EmbedValidationErrorType::FooterTextTooLarge
/// [`TitleTooLarge`]: EmbedValidationErrorType::TitleTooLarge
/// [`TooManyFields`]: EmbedValidationErrorType::TooManyFields
pub fn embed(embed: &Embed) -> Result<(), EmbedValidationError> {
    let chars = self::chars(embed);

    if chars > EMBED_TOTAL_LENGTH {
        return Err(EmbedValidationError {
            kind: EmbedValidationErrorType::EmbedTooLarge { chars },
        });
    }

    if let Some(color) = embed.color
        && color > COLOR_MAXIMUM
    {
        return Err(EmbedValidationError {
            kind: EmbedValidationErrorType::ColorNotRgb { color },
        });
    }

    if let Some(description) = embed.description.as_ref() {
        let chars = description.chars().count();

        if chars > DESCRIPTION_LENGTH {
            return Err(EmbedValidationError {
                kind: EmbedValidationErrorType::DescriptionTooLarge { chars },
            });
        }
    }

    if embed.fields.len() > FIELD_COUNT {
        return Err(EmbedValidationError {
            kind: EmbedValidationErrorType::TooManyFields {
                amount: embed.fields.len(),
            },
        });
    }

    for field in &embed.fields {
        let name_chars = field.name.chars().count();

        if name_chars > FIELD_NAME_LENGTH {
            return Err(EmbedValidationError {
                kind: EmbedValidationErrorType::FieldNameTooLarge { chars: name_chars },
            });
        }

        let value_chars = field.value.chars().count();

        if value_chars > FIELD_VALUE_LENGTH {
            return Err(EmbedValidationError {
                kind: EmbedValidationErrorType::FieldValueTooLarge { chars: value_chars },
            });
        }
    }

    if let Some(footer) = embed.footer.as_ref() {
        let chars = footer.text.chars().count();

        if chars > FOOTER_TEXT_LENGTH {
            return Err(EmbedValidationError {
                kind: EmbedValidationErrorType::FooterTextTooLarge { chars },
            });
        }
    }

    if let Some(name) = embed.author.as_ref().map(|author| &author.name) {
        let chars = name.chars().count();

        if chars > AUTHOR_NAME_LENGTH {
            return Err(EmbedValidationError {
                kind: EmbedValidationErrorType::AuthorNameTooLarge { chars },
            });
        }
    }

    if let Some(title) = embed.title.as_ref() {
        let chars = title.chars().count();

        if chars > TITLE_LENGTH {
            return Err(EmbedValidationError {
                kind: EmbedValidationErrorType::TitleTooLarge { chars },
            });
        }
    }

    Ok(())
}

/// Calculate the total character count of an embed.
#[must_use]
pub fn chars(embed: &Embed) -> usize {
    let mut chars = 0;

    if let Some(author) = &embed.author {
        chars += author.name.len();
    }

    if let Some(description) = &embed.description {
        chars += description.len();
    }

    if let Some(footer) = &embed.footer {
        chars += footer.text.len();
    }

    for field in &embed.fields {
        chars += field.name.len();
        chars += field.value.len();
    }

    if let Some(title) = &embed.title {
        chars += title.len();
    }

    chars
}

#[cfg(test)]
mod tests {
    use super::{EmbedValidationError, EmbedValidationErrorType};
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;
    use twilight_model::channel::message::{
        Embed,
        embed::{EmbedAuthor, EmbedField, EmbedFooter},
    };

    assert_impl_all!(EmbedValidationErrorType: Debug, Send, Sync);
    assert_impl_all!(EmbedValidationError: Debug, Send, Sync);

    fn base_embed() -> Embed {
        Embed {
            author: None,
            color: None,
            description: None,
            fields: Vec::new(),
            footer: None,
            image: None,
            kind: "rich".to_owned(),
            provider: None,
            thumbnail: None,
            timestamp: None,
            title: None,
            url: None,
            video: None,
        }
    }

    #[test]
    fn embed_base() {
        let embed = base_embed();

        assert!(super::embed(&embed).is_ok());
    }

    #[test]
    fn embed_normal() {
        let mut embed = base_embed();
        embed.author.replace(EmbedAuthor {
            icon_url: None,
            name: "twilight".to_owned(),
            proxy_icon_url: None,
            url: None,
        });
        embed.color.replace(0xff_00_00);
        embed.description.replace("a".repeat(100));
        embed.fields.push(EmbedField {
            inline: true,
            name: "b".repeat(25),
            value: "c".repeat(200),
        });
        embed.title.replace("this is a normal title".to_owned());

        assert!(super::embed(&embed).is_ok());
    }

    #[test]
    fn embed_author_name_limit() {
        let mut embed = base_embed();
        embed.author.replace(EmbedAuthor {
            icon_url: None,
            name: str::repeat("a", 256),
            proxy_icon_url: None,
            url: None,
        });
        assert!(super::embed(&embed).is_ok());

        embed.author.replace(EmbedAuthor {
            icon_url: None,
            name: str::repeat("a", 257),
            proxy_icon_url: None,
            url: None,
        });
        assert!(matches!(
            super::embed(&embed).unwrap_err().kind(),
            EmbedValidationErrorType::AuthorNameTooLarge { chars: 257 }
        ));
    }

    #[test]
    fn embed_description_limit() {
        let mut embed = base_embed();
        embed.description.replace(str::repeat("a", 2048));
        assert!(super::embed(&embed).is_ok());

        embed.description.replace(str::repeat("a", 4096));
        assert!(super::embed(&embed).is_ok());

        embed.description.replace(str::repeat("a", 4097));
        assert!(matches!(
            super::embed(&embed).unwrap_err().kind(),
            EmbedValidationErrorType::DescriptionTooLarge { chars: 4097 }
        ));
    }

    #[test]
    fn embed_field_count_limit() {
        let mut embed = base_embed();

        for _ in 0..26 {
            embed.fields.push(EmbedField {
                inline: true,
                name: "a".to_owned(),
                value: "a".to_owned(),
            });
        }

        assert!(matches!(
            super::embed(&embed).unwrap_err().kind(),
            EmbedValidationErrorType::TooManyFields { amount: 26 }
        ));
    }

    #[test]
    fn embed_field_name_limit() {
        let mut embed = base_embed();
        embed.fields.push(EmbedField {
            inline: true,
            name: str::repeat("a", 256),
            value: "a".to_owned(),
        });
        assert!(super::embed(&embed).is_ok());

        embed.fields.push(EmbedField {
            inline: true,
            name: str::repeat("a", 257),
            value: "a".to_owned(),
        });
        assert!(matches!(
            super::embed(&embed).unwrap_err().kind(),
            EmbedValidationErrorType::FieldNameTooLarge { chars: 257 }
        ));
    }

    #[test]
    fn embed_field_value_limit() {
        let mut embed = base_embed();
        embed.fields.push(EmbedField {
            inline: true,
            name: "a".to_owned(),
            value: str::repeat("a", 1024),
        });
        assert!(super::embed(&embed).is_ok());

        embed.fields.push(EmbedField {
            inline: true,
            name: "a".to_owned(),
            value: str::repeat("a", 1025),
        });
        assert!(matches!(
            super::embed(&embed).unwrap_err().kind(),
            EmbedValidationErrorType::FieldValueTooLarge { chars: 1025 }
        ));
    }

    #[test]
    fn embed_footer_text_limit() {
        let mut embed = base_embed();
        embed.footer.replace(EmbedFooter {
            icon_url: None,
            proxy_icon_url: None,
            text: str::repeat("a", 2048),
        });
        assert!(super::embed(&embed).is_ok());

        embed.footer.replace(EmbedFooter {
            icon_url: None,
            proxy_icon_url: None,
            text: str::repeat("a", 2049),
        });
        assert!(matches!(
            super::embed(&embed).unwrap_err().kind(),
            EmbedValidationErrorType::FooterTextTooLarge { chars: 2049 }
        ));
    }

    #[test]
    fn embed_title_limit() {
        let mut embed = base_embed();
        embed.title.replace(str::repeat("a", 256));
        assert!(super::embed(&embed).is_ok());

        embed.title.replace(str::repeat("a", 257));
        assert!(matches!(
            super::embed(&embed).unwrap_err().kind(),
            EmbedValidationErrorType::TitleTooLarge { chars: 257 }
        ));
    }

    #[test]
    fn embed_combined_limit() {
        let mut embed = base_embed();
        embed.description.replace(str::repeat("a", 2048));
        embed.title.replace(str::repeat("a", 256));

        for _ in 0..5 {
            embed.fields.push(EmbedField {
                inline: true,
                name: str::repeat("a", 100),
                value: str::repeat("a", 500),
            });
        }

        // we're at 5304 characters now
        assert!(super::embed(&embed).is_ok());

        embed.footer.replace(EmbedFooter {
            icon_url: None,
            proxy_icon_url: None,
            text: str::repeat("a", 1000),
        });

        assert!(matches!(
            super::embed(&embed).unwrap_err().kind(),
            EmbedValidationErrorType::EmbedTooLarge { chars: 6304 }
        ));
    }
}
