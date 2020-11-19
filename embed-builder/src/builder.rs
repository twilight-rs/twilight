//! Create embeds.

use super::image_source::ImageSource;
use std::{
    convert::TryFrom,
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::channel::embed::{
    Embed, EmbedAuthor, EmbedField, EmbedFooter, EmbedImage, EmbedThumbnail,
};

/// Error building an embed.
///
/// This is returned from [`EmbedBuilder::build`].
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum EmbedBuildError {
    /// The total content of the embed is too large.
    ///
    /// Refer to [`EmbedBuilder::EMBED_LENGTH_LIMIT`] for more information about
    /// what goes into this limit.
    ContentTooLarge {
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

impl Display for EmbedBuildError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::ContentTooLarge { .. } => f.write_str("the content of the embed is too large"),
            Self::TooManyFields { .. } => f.write_str("more than 25 fields were provided"),
        }
    }
}

impl Error for EmbedBuildError {}

/// Error working with an embed builder.
///
/// This is returned from [`EmbedBuilder::color`].
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum EmbedColorError {
    /// Color was larger than a valid RGB hexadecimal value.
    NotRgb {
        /// Provided color hex value.
        color: u32,
    },
    /// Color was 0. The value would be thrown out by Discord and is equivalent
    /// to null.
    Zero,
}

impl Display for EmbedColorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::NotRgb { color } => f.write_fmt(format_args!("the color {} is invalid", color)),
            Self::Zero => f.write_str("the given color value is 0, which is not acceptable"),
        }
    }
}

impl Error for EmbedColorError {}

/// Error adding a description to an embed builder.
///
/// This is returned from [`EmbedBuilder::description`].
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum EmbedDescriptionError {
    /// Description is empty.
    Empty {
        /// Provided description. Although empty, the same owned allocation is
        /// included.
        description: String,
    },
    /// Description is longer than 2048 UTF-16 code points.
    TooLong {
        /// Provided description.
        description: String,
    },
}

impl Display for EmbedDescriptionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Empty { .. } => f.write_str("the description is empty"),
            Self::TooLong { .. } => f.write_str("the description is too long"),
        }
    }
}

impl Error for EmbedDescriptionError {}

/// Error adding a title to an embed builder.
///
/// This is returned from [`EmbedBuilder::title`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EmbedTitleError {
    /// Title is empty.
    Empty {
        /// Provided title. Although empty, the same owned allocation is
        /// included.
        title: String,
    },
    /// Title is longer than 256 UTF-16 code points.
    TooLong {
        /// Provided title.
        title: String,
    },
}

impl Display for EmbedTitleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Empty { .. } => f.write_str("the title is empty"),
            Self::TooLong { .. } => f.write_str("the title is too long"),
        }
    }
}

