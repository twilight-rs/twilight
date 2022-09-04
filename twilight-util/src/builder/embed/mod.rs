//! Create an [`Embed`] with a builder.

pub mod image_source;

mod author;
mod field;
mod footer;

pub use self::{
    author::EmbedAuthorBuilder, field::EmbedFieldBuilder, footer::EmbedFooterBuilder,
    image_source::ImageSource,
};

use twilight_model::{
    channel::message::embed::{
        Embed, EmbedAuthor, EmbedField, EmbedFooter, EmbedImage, EmbedThumbnail,
    },
    util::Timestamp,
};
use twilight_validate::embed::{embed as validate_embed, EmbedValidationError};

/// Create an [`Embed`] with a builder.
///
/// # Examples
///
/// Build a simple embed:
///
/// ```no_run
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use twilight_util::builder::embed::{EmbedBuilder, EmbedFieldBuilder};
///
/// let embed = EmbedBuilder::new()
///     .description("Here's a list of reasons why Twilight is the best pony:")
///     .field(EmbedFieldBuilder::new("Wings", "She has wings.").inline())
///     .field(
///         EmbedFieldBuilder::new("Horn", "She can do magic, and she's really good at it.")
///             .inline(),
///     )
///     .validate()?
///     .build();
/// # Ok(()) }
/// ```
///
/// Build an embed with an image:
///
/// ```no_run
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use twilight_util::builder::embed::{EmbedBuilder, ImageSource};
///
/// let embed = EmbedBuilder::new()
///     .description("Here's a cool image of Twilight Sparkle")
///     .image(ImageSource::attachment("bestpony.png")?)
///     .validate()?
///     .build();
/// # Ok(()) }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into an embed"]
pub struct EmbedBuilder(Embed);

impl EmbedBuilder {
    /// Create a new embed builder.
    pub fn new() -> Self {
        EmbedBuilder(Embed {
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
        })
    }

    /// Build this into an embed.
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "should be used as part of something like a message"]
    pub fn build(self) -> Embed {
        self.0
    }

    /// Ensure the embed is valid.
    ///
    /// # Errors
    ///
    /// Refer to the documentation of [`twilight_validate::embed::embed`] for
    /// possible errors.
    pub fn validate(self) -> Result<Self, EmbedValidationError> {
        if let Err(source) = validate_embed(&self.0) {
            return Err(source);
        }

        Ok(self)
    }

    /// Set the author.
    ///
    /// # Examples
    ///
    /// Create an embed author:
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use twilight_util::builder::embed::{EmbedAuthorBuilder, EmbedBuilder};
    ///
    /// let author = EmbedAuthorBuilder::new("Twilight")
    ///     .url("https://github.com/twilight-rs/twilight")
    ///     .build();
    ///
    /// let embed = EmbedBuilder::new().author(author).validate()?.build();
    /// # Ok(()) }
    /// ```
    pub fn author(mut self, author: impl Into<EmbedAuthor>) -> Self {
        self.0.author = Some(author.into());

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
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use twilight_util::builder::embed::EmbedBuilder;
    ///
    /// let embed = EmbedBuilder::new()
    ///     .color(0xfd_69_b3)
    ///     .description("a description")
    ///     .validate()?
    ///     .build();
    /// # Ok(()) }
    /// ```
    ///
    /// [`COLOR_MAXIMUM`]: twilight_validate::embed::COLOR_MAXIMUM
    pub const fn color(mut self, color: u32) -> Self {
        self.0.color = Some(color);

        self
    }

    /// Set the description.
    ///
    /// Refer to [`DESCRIPTION_LENGTH`] for the maximum number of UTF-16 code
    /// points that can be in a description.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use twilight_util::builder::embed::EmbedBuilder;
    ///
    /// let embed = EmbedBuilder::new()
    ///     .description("this is an embed")
    ///     .validate()?
    ///     .build();
    /// # Ok(()) }
    /// ```
    ///
    /// [`DESCRIPTION_LENGTH`]: twilight_validate::embed::DESCRIPTION_LENGTH
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.0.description = Some(description.into());

        self
    }

    /// Add a field to the embed.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use twilight_util::builder::embed::{EmbedBuilder, EmbedFieldBuilder};
    ///
    /// let embed = EmbedBuilder::new()
    ///     .description("this is an embed")
    ///     .field(EmbedFieldBuilder::new("a field", "and its value"))
    ///     .validate()?
    ///     .build();
    /// # Ok(()) }
    /// ```
    pub fn field(mut self, field: impl Into<EmbedField>) -> Self {
        self.0.fields.push(field.into());

        self
    }

    /// Set the footer of the embed.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use twilight_util::builder::embed::{EmbedBuilder, EmbedFooterBuilder};
    ///
    /// let embed = EmbedBuilder::new()
    ///     .description("this is an embed")
    ///     .footer(EmbedFooterBuilder::new("a footer"))
    ///     .validate()?
    ///     .build();
    /// # Ok(()) }
    /// ```
    pub fn footer(mut self, footer: impl Into<EmbedFooter>) -> Self {
        self.0.footer = Some(footer.into());

        self
    }

    /// Set the image.
    ///
    /// # Examples
    ///
    /// Set the image source to a URL:
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use twilight_util::builder::embed::{EmbedBuilder, EmbedFooterBuilder, ImageSource};
    ///
    /// let source =
    ///     ImageSource::url("https://raw.githubusercontent.com/twilight-rs/twilight/main/logo.png")?;
    /// let embed = EmbedBuilder::new()
    ///     .footer(EmbedFooterBuilder::new("twilight"))
    ///     .image(source)
    ///     .validate()?
    ///     .build();
    /// # Ok(()) }
    /// ```
    #[allow(clippy::missing_const_for_fn)]
    pub fn image(mut self, image_source: ImageSource) -> Self {
        self.0.image = Some(EmbedImage {
            height: None,
            proxy_url: None,
            url: image_source.0,
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
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use twilight_util::builder::embed::{EmbedBuilder, ImageSource};
    ///
    /// let embed = EmbedBuilder::new()
    ///     .description("a picture of twilight")
    ///     .thumbnail(ImageSource::attachment("twilight.png")?)
    ///     .validate()?
    ///     .build();
    /// # Ok(()) }
    /// ```
    #[allow(clippy::missing_const_for_fn)]
    pub fn thumbnail(mut self, image_source: ImageSource) -> Self {
        self.0.thumbnail = Some(EmbedThumbnail {
            height: None,
            proxy_url: None,
            url: image_source.0,
            width: None,
        });

        self
    }

    /// Set the ISO 8601 timestamp.
    pub const fn timestamp(mut self, timestamp: Timestamp) -> Self {
        self.0.timestamp = Some(timestamp);

        self
    }

    /// Set the title.
    ///
    /// Refer to [`TITLE_LENGTH`] for the maximum number of UTF-16 code points
    /// that can be in a title.
    ///
    /// # Examples
    ///
    /// Set the title to "twilight":
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use twilight_util::builder::embed::EmbedBuilder;
    ///
    /// let embed = EmbedBuilder::new()
    ///     .title("twilight")
    ///     .url("https://github.com/twilight-rs/twilight")
    ///     .validate()?
    ///     .build();
    /// # Ok(()) }
    /// ```
    ///
    /// [`TITLE_LENGTH`]: twilight_validate::embed::TITLE_LENGTH
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.0.title = Some(title.into());

        self
    }

    /// Set the URL.
    ///
    /// # Examples
    ///
    /// Set the URL to [twilight's repository]:
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use twilight_util::builder::embed::{EmbedBuilder, EmbedFooterBuilder};
    ///
    /// let embed = EmbedBuilder::new()
    ///     .description("twilight's repository")
    ///     .url("https://github.com/twilight-rs/twilight")
    ///     .validate()?
    ///     .build();
    /// # Ok(()) }
    /// ```
    ///
    /// [twilight's repository]: https://github.com/twilight-rs/twilight
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.0.url = Some(url.into());

        self
    }
}

