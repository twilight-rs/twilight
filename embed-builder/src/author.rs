//! Create embed authors.

use super::image_source::ImageSource;
use twilight_model::channel::embed::EmbedAuthor;

/// Create an embed author with a builder.
///
/// This can be passed into [`EmbedBuilder::author`].
///
/// [`EmbedBuilder::author`]: crate::EmbedBuilder::author
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
    /// Refer to [`EmbedBuilder::AUTHOR_NAME_LENGTH_LIMIT`] for the maximum
    /// number of UTF-16 code points that can be in an author name.
    ///
    /// [`EmbedBuilder::AUTHOR_NAME_LENGTH_LIMIT`]: crate::EmbedBuilder::AUTHOR_NAME_LENGTH_LIMIT
    pub fn name(self, name: impl Into<String>) -> Self {
        self._name(name.into())
    }

    #[allow(clippy::missing_const_for_fn)]
    fn _name(mut self, name: String) -> Self {
        self.0.name = name;

        self
    }

    /// The author's url.
    pub fn url(self, url: impl Into<String>) -> Self {
        self._url(url.into())
    }

    fn _url(mut self, url: String) -> Self {
        self.0.url.replace(url);

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
    use super::EmbedAuthorBuilder;
    use crate::{EmbedBuilder, EmbedErrorType, ImageSource};
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;
    use twilight_model::channel::embed::EmbedAuthor;

    assert_impl_all!(EmbedAuthorBuilder: Clone, Debug, Eq, PartialEq, Send, Sync);
    assert_impl_all!(EmbedAuthor: From<EmbedAuthorBuilder>);

    #[test]
    fn test_name_empty() {
        let builder = EmbedBuilder::new().author(EmbedAuthorBuilder::new("".to_owned()));

        assert!(matches!(
            builder.build().unwrap_err().kind(),
            EmbedErrorType::AuthorNameEmpty { .. }
        ));
    }

    #[test]
    fn test_name_too_long() {
        let builder = EmbedBuilder::new().author(EmbedAuthorBuilder::new("a".repeat(256)));
        assert!(builder.build().is_ok());

        let builder = EmbedBuilder::new().author(EmbedAuthorBuilder::new("a".repeat(257)));
        assert!(matches!(
            builder.build().unwrap_err().kind(),
            EmbedErrorType::AuthorNameTooLong { .. }
        ));
    }

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
