//! Utilities for parsing webhook URLs.
//!
//! The URL is typically provided by the desktop client GUI when configuring a
//! webhook integration.

use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    num::NonZero,
};
use twilight_model::id::{Id, marker::WebhookMarker};

/// Error when [parsing] a webhook URL.
///
/// [parsing]: parse
#[derive(Debug)]
pub struct WebhookParseError {
    kind: WebhookParseErrorType,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl WebhookParseError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &WebhookParseErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        self.source
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(self) -> (WebhookParseErrorType, Option<Box<dyn Error + Send + Sync>>) {
        (self.kind, self.source)
    }
}

impl Display for WebhookParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.kind {
            WebhookParseErrorType::IdInvalid => f.write_str("url path segment isn't a valid ID"),
            WebhookParseErrorType::SegmentMissing => {
                f.write_str("url is missing a required path segment")
            }
        }
    }
}

impl Error for WebhookParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn Error + 'static))
    }
}

/// Type of [`WebhookParseError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum WebhookParseErrorType {
    /// ID segment in the URL path is not an integer.
    IdInvalid,
    /// Required segment of the URL path is missing.
    SegmentMissing,
}

/// Parse the webhook ID and token from a webhook URL, if it exists in the
/// string.
///
/// # Examples
///
/// Parse a webhook URL with a token:
///
/// ```
/// use twilight_model::id::Id;
/// use twilight_util::link::webhook;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let url = "https://canary.discord.com/api/webhooks/794590023369752587/tjxHaPHLKp9aEdSwJuLeHhHHGEqIxt1aay4I67FOP9uzsYEWmj0eJmDn-2ZvCYLyOb_K";
///
/// let (id, token) = webhook::parse(url)?;
/// assert_eq!(Id::new(794590023369752587), id);
/// assert_eq!(
///     Some("tjxHaPHLKp9aEdSwJuLeHhHHGEqIxt1aay4I67FOP9uzsYEWmj0eJmDn-2ZvCYLyOb_K"),
///     token,
/// );
/// # Ok(()) }
/// ```
///
/// Parse a webhook URL without a token:
///
/// ```
/// use twilight_model::id::Id;
/// use twilight_util::link::webhook;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let url = "https://canary.discord.com/api/webhooks/794590023369752587";
///
/// let (id, token) = webhook::parse(url)?;
/// assert_eq!(Id::new(794590023369752587), id);
/// assert!(token.is_none());
/// # Ok(()) }
/// ```
///
/// # Errors
///
/// Returns [`WebhookParseErrorType::IdInvalid`] error type if the ID segment of
/// the URL is not a valid integer.
///
/// Returns [`WebhookParseErrorType::SegmentMissing`] error type if one of the
/// required segments is missing. This can be the "api" or "webhooks" standard
/// segment of the URL or the segment containing the webhook ID.
pub fn parse(url: &str) -> Result<(Id<WebhookMarker>, Option<&str>), WebhookParseError> {
    let mut segments = {
        let mut start = url.split("discord.com/api/webhooks/");
        let path = start.nth(1).ok_or(WebhookParseError {
            kind: WebhookParseErrorType::SegmentMissing,
            source: None,
        })?;

        path.split('/')
    };

    let id_segment = segments.next().ok_or(WebhookParseError {
        kind: WebhookParseErrorType::SegmentMissing,
        source: None,
    })?;

    // If we don't have this check it'll return `IdInvalid`, which isn't right.
    if id_segment.is_empty() {
        return Err(WebhookParseError {
            kind: WebhookParseErrorType::SegmentMissing,
            source: None,
        });
    }

    let id = id_segment
        .parse::<NonZero<u64>>()
        .map_err(|source| WebhookParseError {
            kind: WebhookParseErrorType::IdInvalid,
            source: Some(Box::new(source)),
        })?;
    let mut token = segments.next();

    // Don't return an empty token if the segment is empty.
    if token.is_some_and(str::is_empty) {
        token = None;
    }

    Ok((Id::from(id), token))
}

#[cfg(test)]
mod tests {
    use super::{WebhookParseError, WebhookParseErrorType};
    use static_assertions::assert_impl_all;
    use std::{error::Error, fmt::Debug};
    use twilight_model::id::Id;

    assert_impl_all!(WebhookParseErrorType: Debug, Send, Sync);
    assert_impl_all!(WebhookParseError: Debug, Error, Send, Sync);

    #[test]
    fn parse_no_token() {
        assert_eq!(
            (Id::new(123), None),
            super::parse("https://discord.com/api/webhooks/123").unwrap(),
        );
        // There's a / after the ID signifying another segment, but the token
        // ends up being None.
        assert_eq!(
            (Id::new(123), None),
            super::parse("https://discord.com/api/webhooks/123").unwrap(),
        );
        assert!(
            super::parse("https://discord.com/api/webhooks/123/")
                .unwrap()
                .1
                .is_none()
        );
    }

    #[test]
    fn parse_with_token() {
        assert_eq!(
            super::parse("https://discord.com/api/webhooks/456/token").unwrap(),
            (Id::new(456), Some("token")),
        );
        // The value of the segment(s) after the token are ignored.
        assert_eq!(
            super::parse("https://discord.com/api/webhooks/456/token/github").unwrap(),
            (Id::new(456), Some("token")),
        );
        assert_eq!(
            super::parse("https://discord.com/api/webhooks/456/token/slack").unwrap(),
            (Id::new(456), Some("token")),
        );
        assert_eq!(
            super::parse("https://discord.com/api/webhooks/456/token/randomsegment").unwrap(),
            (Id::new(456), Some("token")),
        );
        assert_eq!(
            super::parse("https://discord.com/api/webhooks/456/token/one/two/three").unwrap(),
            (Id::new(456), Some("token")),
        );
    }

    #[test]
    fn parse_invalid() {
        // Base URL is improper.
        assert!(matches!(
            super::parse("https://discord.com/foo/bar/456")
                .unwrap_err()
                .kind(),
            &WebhookParseErrorType::SegmentMissing,
        ));
        // No ID is present.
        assert!(matches!(
            super::parse("https://discord.com/api/webhooks/")
                .unwrap_err()
                .kind(),
            &WebhookParseErrorType::SegmentMissing,
        ));
        // ID segment isn't an integer.
        assert!(matches!(
            super::parse("https://discord.com/api/webhooks/notaninteger")
                .unwrap_err()
                .kind(),
            &WebhookParseErrorType::IdInvalid,
        ));
    }
}
