//! Create embeds.

use super::image_source::ImageSource;
use std::{
    convert::TryFrom,
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    mem,
};
use twilight_model::channel::embed::{
    Embed, EmbedAuthor, EmbedField, EmbedFooter, EmbedImage, EmbedThumbnail,
};

/// Error building an embed.
///
/// This is returned from [`EmbedBuilder::build`].
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum EmbedError {
    /// Name is empty.
    AuthorNameEmpty {
        /// Provided name. Although empty, the same owned allocation is
        /// included.
        name: String,
    },
    /// Name is longer than 256 UTF-16 code points.
    AuthorNameTooLong {
        /// Provided name.
        name: String,
    },
    /// Color was larger than a valid RGB hexadecimal value.
    ColorNotRgb {
        /// Provided color hex value.
        color: u32,
    },
    /// Color was 0. The value would be thrown out by Discord and is equivalent
    /// to null.
    ColorZero,
    /// Description is empty.
    DescriptionEmpty {
        /// Provided description. Although empty, the same owned allocation is
        /// included.
        description: String,
    },
    /// Description is longer than 2048 UTF-16 code points.
    DescriptionTooLong {
        /// Provided description.
        description: String,
    },
    /// Name is empty.
    FieldNameEmpty {
        /// Provided name. Although empty, the same owned allocation is
        /// included.
        name: String,
        /// Provided value.
        value: String,
    },
    /// Name is longer than 256 UTF-16 code points.
    FieldNameTooLong {
        /// Provided name.
        name: String,
        /// Provided value.
        value: String,
    },
    /// Value is empty.
    FieldValueEmpty {
        /// Provided name.
        name: String,
        /// Provided value. Although empty, the same owned allocation is
        /// included.
        value: String,
    },
    /// Value is longer than 1024 UTF-16 code points.
    FieldValueTooLong {
        /// Provided name.
        name: String,
        /// Provided value.
        value: String,
    },
    /// Footer text is empty.
    FooterTextEmpty {
        /// Provided text. Although empty, the same owned allocation is
        /// included.
        text: String,
    },
    /// Footer text is longer than 2048 UTF-16 code points.
    FooterTextTooLong {
        /// Provided text.
        text: String,
    },
    /// Title is empty.
    TitleEmpty {
        /// Provided title. Although empty, the same owned allocation is
        /// included.
        title: String,
    },
    /// Title is longer than 256 UTF-16 code points.
    TitleTooLong {
        /// Provided title.
        title: String,
    },
    /// The total content of the embed is too large.
    ///
    /// Refer to [`EmbedBuilder::EMBED_LENGTH_LIMIT`] for more information about
    /// what goes into this limit.
    TotalContentTooLarge {
        /// The total length of the embed.
        length: usize,
    },
    /// Too many fields were provided.
    ///
    /// Refer to [`EmbedBuilder::EMBED_FIELD_LIMIT`] for more information about
    /// what the limit is.
    TooManyFields {
        /// The provided fields.
        fields: Vec<EmbedField>,
    },
}

impl Display for EmbedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::AuthorNameEmpty { .. } => f.write_str("the author name is empty"),
            Self::AuthorNameTooLong { .. } => f.write_str("the author name is too long"),
            Self::ColorNotRgb { color } => {
                f.write_fmt(format_args!("the color {} is invalid", color))
            }
            Self::ColorZero => f.write_str("the given color value is 0, which is not acceptable"),
            Self::DescriptionEmpty { .. } => f.write_str("the description is empty"),
            Self::DescriptionTooLong { .. } => f.write_str("the description is too long"),
            Self::FieldNameEmpty { .. } => f.write_str("the field name is empty"),
            Self::FieldNameTooLong { .. } => f.write_str("the field name is too long"),
            Self::FieldValueEmpty { .. } => f.write_str("the field value is empty"),
            Self::FieldValueTooLong { .. } => f.write_str("the field value is too long"),
            Self::FooterTextEmpty { .. } => f.write_str("the footer text is empty"),
            Self::FooterTextTooLong { .. } => f.write_str("the footer text is too long"),
            Self::TitleEmpty { .. } => f.write_str("the title is empty"),
            Self::TitleTooLong { .. } => f.write_str("the title is too long"),
            Self::TotalContentTooLarge { .. } => {
                f.write_str("the total content of the embed is too large")
            }
            Self::TooManyFields { .. } => f.write_str("more than 25 fields were provided"),
        }
    }
}

