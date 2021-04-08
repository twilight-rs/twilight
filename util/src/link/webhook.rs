//! Utilities for parsing webhook URLs.
//!
//! The URL is typically provided by the desktop client GUI when configuring a
//! webhook integration.

use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    num::ParseIntError,
};
use twilight_model::id::WebhookId;

/// Error when [parsing] a webhook URL.
///
/// [parsing]: parse
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum WebhookParseError {
    /// ID segment in the URL path is not an integer.
    IdInvalid {
        /// Reason for the error.
        source: ParseIntError,
    },
    /// Required segment of the URL path is missing.
    SegmentMissing,
}

impl Display for WebhookParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::IdInvalid { .. } => f.write_str("url path segment isn't a valid ID"),
            Self::SegmentMissing => f.write_str("url is missing a required path segment"),
        }
    }
}

impl Error for WebhookParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::IdInvalid { source } => Some(source),
            Self::SegmentMissing => None,
        }
    }
}

/// Parse the webhook ID and token from a webhook URL, if it exists in the
/// string.
///
/// # Examples
///
/// Parse a webhook URL with a token:
///
/// ```
/// use twilight_model::id::WebhookId;
/// use twilight_util::link::webhook;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let url = "https://canary.discord.com/api/webhooks/794590023369752587/tjxHaPHLKp9aEdSwJuLeHhHHGEqIxt1aay4I67FOP9uzsYEWmj0eJmDn-2ZvCYLyOb_K";
///
/// let (id, token) = webhook::parse(url)?;
/// assert_eq!(WebhookId(794590023369752587), id);
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
/// use twilight_model::id::WebhookId;
/// use twilight_util::link::webhook;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let url = "https://canary.discord.com/api/webhooks/794590023369752587";
///
/// let (id, token) = webhook::parse(url)?;
/// assert_eq!(WebhookId(794590023369752587), id);
/// assert!(token.is_none());
/// # Ok(()) }
/// ```
///
/// # Errors
///
/// Returns [`WebhookParseError::IdInvalid`] if the ID segment of the URL is not
/// a valid integer.
///
/// Returns [`WebhookParseError::SegmentMissing`] if one of the required
/// segments is missing. This can be the "api" or "webhooks" standard segment
/// of the URL or the segment containing the webhook ID.
pub fn parse(url: &str) -> Result<(WebhookId, Option<&str>), WebhookParseError> {
    let mut segments = {
        let mut start = url.split("discord.com/api/webhooks/");
        let path = start.nth(1).ok_or(WebhookParseError::SegmentMissing)?;

        path.split('/')
    };

    let id_segment = segments.next().ok_or(WebhookParseError::SegmentMissing)?;

    // If we don't have this check it'll return `IdInvalid`, which isn't right.
    if id_segment.is_empty() {
        return Err(WebhookParseError::SegmentMissing);
    }

    let id = id_segment
        .parse()
        .map_err(|source| WebhookParseError::IdInvalid { source })?;
    let mut token = segments.next();

    // Don't return an empty token if the segment is empty.
    if token.map(str::is_empty).unwrap_or_default() {
        token = None;
    }

    Ok((WebhookId(id), token))
}

#[cfg(test)]
mod tests {
    use super::{WebhookId, WebhookParseError};
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{
        error::Error,
        fmt::{Debug, Display},
    };

    assert_fields!(WebhookParseError::IdInvalid: source);
    assert_impl_all!(
        WebhookParseError: Clone,
        Debug,
        Display,
        Eq,
        Error,
        PartialEq
    );

    #[test]
    fn test_parse_no_token() {
        assert_eq!(
            (WebhookId(123), None),
            super::parse("https://discord.com/api/webhooks/123").unwrap(),
        );
        // There's a / after the ID signifying another segment, but the token
        // ends up being None.
        assert_eq!(
            (WebhookId(123), None),
            super::parse("https://discord.com/api/webhooks/123").unwrap(),
        );
        assert!(super::parse("https://discord.com/api/webhooks/123/")
            .unwrap()
            .1
            .is_none());
    }

    #[test]
    fn test_parse_with_token() {
        assert_eq!(
            super::parse("https://discord.com/api/webhooks/456/token").unwrap(),
            (WebhookId(456), Some("token")),
        );
        // The value of the segment(s) after the token are ignored.
        assert_eq!(
            super::parse("https://discord.com/api/webhooks/456/token/github").unwrap(),
            (WebhookId(456), Some("token")),
        );
        assert_eq!(
            super::parse("https://discord.com/api/webhooks/456/token/slack").unwrap(),
            (WebhookId(456), Some("token")),
        );
        assert_eq!(
            super::parse("https://discord.com/api/webhooks/456/token/randomsegment").unwrap(),
            (WebhookId(456), Some("token")),
        );
        assert_eq!(
            super::parse("https://discord.com/api/webhooks/456/token/one/two/three").unwrap(),
            (WebhookId(456), Some("token")),
        );
    }

    #[test]
    fn test_parse_invalid() {
        // Base URL is improper.
        assert_eq!(
            WebhookParseError::SegmentMissing,
            super::parse("https://discord.com/foo/bar/456").unwrap_err(),
        );
        // No ID is present.
        assert_eq!(
            WebhookParseError::SegmentMissing,
            super::parse("https://discord.com/api/webhooks/").unwrap_err(),
        );
        // ID segment isn't an integer.
        assert!(matches!(
            super::parse("https://discord.com/api/webhooks/notaninteger").unwrap_err(),
            WebhookParseError::IdInvalid { .. },
        ));
    }
}