impl Default for EmbedBuilder {
    /// Create an embed builder with a default embed.
    ///
    /// All embeds have a "rich" type.
    fn default() -> Self {
        Self::new()
    }
}

impl From<Embed> for EmbedBuilder {
    fn from(value: Embed) -> Self {
        Self(Embed {
            kind: "rich".to_owned(),
            ..value
        })
    }
}

impl TryFrom<EmbedBuilder> for Embed {
    type Error = EmbedValidationError;

    /// Convert an embed builder into an embed, validating its contents.
    ///
    /// This is equivalent to calling [`EmbedBuilder::validate`], then
    /// [`EmbedBuilder::build`].
    fn try_from(builder: EmbedBuilder) -> Result<Self, Self::Error> {
        Ok(builder.validate()?.build())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(EmbedBuilder: Clone, Debug, Eq, PartialEq, Send, Sync);
    assert_impl_all!(Embed: TryFrom<EmbedBuilder>);

    #[test]
    fn builder() {
        let footer_image = ImageSource::url(
            "https://raw.githubusercontent.com/twilight-rs/twilight/main/logo.png",
        )
        .unwrap();
        let timestamp = Timestamp::from_secs(1_580_608_922).expect("non zero");

        let embed = EmbedBuilder::new()
            .color(0x00_43_ff)
            .description("Description")
            .timestamp(timestamp)
            .footer(EmbedFooterBuilder::new("Warn").icon_url(footer_image))
            .field(EmbedFieldBuilder::new("name", "title").inline())
            .build();

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
                    "https://raw.githubusercontent.com/twilight-rs/twilight/main/logo.png"
                        .to_string(),
                ),
                proxy_icon_url: None,
                text: "Warn".to_string(),
            }),
            image: None,
            kind: "rich".to_string(),
            provider: None,
            thumbnail: None,
            timestamp: Some(timestamp),
            title: None,
            url: None,
            video: None,
        };

        assert_eq!(embed, expected);
    }
}