impl Error for EmbedError {}

/// Create an embed with a builder.
///
/// # Examples
///
/// Refer to the [crate-level documentation] for examples.
///
/// [crate-level documentation]: crate
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into an embed"]
pub struct EmbedBuilder(Embed);

impl EmbedBuilder {
    /// The maximum number of UTF-16 code points that can be in an author name.
    pub const AUTHOR_NAME_LENGTH_LIMIT: usize = 256;

    /// The maximum accepted color value.
    pub const COLOR_MAXIMUM: u32 = 0xff_ff_ff;

    /// The maximum number of UTF-16 code points that can be in a description.
    pub const DESCRIPTION_LENGTH_LIMIT: usize = 2048;

    /// The maximum number of fields that can be in an embed.
    pub const EMBED_FIELD_LIMIT: usize = 25;

    /// The maximum total textual length of the embed in UTF-16 code points.
    ///
    /// This combines the text of the author name, description, footer text,
    /// field names and values, and title.
    pub const EMBED_LENGTH_LIMIT: usize = 6000;

    /// The maximum number of UTF-16 code points that can be in a field name.
    pub const FIELD_NAME_LENGTH_LIMIT: usize = 256;

    /// The maximum number of UTF-16 code points that can be in a field value.
    pub const FIELD_VALUE_LENGTH_LIMIT: usize = 1024;

    /// The maximum number of UTF-16 code points that can be in a footer's text.
    pub const FOOTER_TEXT_LENGTH_LIMIT: usize = 2048;

    /// The maximum number of UTF-16 code points that can be in a title.
    pub const TITLE_LENGTH_LIMIT: usize = 256;

    /// Create a new default embed builder.
    ///
    /// See the [crate-level documentation] for examples and additional
    /// information.
    ///
    /// This is equivalent to the [default implementation].
    ///
    /// [crate-level documentation]: crate
    /// [default implementation]: Self::default
    pub fn new() -> Self {
        Self::default()
    }

