//! Sources to image URLs and attachments.

use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

/// Error creating an embed field.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct ImageSourceAttachmentError {
    kind: ImageSourceAttachmentErrorType,
}

impl ImageSourceAttachmentError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &ImageSourceAttachmentErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[allow(clippy::unused_self)]
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        None
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(
        self,
    ) -> (
        ImageSourceAttachmentErrorType,
        Option<Box<dyn Error + Send + Sync>>,
    ) {
        (self.kind, None)
    }
}

impl Display for ImageSourceAttachmentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            ImageSourceAttachmentErrorType::ExtensionEmpty { .. } => {
                f.write_str("the extension is empty")
            }
            ImageSourceAttachmentErrorType::ExtensionMissing { .. } => {
                f.write_str("the extension is missing")
            }
        }
    }
}

impl Error for ImageSourceAttachmentError {}

/// Type of [`ImageSourceAttachmentError`] that occurred.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
#[non_exhaustive]
pub enum ImageSourceAttachmentErrorType {
    /// An extension is present in the provided filename but it is empty.
    ExtensionEmpty,
    /// An extension is missing in the provided filename.
    ExtensionMissing,
}

/// Error creating an embed field.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct ImageSourceUrlError {
    kind: ImageSourceUrlErrorType,
}

impl ImageSourceUrlError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &ImageSourceUrlErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[allow(clippy::unused_self)]
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        None
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(
        self,
    ) -> (
        ImageSourceUrlErrorType,
        Option<Box<dyn Error + Send + Sync>>,
    ) {
        (self.kind, None)
    }
}

impl Display for ImageSourceUrlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            ImageSourceUrlErrorType::ProtocolUnsupported { .. } => {
                f.write_str("the provided URL's protocol is unsupported by Discord")
            }
        }
    }
}

impl Error for ImageSourceUrlError {}

/// Type of [`ImageSourceUrlError`] that occurred.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
#[non_exhaustive]
pub enum ImageSourceUrlErrorType {
    /// The Protocol of the URL is unsupported by the Discord REST API.
    ///
    /// Refer to [`ImageSource::url`] for a list of protocols that are acceptable.
    ProtocolUnsupported {
        /// Provided URL.
        url: String,
    },
}

/// Image sourcing for embed images.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct ImageSource(pub(crate) String);

impl ImageSource {
    /// Create an attachment image source.
    ///
    /// This will automatically prepend `attachment://` to the source.
    ///
    /// # Errors
    ///
    /// Returns an [`ImageSourceAttachmentErrorType::ExtensionEmpty`] if an
    /// extension exists but is empty.
    ///
    /// Returns an [`ImageSourceAttachmentErrorType::ExtensionMissing`] if an
    /// extension is missing.
    pub fn attachment(filename: impl AsRef<str>) -> Result<Self, ImageSourceAttachmentError> {
        Self::_attachment(filename.as_ref())
    }

    fn _attachment(filename: &str) -> Result<Self, ImageSourceAttachmentError> {
        let dot = filename.rfind('.').ok_or(ImageSourceAttachmentError {
            kind: ImageSourceAttachmentErrorType::ExtensionMissing,
        })? + 1;

        if filename
            .get(dot..)
            .ok_or(ImageSourceAttachmentError {
                kind: ImageSourceAttachmentErrorType::ExtensionMissing,
            })?
            .is_empty()
        {
            return Err(ImageSourceAttachmentError {
                kind: ImageSourceAttachmentErrorType::ExtensionEmpty,
            });
        }

        Ok(Self(format!("attachment://{filename}")))
    }

    /// Create a URL image source.
    ///
    /// The following URL protocols are acceptable:
    ///
    /// - https
    /// - http
    ///
    /// # Errors
    ///
    /// Returns an [`ImageSourceUrlErrorType::ProtocolUnsupported`] error type
    /// if the URL's protocol is unsupported.
    pub fn url(url: impl Into<String>) -> Result<Self, ImageSourceUrlError> {
        Self::_url(url.into())
    }

    fn _url(url: String) -> Result<Self, ImageSourceUrlError> {
        if !url.starts_with("https:") && !url.starts_with("http:") {
            return Err(ImageSourceUrlError {
                kind: ImageSourceUrlErrorType::ProtocolUnsupported { url },
            });
        }

        Ok(Self(url))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ImageSource, ImageSourceAttachmentError, ImageSourceAttachmentErrorType,
        ImageSourceUrlError, ImageSourceUrlErrorType,
    };
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{error::Error, fmt::Debug};

    assert_impl_all!(ImageSourceAttachmentErrorType: Debug, Send, Sync);
    assert_impl_all!(ImageSourceAttachmentError: Error, Send, Sync);
    assert_impl_all!(ImageSourceUrlErrorType: Debug, Send, Sync);
    assert_impl_all!(ImageSourceUrlError: Error, Send, Sync);
    assert_fields!(ImageSourceUrlErrorType::ProtocolUnsupported: url);
    assert_impl_all!(ImageSource: Clone, Debug, Eq, PartialEq, Send, Sync);

    #[test]
    fn test_attachment() -> Result<(), Box<dyn Error>> {
        assert!(matches!(
            ImageSource::attachment("abc").unwrap_err().kind(),
            ImageSourceAttachmentErrorType::ExtensionMissing
        ));
        assert!(matches!(
            ImageSource::attachment("abc.").unwrap_err().kind(),
            ImageSourceAttachmentErrorType::ExtensionEmpty
        ));
        assert_eq!(
            ImageSource::attachment("abc.png")?,
            ImageSource("attachment://abc.png".to_owned()),
        );

        Ok(())
    }

    #[test]
    fn test_url() -> Result<(), Box<dyn Error>> {
        assert!(matches!(
            ImageSource::url("ftp://example.com/foo").unwrap_err().kind(),
            ImageSourceUrlErrorType::ProtocolUnsupported { url }
            if url == "ftp://example.com/foo"
        ));
        assert_eq!(
            ImageSource::url("https://example.com")?,
            ImageSource("https://example.com".to_owned()),
        );
        assert_eq!(
            ImageSource::url("http://example.com")?,
            ImageSource("http://example.com".to_owned()),
        );

        Ok(())
    }
}
