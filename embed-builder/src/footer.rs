//! Create embed footers.

use super::image_source::ImageSource;
use twilight_model::channel::embed::EmbedFooter;

/// Create an embed footer with a builder.
///
/// This can be passed into [`EmbedBuilder::footer`].
///
/// [`EmbedBuilder::footer`]: crate::EmbedBuilder::footer
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into an embed footer"]
pub struct EmbedFooterBuilder(EmbedFooter);

impl EmbedFooterBuilder {
    /// Create a new default embed footer builder.
    ///
    /// Refer to [`EmbedBuilder::FOOTER_TEXT_LENGTH_LIMIT`] for the maximum
    /// number of UTF-16 code points that can be in a footer's text.
    ///
    /// [`EmbedBuilder::FOOTER_TEXT_LENGTH_LIMIT`]: crate::EmbedBuilder::FOOTER_TEXT_LENGTH_LIMIT
    pub fn new(text: impl Into<String>) -> Self {
        Self::_new(text.into())
    }

    const fn _new(text: String) -> Self {
        Self(EmbedFooter {
            icon_url: None,
            proxy_icon_url: None,
            text,
        })
    }

    /// Build into an embed footer.
    #[allow(clippy::missing_const_for_fn)]
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
    /// ```
    /// use twilight_embed_builder::{EmbedFooterBuilder, ImageSource};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let icon_url = ImageSource::url("https://raw.githubusercontent.com/twilight-rs/twilight/main/logo.png")?;
    /// let footer = EmbedFooterBuilder::new("Twilight")
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
    fn from(builder: EmbedFooterBuilder) -> Self {
        builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::EmbedFooterBuilder;
    use crate::{EmbedBuilder, EmbedErrorType, ImageSource};
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;
    use twilight_model::channel::embed::EmbedFooter;

    assert_impl_all!(EmbedFooterBuilder: Clone, Debug, Eq, PartialEq, Send, Sync);
    assert_impl_all!(EmbedFooter: From<EmbedFooterBuilder>);

    #[test]
    fn text() {
        assert!(matches!(
            EmbedBuilder::new().footer(EmbedFooterBuilder::new("")).build().unwrap_err().kind(),
            EmbedErrorType::FooterTextEmpty { text }
            if text.is_empty()
        ));
        let too_long_len = EmbedBuilder::FOOTER_TEXT_LENGTH_LIMIT + 1;
        assert!(matches!(
            EmbedBuilder::new().footer(EmbedFooterBuilder::new("a".repeat(too_long_len))).build().unwrap_err().kind(),
            EmbedErrorType::FooterTextTooLong { text }
            if text.len() == too_long_len
        ));

        let expected = EmbedFooter {
            icon_url: None,
            proxy_icon_url: None,
            text: "a footer".to_owned(),
        };
        let actual = EmbedFooterBuilder::new("a footer").build();
        assert_eq!(actual, expected);
    }

    #[test]
    fn builder() {
        let expected = EmbedFooter {
            icon_url: Some("https://example.com/1.png".to_owned()),
            proxy_icon_url: None,
            text: "a footer".to_owned(),
        };
        let image = ImageSource::url("https://example.com/1.png").unwrap();
        let actual = EmbedFooterBuilder::new("a footer").icon_url(image).build();
        assert_eq!(actual, expected);
    }
}