impl Error for EmbedTitleError {}

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
    /// The maximum accepted color value.
    ///
    /// This is used by [`color`].
    ///
    /// [`color`]: Self::color
    pub const COLOR_MAXIMUM: u32 = 0xff_ff_ff;

    /// The maximum number of UTF-16 code points that can be in a description.
    ///
    /// This is used by [`description`].
    ///
    /// [`description`]: Self::description
    pub const DESCRIPTION_LENGTH_LIMIT: usize = 2048;

    /// The maximum number of fields that can be in an embed.
    ///
    /// This is used by [`build`].
    ///
    /// [`build`]: Self::build
    pub const EMBED_FIELD_LIMIT: usize = 25;

    /// The maximum total textual length of the embed in UTF-16 code points.
    ///
    /// This combines the text of the author name, description, footer text,
    /// field names and values, and title.
    ///
    /// This is used by [`build`].
    ///
    /// [`build`]: Self::build
    pub const EMBED_LENGTH_LIMIT: usize = 6000;

    /// The maximum number of UTF-16 code points that can be in a title.
    ///
    /// This is used by [`title`].
    ///
    /// [`title`]: Self::title
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
    /// Returns [`EmbedBuildError::ContentTooLarge`] if the textual content of
    /// the embed is too large. Refer to [`EMBED_LENGTH_LIMIT`] for the limit
    /// value and what counts towards it.
    ///
    /// Returns [`EmbedBuildError::TooManyFields`] if there are too many fields
    /// in the embed. Refer to [`EMBED_FIELD_LIMIT`] for the limit value.
    ///
    /// [`EMBED_FIELD_LIMIT`]: Self::EMBED_FIELD_LIMIT
    /// [`EMBED_LENGTH_LIMIT`]: Self::EMBED_LENGTH_LIMIT
    #[must_use = "should be used as part of something like a message"]
    pub fn build(self) -> Result<Embed, EmbedBuildError> {
        if self.0.fields.len() > Self::EMBED_FIELD_LIMIT {
            return Err(EmbedBuildError::TooManyFields {
                fields: self.0.fields,
            });
        }

        let mut total = 0;

        if let Some(name) = self
            .0
            .author
            .as_ref()
            .and_then(|author| author.name.as_ref())
        {
            total += name.chars().count();
        }

        if let Some(description) = self.0.description.as_ref() {
            total += description.chars().count();
        }

        if let Some(footer) = self.0.footer.as_ref() {
            total += footer.text.chars().count();
        }

        for field in &self.0.fields {
            total += field.name.chars().count() + field.value.chars().count();
        }

        if let Some(title) = self.0.title.as_ref() {
            total += title.chars().count();
        }

        if total > Self::EMBED_LENGTH_LIMIT {
            return Err(EmbedBuildError::ContentTooLarge { length: total });
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
    ///     .name("Twilight")?
    ///     .url("https://github.com/twilight-rs/twilight")
    ///     .build();
    ///
    /// let embed = EmbedBuilder::new().author(author).build();
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
    /// acceptable value as it would be thrown out by Discord.
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
    ///     .color(0xfd_69_b3)?
    ///     .description("a description")?
    ///     .build();
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`EmbedColorError::NotRgb`] if the provided color is not a valid
    /// RGB integer. Refer to [`COLOR_MAXIMUM`] to know what the maximum
    /// accepted value is.
    ///
    /// Returns [`EmbedColorError::Zero`] if the provided color is 0, which is not
    /// an acceptable value.
    ///
    /// [`COLOR_MAXIMUM`]: Self::COLOR_MAXIMUM
    pub fn color(mut self, color: u32) -> Result<Self, EmbedColorError> {
        if color == 0 {
            return Err(EmbedColorError::Zero);
        }

        if color > Self::COLOR_MAXIMUM {
            return Err(EmbedColorError::NotRgb { color });
        }

        self.0.color.replace(color);

        Ok(self)
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
    /// let embed = EmbedBuilder::new().description("this is an embed")?.build();
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`EmbedDescriptionError::TooLong`] if the provided
    /// description is longer than the maximum number of code points.
    ///
    /// [`DESCRIPTION_LENGTH_LIMIT`]: Self::DESCRIPTION_LENGTH_LIMIT
    pub fn description(
        self,
        description: impl Into<String>,
    ) -> Result<Self, EmbedDescriptionError> {
        self._description(description.into())
    }

    fn _description(mut self, description: String) -> Result<Self, EmbedDescriptionError> {
        if description.is_empty() {
            return Err(EmbedDescriptionError::Empty { description });
        }

        if description.chars().count() > Self::DESCRIPTION_LENGTH_LIMIT {
            return Err(EmbedDescriptionError::TooLong { description });
        }

        self.0.description.replace(description);

        Ok(self)
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
    ///     .description("this is an embed")?
    ///     .field(EmbedFieldBuilder::new("a field", "and its value")?)
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
    ///     .description("this is an embed")?
    ///     .footer(EmbedFooterBuilder::new("a footer")?)
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
    ///     .footer(EmbedFooterBuilder::new("twilight")?)
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
    ///     .description("a picture of twilight")?
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
    ///     .title("twilight")?
    ///     .url("https://github.com/twilight-rs/twilight")
    ///     .build()?;
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`EmbedTitleError::Empty`] if the provided title is empty.
    ///
    /// Returns [`EmbedTitleError::TooLong`] if the provided title is longer
    /// than the limit defined at [`TITLE_LENGTH_LIMIT`].
    ///
    /// [`TITLE_LENGTH_LIMIT`]: Self::TITLE_LENGTH_LIMIT
    pub fn title(self, title: impl Into<String>) -> Result<Self, EmbedTitleError> {
        self._title(title.into())
    }

    fn _title(mut self, title: String) -> Result<Self, EmbedTitleError> {
        if title.is_empty() {
            return Err(EmbedTitleError::Empty { title });
        }

        if title.chars().count() > Self::TITLE_LENGTH_LIMIT {
            return Err(EmbedTitleError::TooLong { title });
        }

        self.0.title.replace(title);

        Ok(self)
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
    ///     .description("twilight's repository")?
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
    type Error = EmbedBuildError;

    /// Convert an embed builder into an embed.
    ///
    /// This is equivalent to calling [`EmbedBuilder::build`].
    fn try_from(builder: EmbedBuilder) -> Result<Self, Self::Error> {
        builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        EmbedBuildError, EmbedBuilder, EmbedColorError, EmbedDescriptionError, EmbedTitleError,
    };
    use crate::{field::EmbedFieldBuilder, footer::EmbedFooterBuilder, image_source::ImageSource};
    use static_assertions::{assert_fields, assert_impl_all, const_assert};
    use std::{convert::TryFrom, error::Error, fmt::Debug};
    use twilight_model::channel::embed::{Embed, EmbedField, EmbedFooter};

    assert_impl_all!(
        EmbedBuildError: Clone,
        Debug,
        Error,
        Eq,
        PartialEq,
        Send,
        Sync
    );
    assert_fields!(EmbedBuildError::ContentTooLarge: length);
    assert_fields!(EmbedBuildError::TooManyFields: fields);
    assert_impl_all!(
        EmbedColorError: Clone,
        Debug,
        Error,
        Eq,
        PartialEq,
        Send,
        Sync
    );
    assert_fields!(EmbedColorError::NotRgb: color);
    assert_impl_all!(
        EmbedDescriptionError: Clone,
        Debug,
        Error,
        Eq,
        PartialEq,
        Send,
        Sync
    );
    assert_fields!(EmbedDescriptionError::Empty: description);
    assert_fields!(EmbedDescriptionError::TooLong: description);
    assert_impl_all!(
        EmbedTitleError: Clone,
        Debug,
        Error,
        Eq,
        PartialEq,
        Send,
        Sync
    );
    assert_fields!(EmbedTitleError::Empty: title);
    assert_fields!(EmbedTitleError::TooLong: title);
    assert_impl_all!(
        EmbedBuilder: Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        Send,
        Sync
    );
    const_assert!(EmbedBuilder::COLOR_MAXIMUM == 0xff_ff_ff);
    const_assert!(EmbedBuilder::DESCRIPTION_LENGTH_LIMIT == 2048);
    const_assert!(EmbedBuilder::EMBED_FIELD_LIMIT == 25);
    const_assert!(EmbedBuilder::EMBED_LENGTH_LIMIT == 6000);
    const_assert!(EmbedBuilder::TITLE_LENGTH_LIMIT == 256);
    assert_impl_all!(Embed: TryFrom<EmbedBuilder>);

    #[test]
    fn test_color_error() -> Result<(), Box<dyn Error>> {
        assert!(matches!(
            EmbedBuilder::new().color(0).unwrap_err(),
            EmbedColorError::Zero
        ));
        assert!(matches!(
            EmbedBuilder::new().color(u32::MAX).unwrap_err(),
            EmbedColorError::NotRgb { color }
            if color == u32::MAX
        ));

        Ok(())
    }

    #[test]
    fn test_description_error() {
        assert!(matches!(
            EmbedBuilder::new().description("").unwrap_err(),
            EmbedDescriptionError::Empty { description }
            if description.is_empty()
        ));
        let description_too_long = EmbedBuilder::DESCRIPTION_LENGTH_LIMIT + 1;
        assert!(matches!(
            EmbedBuilder::new().description("a".repeat(description_too_long)).unwrap_err(),
            EmbedDescriptionError::TooLong { description }
            if description.len() == description_too_long
        ));
    }

    #[test]
    fn test_title_error() {
        assert!(matches!(
            EmbedBuilder::new().title("").unwrap_err(),
            EmbedTitleError::Empty { title }
            if title.is_empty()
        ));
        let title_too_long = EmbedBuilder::TITLE_LENGTH_LIMIT + 1;
        assert!(matches!(
            EmbedBuilder::new().title("a".repeat(title_too_long)).unwrap_err(),
            EmbedTitleError::TooLong { title }
            if title.len() == title_too_long
        ));
    }

    #[test]
    fn test_builder() -> Result<(), Box<dyn Error>> {
        let footer_image = ImageSource::url(
            "https://raw.githubusercontent.com/twilight-rs/twilight/trunk/logo.png",
        )?;
        let embed = EmbedBuilder::new()
            .color(0x00_43_ff)?
            .description("Description")?
            .timestamp("123")
            .footer(EmbedFooterBuilder::new("Warn")?.icon_url(footer_image))
            .field(EmbedFieldBuilder::new("name", "title")?.inline())
            .build()?;

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

        Ok(())
    }
}
