//! Create embed authors.

use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::channel::embed::EmbedAuthor;

/// Error setting an embed author's name.
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
/// [`EmbedBuilder::author`]: struct.EmbedBuilder.html#method.author
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into an embed author"]
pub struct EmbedAuthorBuilder(EmbedAuthor);

impl EmbedAuthorBuilder {
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
    ///
    /// Either the URL to an image or an `attachment://` path.
    pub fn icon_url(self, icon_url: impl Into<String>) -> Self {
        self._icon_url(icon_url.into())
    }

    fn _icon_url(mut self, icon_url: String) -> Self {
        self.0.icon_url.replace(icon_url);

        self
    }

    /// The author's name.
    ///
    /// Limited to 256 UTF-16 code points.
    ///
    /// # Errors
    ///
    /// Returns [`EmbedAuthorNameError::Empty`] if the provided name is empty.
    ///
    /// Returns [`EmbedAuthorNameError::TooLong`] if the provided name is longer
    /// than 256 UTF-16 code points.
    ///
    /// [`EmbedAuthorNameError::Empty`]: enum.EmbedAuthorNameError.html#variant.Empty
    /// [`EmbedAuthorNameError::TooLong`]: enum.EmbedAuthorNameError.html#variant.TooLong
    pub fn name(self, name: impl Into<String>) -> Result<Self, EmbedAuthorNameError> {
        self._name(name.into())
    }

    fn _name(mut self, name: String) -> Result<Self, EmbedAuthorNameError> {
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
