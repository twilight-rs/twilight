//! Sources to image URLs and attachments.

use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

/// Error creating an embed field.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ImageSourceAttachmentError {
    /// An extension is present in the provided filename but it is empty.
    ExtensionEmpty,
    /// An extension is missing in the provided filename.
    ExtensionMissing,
}

impl Display for ImageSourceAttachmentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::ExtensionEmpty { .. } => f.write_str("the extension is empty"),
            Self::ExtensionMissing { .. } => f.write_str("the extension is missing"),
        }
    }
}

impl Error for ImageSourceAttachmentError {}

/// Error creating an embed field.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ImageSourceUrlError {
    /// The Protocol of the URL is unsupported by the Discord REST API.
    ///
    /// Refer to [`ImageSource::url`] for a list of protocols that are acceptable.
    ProtocolUnsupported {
        /// Provided URL.
        url: String,
    },
}

impl Display for ImageSourceUrlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::ProtocolUnsupported { .. } => {
                f.write_str("the provided URL's protocol is unsupported by Discord")
            }
        }
    }
}

impl Error for ImageSourceUrlError {}

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
    /// Returns [`ImageSourceAttachmentError::ExtensionEmpty`] if an extension exists
    /// but is empty.
    ///
    /// Returns [`ImageSourceAttachmentError::ExtensionMissing`] if an extension is
    /// missing.
    ///
    /// [`ImageSourceAttachmentError::ExtensionEmpty`]: enum.ImageSourceAttachmentError.html#variant.ExtensionEmpty
    /// [`ImageSourceAttachmentError::ExtensionMissing`]: enum.ImageSourceAttachmentError.html#variant.ExtensionMissing
    pub fn attachment(filename: impl AsRef<str>) -> Result<Self, ImageSourceAttachmentError> {
        Self::_attachment(filename.as_ref())
    }

    fn _attachment(filename: &str) -> Result<Self, ImageSourceAttachmentError> {
        let dot = filename
            .rfind('.')
            .ok_or(ImageSourceAttachmentError::ExtensionMissing)?
            + 1;

        if filename
            .get(dot..)
            .ok_or(ImageSourceAttachmentError::ExtensionMissing)?
            .is_empty()
        {
            return Err(ImageSourceAttachmentError::ExtensionEmpty);
        }

        Ok(Self(format!("attachment://{}", filename)))
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
    /// Returns [`ImageSourceUrlError::ProtocolUnsupported`] if the URL's protocol is unsupported.
    ///
    /// [`ImageSourceUrlError::ProtocolUnsupported`]: enum.ImageSourceUrlError.html#variant.ProtocolUnsupported
    pub fn url(url: impl Into<String>) -> Result<Self, ImageSourceUrlError> {
        Self::_url(url.into())
    }

    fn _url(url: String) -> Result<Self, ImageSourceUrlError> {
        if !url.starts_with("https:") && !url.starts_with("http:") {
            return Err(ImageSourceUrlError::ProtocolUnsupported { url });
        }

        Ok(Self(url))
    }
}

#[cfg(test)]
mod tests {
    use super::{ImageSource, ImageSourceAttachmentError, ImageSourceUrlError};
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{error::Error, fmt::Debug};

    assert_impl_all!(
        ImageSourceAttachmentError: Clone,
        Debug,
        Error,
        Eq,
        PartialEq,
        Send,
        Sync
    );
    assert_impl_all!(
        ImageSourceUrlError: Clone,
        Debug,
        Error,
        Eq,
        PartialEq,
        Send,
        Sync
    );
    assert_fields!(ImageSourceUrlError::ProtocolUnsupported: url);
    assert_impl_all!(ImageSource: Clone, Debug, Eq, PartialEq, Send, Sync);

    #[test]
    fn test_attachment() -> Result<(), Box<dyn Error>> {
        assert!(matches!(
            ImageSource::attachment("abc").unwrap_err(),
            ImageSourceAttachmentError::ExtensionMissing
        ));
        assert!(matches!(
            ImageSource::attachment("abc.").unwrap_err(),
            ImageSourceAttachmentError::ExtensionEmpty
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
            ImageSource::url("ftp://example.com/foo").unwrap_err(),
            ImageSourceUrlError::ProtocolUnsupported { url }
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