    /// Build this into an embed.
    ///
    /// # Errors
    ///
    /// Returns [`EmbedError::AuthorNameEmpty`] if the provided name is empty.
    ///
    /// Returns [`EmbedError::AuthorNameTooLong`] if the provided name is longer
    /// than [`AUTHOR_NAME_LENGTH_LIMIT`].
    ///
    /// Returns [`EmbedError::ColorNotRgb`] if the provided color is not a valid
    /// RGB integer. Refer to [`COLOR_MAXIMUM`] to know what the maximum
    /// accepted value is.
    ///
    /// Returns [`EmbedError::ColorZero`] if the provided color is 0, which is
    /// not an acceptable value.
    ///
    /// Returns [`EmbedError::DescriptionEmpty`] if a provided description is
    /// empty.
    ///
    /// Returns [`EmbedError::DescriptionTooLong`] if a provided description is
    /// longer than [`DESCRIPTION_LENGTH_LIMIT`].
    ///
    /// Returns [`EmbedError::FieldNameEmpty`] if a provided field name is
    /// empty.
    ///
    /// Returns [`EmbedError::FieldNameTooLong`] if a provided field name is
    /// longer than [`FIELD_NAME_LENGTH_LIMIT`].
    ///
    /// Returns [`EmbedError::FieldValueEmpty`] if a provided field value is
    /// empty.
    ///
    /// Returns [`EmbedError::FieldValueTooLong`] if a provided field value is
    /// longer than [`FIELD_VALUE_LENGTH_LIMIT`].
    ///
    /// Returns [`EmbedError::FooterTextEmpty`] if the provided text is empty.
    ///
    /// Returns [`EmbedError::FooterTextTooLong`] if the provided text is longer
    /// than the limit defined at [`FOOTER_TEXT_LENGTH_LIMIT`].
    ///
    /// Returns [`EmbedError::TitleEmpty`] if the provided title is empty.
    ///
    /// Returns [`EmbedError::TitleTooLong`] if the provided text is longer
    /// than the limit defined at [`TITLE_LENGTH_LIMIT`].
    ///
    /// Returns [`EmbedError::TooManyFields`] if there are too many fields
    /// in the embed. Refer to [`EMBED_FIELD_LIMIT`] for the limit value.
    ///
    /// Returns [`EmbedError::TotalContentTooLarge`] if the textual content of
    /// the embed is too large. Refer to [`EMBED_LENGTH_LIMIT`] for the limit
    /// value and what counts towards it.
    ///
    /// [`AUTHOR_NAME_LENGTH_LIMIT`]: Self::AUTHOR_NAME_LENGTH_LIMIT
    /// [`COLOR_MAXIMUM`]: Self::COLOR_MAXIMUM
    /// [`DESCRIPTION_LENGTH_LIMIT`]: Self::DESCRIPTION_LENGTH_LIMIT
    /// [`EMBED_FIELD_LIMIT`]: Self::EMBED_FIELD_LIMIT
    /// [`EMBED_LENGTH_LIMIT`]: Self::EMBED_LENGTH_LIMIT
    /// [`FIELD_NAME_LENGTH_LIMIT`]: Self::FIELD_NAME_LENGTH_LIMIT
    /// [`FIELD_VALUE_LENGTH_LIMIT`]: Self::FIELD_VALUE_LENGTH_LIMIT
    /// [`FOOTER_TEXT_LENGTH_LIMIT`]: Self::FOOTER_TEXT_LENGTH_LIMIT
    /// [`TITLE_LENGTH_LIMIT`]: Self::TITLE_LENGTH_LIMIT
    #[must_use = "should be used as part of something like a message"]
    pub fn build(mut self) -> Result<Embed, EmbedError> {
        if self.0.fields.len() > Self::EMBED_FIELD_LIMIT {
            return Err(EmbedError::TooManyFields {
                fields: self.0.fields,
            });
        }

        if let Some(color) = self.0.color {
            if color == 0 {
                return Err(EmbedError::ColorZero);
            }

            if color > Self::COLOR_MAXIMUM {
                return Err(EmbedError::ColorNotRgb { color });
            }
        }

        let mut total = 0;

        if let Some(mut author) = self.0.author.take() {
            if let Some(name) = author.name.take() {
                if name.is_empty() {
                    return Err(EmbedError::AuthorNameEmpty { name });
                }

                if name.chars().count() > Self::AUTHOR_NAME_LENGTH_LIMIT {
                    return Err(EmbedError::AuthorNameTooLong { name });
                }

                total += name.chars().count();
                author.name.replace(name);
            }

            self.0.author.replace(author);
        }

        if let Some(description) = self.0.description.take() {
            if description.is_empty() {
                return Err(EmbedError::DescriptionEmpty { description });
            }

            if description.chars().count() > Self::DESCRIPTION_LENGTH_LIMIT {
                return Err(EmbedError::DescriptionTooLong { description });
            }

            total += description.chars().count();
            self.0.description.replace(description);
        }

        if let Some(footer) = self.0.footer.take() {
            if footer.text.is_empty() {
                return Err(EmbedError::FooterTextEmpty { text: footer.text });
            }

            if footer.text.chars().count() > Self::FOOTER_TEXT_LENGTH_LIMIT {
                return Err(EmbedError::FooterTextTooLong { text: footer.text });
            }

            total += footer.text.chars().count();
            self.0.footer.replace(footer);
        }

        {
            let field_count = self.0.fields.len();
            let fields = mem::replace(&mut self.0.fields, Vec::with_capacity(field_count));

            for field in fields {
                if field.name.is_empty() {
                    return Err(EmbedError::FieldNameEmpty {
                        name: field.name,
                        value: field.value,
                    });
                }

                if field.name.chars().count() > Self::FIELD_NAME_LENGTH_LIMIT {
                    return Err(EmbedError::FieldNameTooLong {
                        name: field.name,
                        value: field.value,
                    });
                }

                if field.value.is_empty() {
                    return Err(EmbedError::FieldValueEmpty {
                        name: field.name,
                        value: field.value,
                    });
                }

                if field.value.chars().count() > Self::FIELD_VALUE_LENGTH_LIMIT {
                    return Err(EmbedError::FieldValueTooLong {
                        name: field.name,
                        value: field.value,
                    });
                }

                total += field.name.chars().count() + field.value.chars().count();
                self.0.fields.push(field);
            }
        }

        if let Some(title) = self.0.title.take() {
            if title.is_empty() {
                return Err(EmbedError::TitleEmpty { title });
            }

            if title.chars().count() > Self::TITLE_LENGTH_LIMIT {
                return Err(EmbedError::TitleTooLong { title });
            }

            total += title.chars().count();
            self.0.title.replace(title);
        }

        if total > Self::EMBED_LENGTH_LIMIT {
            return Err(EmbedError::TotalContentTooLarge { length: total });
        }

        Ok(self.0)
    }

