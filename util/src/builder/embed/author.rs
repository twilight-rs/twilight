//! Create embed authors.

use super::image_source::ImageSource;
use twilight_model::channel::embed::EmbedAuthor;

/// Create an embed author with a builder.
///
/// This can be passed into [`EmbedBuilder::author`].
///
/// [`EmbedBuilder::author`]: crate::builder::embed::EmbedBuilder::author
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into an embed author"]
pub struct EmbedAuthorBuilder(EmbedAuthor);

impl EmbedAuthorBuilder {
    /// Create a new embed author builder.
    pub const fn new(name: String) -> Self {
        Self(EmbedAuthor {
            icon_url: None,
            name,
            proxy_icon_url: None,
            url: None,
        })
    }

    /// Build into an embed author.
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "should be used as part of an embed builder"]
    pub fn build(self) -> EmbedAuthor {
        self.0
    }

    /// Add an author icon.
    pub fn icon_url(mut self, image_source: ImageSource) -> Self {
        self.0.icon_url.replace(image_source.0);

        self
    }

    /// The author's name.
    ///
    /// Refer to [`AUTHOR_NAME_LENGTH`] for the maximum number of UTF-16 code
    /// points that can be in an author name.
    ///
    /// [`AUTHOR_NAME_LENGTH`]: twilight_validate::embed::AUTHOR_NAME_LENGTH
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.0.name = name.into();

        self
    }

    /// The author's url.
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.0.url = Some(url.into());

        self
    }
}

impl From<EmbedAuthorBuilder> for EmbedAuthor {
    /// Convert an embed author builder into an embed author.
    ///
    /// This is equivalent to calling [`EmbedAuthorBuilder::build`].
    fn from(builder: EmbedAuthorBuilder) -> Self {
        builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;
    use twilight_model::channel::embed::EmbedAuthor;

    assert_impl_all!(EmbedAuthorBuilder: Clone, Debug, Eq, PartialEq, Send, Sync);
    assert_impl_all!(EmbedAuthor: From<EmbedAuthorBuilder>);

    #[test]
    fn test_builder() {
        let expected = EmbedAuthor {
            icon_url: Some("https://example.com/1.png".to_owned()),
            name: "an author".to_owned(),
            proxy_icon_url: None,
            url: Some("https://example.com".to_owned()),
        };

        let source = ImageSource::url("https://example.com/1.png").unwrap();
        let actual = EmbedAuthorBuilder::new("an author".to_owned())
            .icon_url(source)
            .url("https://example.com")
            .build();

        assert_eq!(actual, expected);
    }
}
