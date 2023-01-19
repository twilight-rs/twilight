//! Create embed footers.

use super::ImageSource;
use twilight_model::channel::message::embed::EmbedFooter;

/// Create an embed footer with a builder.
///
/// This can be passed into [`EmbedBuilder::footer`].
///
/// [`EmbedBuilder::footer`]: crate::builder::embed::EmbedBuilder::footer
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into an embed footer"]
pub struct EmbedFooterBuilder(EmbedFooter);

impl EmbedFooterBuilder {
    /// Create a new embed footer builder.
    ///
    /// Refer to [`FOOTER_TEXT_LENGTH`] for the maximum number of UTF-16 code
    /// points that can be in a footer's text.
    ///
    /// [`FOOTER_TEXT_LENGTH`]: twilight_validate::embed::FOOTER_TEXT_LENGTH
    pub fn new(text: impl Into<String>) -> Self {
        Self(EmbedFooter {
            icon_url: None,
            proxy_icon_url: None,
            text: text.into(),
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
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use twilight_util::builder::embed::{EmbedFooterBuilder, ImageSource};
    ///
    /// let icon_url =
    ///     ImageSource::url("https://raw.githubusercontent.com/twilight-rs/twilight/main/logo.png")?;
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
    use super::*;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(EmbedFooterBuilder: Clone, Debug, Eq, PartialEq, Send, Sync);
    assert_impl_all!(EmbedFooter: From<EmbedFooterBuilder>);

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