    /// Set the author.
    ///
    /// # Examples
    ///
    /// Create an embed author:
    ///
    /// ```rust
    /// use twilight_embed_builder::{EmbedAuthorBuilder, EmbedBuilder};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let author = EmbedAuthorBuilder::new()
    ///     .name("Twilight")
    ///     .url("https://github.com/twilight-rs/twilight")
    ///     .build();
    ///
    /// let embed = EmbedBuilder::new().author(author).build()?;
    /// # Ok(()) }
    /// ```
    pub fn author(self, author: impl Into<EmbedAuthor>) -> Self {
        self._author(author.into())
    }

    fn _author(mut self, author: EmbedAuthor) -> Self {
        self.0.author.replace(author);

        self
    }

    /// Set the color.
    ///
    /// This must be a valid hexadecimal RGB value. `0x000000` is not an
    /// acceptable value as it would be thrown out by Discord. Refer to
    /// [`COLOR_MAXIMUM`] for the maximum acceptable value.
    ///
    /// # Examples
    ///
    /// Set the color of an embed to `0xfd69b3`:
    ///
    /// ```rust
    /// use twilight_embed_builder::EmbedBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let embed = EmbedBuilder::new()
    ///     .color(0xfd_69_b3)
    ///     .description("a description")
    ///     .build()?;
    /// # Ok(()) }
    /// ```
    ///
    /// [`COLOR_MAXIMUM`]: Self::COLOR_MAXIMUM
    pub fn color(mut self, color: u32) -> Self {
        self.0.color.replace(color);

        self
    }

    /// Set the description.
    ///
    /// Refer to [`DESCRIPTION_LENGTH_LIMIT`] for the maximum number of UTF-16
    /// code points that can be in a description.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twilight_embed_builder::EmbedBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let embed = EmbedBuilder::new().description("this is an embed").build()?;
    /// # Ok(()) }
    /// ```
    ///
    /// [`DESCRIPTION_LENGTH_LIMIT`]: Self::DESCRIPTION_LENGTH_LIMIT
    pub fn description(self, description: impl Into<String>) -> Self {
        self._description(description.into())
    }

    fn _description(mut self, description: String) -> Self {
        self.0.description.replace(description);

        self
    }

    /// Add a field to the embed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twilight_embed_builder::{EmbedBuilder, EmbedFieldBuilder};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let embed = EmbedBuilder::new()
    ///     .description("this is an embed")
    ///     .field(EmbedFieldBuilder::new("a field", "and its value"))
    ///     .build()?;
    /// # Ok(()) }
    /// ```
    pub fn field(self, field: impl Into<EmbedField>) -> Self {
        self._field(field.into())
    }

    fn _field(mut self, field: EmbedField) -> Self {
        self.0.fields.push(field);

        self
    }

    /// Set the footer of the embed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twilight_embed_builder::{EmbedBuilder, EmbedFooterBuilder};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let embed = EmbedBuilder::new()
    ///     .description("this is an embed")
    ///     .footer(EmbedFooterBuilder::new("a footer"))
    ///     .build()?;
    /// # Ok(()) }
    /// ```
    pub fn footer(self, footer: impl Into<EmbedFooter>) -> Self {
        self._footer(footer.into())
    }

    fn _footer(mut self, footer: EmbedFooter) -> Self {
        self.0.footer.replace(footer);

        self
    }

    /// Set the image.
    ///
    /// # Examples
    ///
    /// Set the image source to a URL:
    ///
    /// ```rust
    /// use twilight_embed_builder::{EmbedBuilder, EmbedFooterBuilder, ImageSource};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let source = ImageSource::url("https://raw.githubusercontent.com/twilight-rs/twilight/trunk/logo.png")?;
    /// let embed = EmbedBuilder::new()
    ///     .footer(EmbedFooterBuilder::new("twilight"))
    ///     .image(source)
    ///     .build()?;
    /// # Ok(()) }
    /// ```
    pub fn image(mut self, image_source: ImageSource) -> Self {
        self.0.image.replace(EmbedImage {
            height: None,
            proxy_url: None,
            url: Some(image_source.0),
            width: None,
        });

        self
    }

