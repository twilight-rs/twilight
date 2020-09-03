//! Create embed authors.

use super::image_source::ImageSource;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::channel::embed::EmbedAuthor;

/// Error setting an embed author's name.
///
/// This is returned from [`EmbedAuthorBuilder::name`].
///
/// [`EmbedAuthorBuilder::name`]: struct.EmbedAuthorBuilder.html#method.name
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum EmbedAuthorNameError {
    /// Name is empty.
    Empty {
        /// Provided name. Although empty, the same owned allocation is
        /// included.
        name: String,
    },
    /// Name is longer than 256 UTF-16 code points.
    TooLong {
        /// Provided name.
        name: String,
    },
}

impl Display for EmbedAuthorNameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Empty { .. } => f.write_str("the author name is empty"),
            Self::TooLong { .. } => f.write_str("the author name is too long"),
        }
    }
}

impl Error for EmbedAuthorNameError {}

/// Create an embed author with a builder.
///
/// This can be passed into [`EmbedBuilder::author`].
///
/// [`EmbedBuilder::author`]: ../builder/struct.EmbedBuilder.html#method.author
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into an embed author"]
pub struct EmbedAuthorBuilder(EmbedAuthor);

impl EmbedAuthorBuilder {
    /// The maximum number of UTF-16 code points that can be in an author name.
    ///
    /// This is used by [`name`].
    ///
    /// [`name`]: #method.name
    pub const NAME_LENGTH_LIMIT: usize = 256;

    /// Create a new default embed author builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Build into an embed author.
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
    /// Refer to [`NAME_LENGTH_LIMIT`] for the maximum number of UTF-16
    /// code points that can be in a description.
    ///
    /// # Errors
    ///
    /// Returns [`EmbedAuthorNameError::Empty`] if the provided name is empty.
    ///
    /// Returns [`EmbedAuthorNameError::TooLong`] if the provided name is longer
    /// than the maximum number of code points.
    ///
    /// [`NAME_LENGTH_LIMIT`]: #const.NAME_LENGTH_LIMIT
    /// [`EmbedAuthorNameError::Empty`]: enum.EmbedAuthorNameError.html#variant.Empty
    /// [`EmbedAuthorNameError::TooLong`]: enum.EmbedAuthorNameError.html#variant.TooLong
    pub fn name(self, name: impl Into<String>) -> Result<Self, EmbedAuthorNameError> {
        self._name(name.into())
    }

    fn _name(mut self, name: String) -> Result<Self, EmbedAuthorNameError> {
        if name.is_empty() {
            return Err(EmbedAuthorNameError::Empty { name });
        }

        if name.chars().count() > Self::NAME_LENGTH_LIMIT {
            return Err(EmbedAuthorNameError::TooLong { name });
        }

        self.0.name.replace(name);

        Ok(self)
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

impl Default for EmbedAuthorBuilder {
    fn default() -> Self {
        Self(EmbedAuthor {
            icon_url: None,
            name: None,
            proxy_icon_url: None,
            url: None,
        })
    }
}

impl From<EmbedAuthorBuilder> for EmbedAuthor {
    /// Convert an embed author builder into an embed author.
    ///
    /// This is equivalent to calling [`EmbedAuthorBuilder::build`].
    ///
    /// [`EmbedAuthorBuilder::build`]: #method.build
    fn from(builder: EmbedAuthorBuilder) -> Self {
        builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::{EmbedAuthorBuilder, EmbedAuthorNameError};
    use crate::ImageSource;
    use static_assertions::{assert_fields, assert_impl_all, const_assert};
    use std::{error::Error, fmt::Debug};
    use twilight_model::channel::embed::EmbedAuthor;

    assert_impl_all!(
        EmbedAuthorNameError: Clone,
        Debug,
        Error,
        Eq,
        PartialEq,
        Send,
        Sync
    );
    assert_fields!(EmbedAuthorNameError::Empty: name);
    assert_fields!(EmbedAuthorNameError::TooLong: name);
    assert_impl_all!(
        EmbedAuthorBuilder: Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        Send,
        Sync
    );
    const_assert!(EmbedAuthorBuilder::NAME_LENGTH_LIMIT == 256);
    assert_impl_all!(EmbedAuthor: From<EmbedAuthorBuilder>);

    #[test]
    fn test_defaults() {
        let expected = EmbedAuthor {
            icon_url: None,
            name: None,
            proxy_icon_url: None,
            url: None,
        };

        assert_eq!(expected, EmbedAuthorBuilder::new().0);
        assert_eq!(EmbedAuthorBuilder::new().0, EmbedAuthorBuilder::default().0);
    }

    #[test]
    fn test_name_empty() {
        assert!(matches!(
            EmbedAuthorBuilder::new().name(""),
            Err(EmbedAuthorNameError::Empty { .. })
        ));
    }

    #[test]
    fn test_name_too_long() {
        assert!(EmbedAuthorBuilder::new().name("a".repeat(256)).is_ok());
        assert!(matches!(
            EmbedAuthorBuilder::new().name("a".repeat(257)),
            Err(EmbedAuthorNameError::TooLong { .. })
        ));
    }

    #[test]
    fn test_builder() -> Result<(), Box<dyn Error>> {
        let expected = EmbedAuthor {
            icon_url: Some("https://example.com/1.png".to_owned()),
            name: Some("an author".to_owned()),
            proxy_icon_url: None,
            url: Some("https://example.com".to_owned()),
        };

        let source = ImageSource::url("https://example.com/1.png")?;
        let actual = EmbedAuthorBuilder::new()
            .icon_url(source)
            .name("an author")?
            .url("https://example.com")
            .build();

        assert_eq!(actual, expected);

        Ok(())
    }
}
