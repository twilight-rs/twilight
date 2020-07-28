//! Create embed footers.

use super::image_source::ImageSource;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::channel::embed::EmbedFooter;

/// Error creating an embed footer.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum EmbedFooterTextError {
    /// Text is empty.
    Empty {
        /// Provided text. Although empty, the same owned allocation is
        /// included.
        text: String,
    },
    /// Text is longer than 2048 UTF-16 code points.
    TooLong {
        /// Provided text.
        text: String,
    },
}

impl Display for EmbedFooterTextError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Empty { .. } => f.write_str("the footer text is empty"),
            Self::TooLong { .. } => f.write_str("the footer text is too long"),
        }
    }
}

impl Error for EmbedFooterTextError {}

/// Create an embed footer with a builder.
///
/// This can be passed into [`EmbedBuilder::footer`].
///
/// [`EmbedBuilder::footer`]: struct.EmbedBuilder.html#method.footer
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into an embed footer"]
pub struct EmbedFooterBuilder(EmbedFooter);

impl EmbedFooterBuilder {
    /// The maximum number of UTF-16 code points that can be in a footer's text.
    pub const TEXT_LENGTH_LIMIT: usize = 2048;

    /// Create a new default embed footer builder.
    ///
    /// Refer to [`TEXT_LENGTH_LIMIT`] for the maximum number of UTF-16 code
    /// points that can be in a footer's text.
    ///
    /// # Errors
    ///
    /// Returns [`EmbedFooterTextError::Empty`] if the provided text is
    /// empty.
    ///
    /// Returns [`EmbedFooterTextError::TooLong`] if the provided text is
    /// longer than the limit defined at [`TEXT_LENGTH_LIMIT`].
    ///
    /// [`TEXT_LENGTH_LIMIT`]: #const.TEXT_LENGTH_LIMIT
    /// [`EmbedFooterTextError::Empty`]: enum.EmbedFooterTextError.variant.Empty
    /// [`EmbedFooterTextError::TooLong`]: enum.EmbedFooterTextError.variant.TooLong
    pub fn new(text: impl Into<String>) -> Result<Self, EmbedFooterTextError> {
        Self::_new(text.into())
    }

    fn _new(text: String) -> Result<Self, EmbedFooterTextError> {
        if text.is_empty() {
            return Err(EmbedFooterTextError::Empty { text });
        }

        if text.chars().count() > Self::TEXT_LENGTH_LIMIT {
            return Err(EmbedFooterTextError::TooLong { text });
        }

        Ok(Self(EmbedFooter {
            icon_url: None,
            proxy_icon_url: None,
            text,
        }))
    }

    /// Build into an embed footer.
    #[must_use = "should be used as part of an embed builder"]
    pub fn build(self) -> EmbedFooter {
        self.0
    }

    /// Add a footer icon.
    ///
    /// # Examples
    ///
    /// Create a footer by Twilight with a URL to an image of its logo:
    ///
    /// ```rust
    /// use twilight_embed_builder::{EmbedFooterBuilder, ImageSource};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let icon_url = ImageSource::url("https://raw.githubusercontent.com/twilight-rs/twilight/trunk/logo.png")?;
    /// let footer = EmbedFooterBuilder::new("Twilight")?
    ///     .icon_url(icon_url)
    ///     .build();
    /// # Ok(()) }
    /// ```
    pub fn icon_url(mut self, image_source: ImageSource) -> Self {
        self.0.icon_url.replace(image_source.0);

        self
    }
}

impl From<EmbedFooterBuilder> for EmbedFooter {
    /// Convert an embed footer builder into an embed footer.
    ///
    /// This is equivalent to calling [`EmbedFooterBuilder::build`].
    ///
    /// [`EmbedFooterBuilder::build`]: #method.build
    fn from(builder: EmbedFooterBuilder) -> Self {
        builder.build()
    }
}