    /// Add a thumbnail.
    ///
    /// # Examples
    ///
    /// Set the thumbnail to an image attachment with the filename
    /// `"twilight.png"`:
    ///
    /// ```rust
    /// use twilight_embed_builder::{EmbedBuilder, ImageSource};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let embed = EmbedBuilder::new()
    ///     .description("a picture of twilight")
    ///     .thumbnail(ImageSource::attachment("twilight.png")?)
    ///     .build()?;
    /// # Ok(()) }
    /// ```
    pub fn thumbnail(mut self, image_source: ImageSource) -> Self {
        self.0.thumbnail.replace(EmbedThumbnail {
            height: None,
            proxy_url: None,
            url: Some(image_source.0),
            width: None,
        });

        self
    }

    /// Set the ISO 8601 timestamp.
    pub fn timestamp(self, timestamp: impl Into<String>) -> Self {
        self._timestamp(timestamp.into())
    }

    fn _timestamp(mut self, timestamp: String) -> Self {
        self.0.timestamp.replace(timestamp);

        self
    }

    /// Set the title.
    ///
    /// Refer to [`TITLE_LENGTH_LIMIT`] for the maximum number of UTF-16 code
    /// points that can be in a title.
    ///
    /// # Examples
    ///
    /// Set the title to "twilight":
    ///
    /// ```rust
    /// use twilight_embed_builder::EmbedBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let embed = EmbedBuilder::new()
    ///     .title("twilight")
    ///     .url("https://github.com/twilight-rs/twilight")
    ///     .build()?;
    /// # Ok(()) }
    /// ```
    ///
    /// [`TITLE_LENGTH_LIMIT`]: Self::TITLE_LENGTH_LIMIT
    pub fn title(self, title: impl Into<String>) -> Self {
        self._title(title.into())
    }

    fn _title(mut self, title: String) -> Self {
        self.0.title.replace(title);

        self
    }

    /// Set the URL.
    ///
    /// # Examples
    ///
    /// Set the URL to [twilight's repository]:
    ///
    /// ```rust
    /// use twilight_embed_builder::{EmbedBuilder, EmbedFooterBuilder};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let embed = EmbedBuilder::new()
    ///     .description("twilight's repository")
    ///     .url("https://github.com/twilight-rs/twilight")
    ///     .build()?;
    /// # Ok(()) }
    /// ```
    ///
    /// [twilight's repository]: https://github.com/twilight-rs/twilight
    pub fn url(self, url: impl Into<String>) -> Self {
        self._url(url.into())
    }

    fn _url(mut self, url: String) -> Self {
        self.0.url.replace(url);

        self
    }
}

impl Default for EmbedBuilder {
    /// Create an embed builder with a default embed.
    ///
    /// All embeds have a "rich" type.
    fn default() -> Self {
        EmbedBuilder(Embed {
            author: None,
            color: None,
            description: None,
            fields: Vec::new(),
            footer: None,
            image: None,
            kind: String::from("rich"),
            provider: None,
            thumbnail: None,
            timestamp: None,
            title: None,
            url: None,
            video: None,
        })
    }
}

impl TryFrom<EmbedBuilder> for Embed {
    type Error = EmbedError;

    /// Convert an embed builder into an embed.
    ///
    /// This is equivalent to calling [`EmbedBuilder::build`].
    fn try_from(builder: EmbedBuilder) -> Result<Self, Self::Error> {
        builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::{EmbedBuilder, EmbedError};
    use crate::{field::EmbedFieldBuilder, footer::EmbedFooterBuilder, image_source::ImageSource};
    use static_assertions::{assert_fields, assert_impl_all, const_assert};
    use std::{convert::TryFrom, error::Error, fmt::Debug};
    use twilight_model::channel::embed::{Embed, EmbedField, EmbedFooter};

    assert_impl_all!(EmbedError: Clone, Debug, Error, Eq, PartialEq, Send, Sync);
    assert_fields!(EmbedError::AuthorNameEmpty: name);
    assert_fields!(EmbedError::AuthorNameTooLong: name);
    assert_fields!(EmbedError::TooManyFields: fields);
    assert_fields!(EmbedError::ColorNotRgb: color);
    assert_fields!(EmbedError::DescriptionEmpty: description);
    assert_fields!(EmbedError::DescriptionTooLong: description);
    assert_fields!(EmbedError::FooterTextEmpty: text);
    assert_fields!(EmbedError::FooterTextTooLong: text);
    assert_fields!(EmbedError::TitleEmpty: title);
    assert_fields!(EmbedError::TitleTooLong: title);
    assert_fields!(EmbedError::TotalContentTooLarge: length);
    assert_fields!(EmbedError::FieldNameEmpty: name, value);
    assert_fields!(EmbedError::FieldNameTooLong: name, value);
    assert_fields!(EmbedError::FieldValueEmpty: name, value);
    assert_fields!(EmbedError::FieldValueTooLong: name, value);
    const_assert!(EmbedBuilder::AUTHOR_NAME_LENGTH_LIMIT == 256);
    const_assert!(EmbedBuilder::COLOR_MAXIMUM == 0xff_ff_ff);
    const_assert!(EmbedBuilder::DESCRIPTION_LENGTH_LIMIT == 2048);
    const_assert!(EmbedBuilder::EMBED_FIELD_LIMIT == 25);
    const_assert!(EmbedBuilder::EMBED_LENGTH_LIMIT == 6000);
    const_assert!(EmbedBuilder::FIELD_NAME_LENGTH_LIMIT == 256);
    const_assert!(EmbedBuilder::FIELD_VALUE_LENGTH_LIMIT == 1024);
    const_assert!(EmbedBuilder::FOOTER_TEXT_LENGTH_LIMIT == 2048);
    const_assert!(EmbedBuilder::TITLE_LENGTH_LIMIT == 256);
    assert_impl_all!(Embed: TryFrom<EmbedBuilder>);

    #[test]
    fn test_color_error() -> Result<(), Box<dyn Error>> {
        assert!(matches!(
            EmbedBuilder::new().color(0).build().unwrap_err(),
            EmbedError::ColorZero
        ));
        assert!(matches!(
            EmbedBuilder::new().color(u32::MAX).build().unwrap_err(),
            EmbedError::ColorNotRgb { color }
            if color == u32::MAX
        ));

        Ok(())
    }

    #[test]
    fn test_description_error() {
        assert!(matches!(
            EmbedBuilder::new().description("").build().unwrap_err(),
            EmbedError::DescriptionEmpty { description }
            if description.is_empty()
        ));
        let description_too_long = EmbedBuilder::DESCRIPTION_LENGTH_LIMIT + 1;
        assert!(matches!(
            EmbedBuilder::new().description("a".repeat(description_too_long)).build().unwrap_err(),
            EmbedError::DescriptionTooLong { description }
            if description.len() == description_too_long
        ));
    }

    #[test]
    fn test_title_error() {
        assert!(matches!(
            EmbedBuilder::new().title("").build().unwrap_err(),
            EmbedError::TitleEmpty { title }
            if title.is_empty()
        ));
        let title_too_long = EmbedBuilder::TITLE_LENGTH_LIMIT + 1;
        assert!(matches!(
            EmbedBuilder::new().title("a".repeat(title_too_long)).build().unwrap_err(),
            EmbedError::TitleTooLong { title }
            if title.len() == title_too_long
        ));
    }

    #[test]
    fn test_builder() {
        let footer_image = ImageSource::url(
            "https://raw.githubusercontent.com/twilight-rs/twilight/trunk/logo.png",
        )
        .unwrap();
        let embed = EmbedBuilder::new()
            .color(0x00_43_ff)
            .description("Description")
            .timestamp("123")
            .footer(EmbedFooterBuilder::new("Warn").icon_url(footer_image))
            .field(EmbedFieldBuilder::new("name", "title").inline())
            .build()
            .unwrap();

        let expected = Embed {
            author: None,
            color: Some(0x00_43_ff),
            description: Some("Description".to_string()),
            fields: [EmbedField {
                inline: true,
                name: "name".to_string(),
                value: "title".to_string(),
            }]
            .to_vec(),
            footer: Some(EmbedFooter {
                icon_url: Some(
                    "https://raw.githubusercontent.com/twilight-rs/twilight/trunk/logo.png"
                        .to_string(),
                ),
                proxy_icon_url: None,
                text: "Warn".to_string(),
            }),
            image: None,
            kind: "rich".to_string(),
            provider: None,
            thumbnail: None,
            timestamp: Some("123".to_string()),
            title: None,
            url: None,
            video: None,
        };

        assert_eq!(embed, expected);
    }
}
