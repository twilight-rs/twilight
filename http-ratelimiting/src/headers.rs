//! Parse typed headers from a response.
//!
//! Parsing response headers is necessary for the [`Ratelimiter`] to properly
//! function.
//!
//! [`Ratelimiter`]: super::Ratelimiter

use std::{
    error::Error,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    str::{self, FromStr, Utf8Error},
};

/// Iterator of header name-value pairs failed to be parsed.
#[derive(Debug)]
pub struct HeaderParsingError {
    /// Detailed reason why the headers failed to be parsed.
    pub(super) kind: HeaderParsingErrorType,
    /// Original error leading up to this one.
    pub(super) source: Option<Box<dyn Error + Send + Sync>>,
}

impl HeaderParsingError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &HeaderParsingErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        self.source
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(self) -> (HeaderParsingErrorType, Option<Box<dyn Error + Send + Sync>>) {
        (self.kind, self.source)
    }

    /// Create a new error because a header is missing in the response.
    pub(super) fn missing(name: HeaderName) -> Self {
        Self {
            kind: HeaderParsingErrorType::Missing { name },
            source: None,
        }
    }

    /// Create a new error because a header is not valid UTF-8.
    pub(super) fn not_utf8(name: HeaderName, value: Vec<u8>, source: Utf8Error) -> Self {
        Self {
            kind: HeaderParsingErrorType::NotUtf8 { name, value },
            source: Some(Box::new(source)),
        }
    }
}

impl Display for HeaderParsingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            HeaderParsingErrorType::Missing { name } => {
                f.write_str("at least one header, '")?;
                f.write_str(name.name())?;

                f.write_str("', is missing")
            }
            HeaderParsingErrorType::NotUtf8 { name, value } => {
                f.write_str("header '")?;
                f.write_str(name.name())?;
                f.write_str("' contains invalid UTF-16: ")?;

                Debug::fmt(value, f)
            }
            HeaderParsingErrorType::Parsing { kind, name, value } => {
                f.write_str("header '")?;
                f.write_str(name.name())?;
                f.write_str("' can not be parsed as a ")?;
                f.write_str(kind.name())?;
                f.write_str(": '")?;
                f.write_str(value)?;

                f.write_str("'")
            }
        }
    }
}

impl Error for HeaderParsingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn Error + 'static))
    }
}

/// Type of [`HeaderParsingError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum HeaderParsingErrorType {
    /// Expected header is missing.
    Missing {
        /// Name of the header that should be present in the list.
        name: HeaderName,
    },
    /// Header value is not UTF-8 valid.
    NotUtf8 {
        /// Name of the header.
        name: HeaderName,
        /// Value of the header.
        value: Vec<u8>,
    },
    /// Header value is not of the expected type.
    Parsing {
        /// Type of header value expected.
        kind: HeaderType,
        /// Name of the header.
        name: HeaderName,
        /// Value of the header.
        value: String,
    },
}

/// Typed name of a header.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum HeaderName {
    /// Information about the ratelimit bucket.
    Bucket,
    /// Global header.
    Global,
    /// Maximum requests allotted in the bucket.
    Limit,
    /// Remaining requested allotted.
    Remaining,
    /// How long until the bucket is reset.
    ResetAfter,
    /// When the bucket resets.
    Reset,
    /// How long until a request can be tried again.
    RetryAfter,
    /// Scope of a ratelimit.
    Scope,
}

impl HeaderName {
    /// Lowercased name for the bucket header.
    pub const BUCKET: &'static str = "x-ratelimit-bucket";

    /// Lowercased name for the global header.
    pub const GLOBAL: &'static str = "x-ratelimit-global";

    /// Lowercased name for the limit header.
    pub const LIMIT: &'static str = "x-ratelimit-limit";

    /// Lowercased name for the remaining header.
    pub const REMAINING: &'static str = "x-ratelimit-remaining";

    /// Lowercased name for the reset-after header.
    pub const RESET_AFTER: &'static str = "x-ratelimit-reset-after";

    /// Lowercased name for the reset header.
    pub const RESET: &'static str = "x-ratelimit-reset";

    /// Lowercased name for the retry-after header.
    // It's correct for this to not have the `x-ratelimit-` prefix.
    pub const RETRY_AFTER: &'static str = "retry-after";

    /// Lowercased name for the scope header.
    pub const SCOPE: &'static str = "x-ratelimit-scope";

    /// Lowercased name of the header.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Bucket => Self::BUCKET,
            Self::Global => Self::GLOBAL,
            Self::Limit => Self::LIMIT,
            Self::Remaining => Self::REMAINING,
            Self::ResetAfter => Self::RESET_AFTER,
            Self::Reset => Self::RESET,
            Self::RetryAfter => Self::RETRY_AFTER,
            Self::Scope => Self::SCOPE,
        }
    }
}

impl Display for HeaderName {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.name())
    }
}

/// Expected type of a header value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum HeaderType {
    /// Type of header value is a bool.
    Bool,
    /// Type of header value is a float.
    Float,
    /// Type of header value is an integer.
    Integer,
    /// Type of header value is a string.
    String,
}

impl HeaderType {
    /// Name of the type of header.
    const fn name(self) -> &'static str {
        match self {
            Self::Bool => "bool",
            Self::Float => "float",
            Self::Integer => "integer",
            Self::String => "string",
        }
    }
}

impl Display for HeaderType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.name())
    }
}

/// Ratelimit for all buckets encountered.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct GlobalLimited {
    /// Number of seconds until the global ratelimit bucket is reset.
    retry_after: u64,
    /// Scope of the ratelimit.
    scope: Option<RatelimitScope>,
}

impl GlobalLimited {
    /// Number of seconds before retrying.
    #[must_use]
    pub const fn retry_after(&self) -> u64 {
        self.retry_after
    }

    /// Scope of the ratelimit.
    ///
    /// This should always be present and should always be
    /// [`RatelimitScope::Global`].
    #[must_use]
    pub const fn scope(&self) -> Option<RatelimitScope> {
        self.scope
    }
}

/// Information about the ratelimit bucket is available.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Present {
    /// Hashed bucket ID, if any.
    bucket: Option<String>,
    /// Total number of tickets allocated to a bucket.
    limit: u64,
    /// Remaining number of tickets.
    remaining: u64,
    /// Number of seconds until the bucket resets.
    reset_after: u64,
    /// When the bucket resets as a Unix timestamp in milliseconds.
    reset: u64,
    /// Scope of a ratelimit when one occurs.
    scope: Option<RatelimitScope>,
}

impl Present {
    /// Immutable reference to the bucket.
    #[must_use]
    pub fn bucket(&self) -> Option<&str> {
        self.bucket.as_deref()
    }

    /// Consume the present ratelimit headers, returning the owned bucket if
    /// available.
    #[allow(clippy::missing_const_for_fn)]
    #[must_use]
    pub fn into_bucket(self) -> Option<String> {
        self.bucket
    }

    /// Total number of tickets allocated to the bucket.
    #[must_use]
    pub const fn limit(&self) -> u64 {
        self.limit
    }

    /// Remaining number of tickets.
    #[must_use]
    pub const fn remaining(&self) -> u64 {
        self.remaining
    }

    /// Number of seconds until the bucket resets.
    #[must_use]
    pub const fn reset_after(&self) -> u64 {
        self.reset_after
    }

    /// When the bucket resets as a Unix timestamp in milliseconds.
    #[must_use]
    pub const fn reset(&self) -> u64 {
        self.reset
    }

    /// Scope of a ratelimit when one occurs.
    #[must_use]
    pub const fn scope(&self) -> Option<RatelimitScope> {
        self.scope
    }
}

/// Scope of a ratelimit when one occurs.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RatelimitScope {
    /// Ratelimit is a global ratelimit and affects the application as a whole.
    Global,
    /// Ratelimit is a shared ratelimit and affects all applications in the
    /// resource.
    ///
    /// This does not affect the application's individual ratelimit buckets or
    /// global limits.
    Shared,
    /// Ratelimit is a per-resource limit, such as for an individual bucket.
    User,
}

impl Display for RatelimitScope {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(match self {
            Self::Global => "global",
            Self::Shared => "shared",
            Self::User => "user",
        })
    }
}

impl FromStr for RatelimitScope {
    type Err = HeaderParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "global" => Self::Global,
            "shared" => Self::Shared,
            "user" => Self::User,
            _ => {
                return Err(HeaderParsingError {
                    kind: HeaderParsingErrorType::Parsing {
                        kind: HeaderType::String,
                        name: HeaderName::Scope,
                        value: s.to_owned(),
                    },
                    source: None,
                })
            }
        })
    }
}

impl TryFrom<&'_ str> for RatelimitScope {
    type Error = HeaderParsingError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

/// Parsed headers.
///
/// The headers of a response may either indicate a global ratelimit, a bucketed
/// ratelimit, or no ratelimit at all.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum RatelimitHeaders {
    /// Ratelimit for all buckets encountered.
    GlobalLimited(GlobalLimited),
    /// No ratelimit headers present.
    None,
    /// Information about the ratelimit bucket is available.
    Present(Present),
}

impl RatelimitHeaders {
    /// Whether the ratelimit headers are a global ratelimit.
    #[must_use]
    pub const fn is_global(&self) -> bool {
        matches!(self, Self::GlobalLimited(_))
    }

    /// Whether there are no ratelimit headers.
    #[must_use]
    pub const fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    /// Whether the ratelimit headers are a present and not a global ratelimit.
    #[must_use]
    pub const fn is_present(&self) -> bool {
        matches!(self, Self::Present(_))
    }

    /// Parse headers from an iterator of tuples containing the header name and
    /// value.
    ///
    /// Headers names must be UTF-8 valid and lowercased while values *may* be
    /// UTF-8 valid. Most values will still be checked for validity prior to
    /// parsing.
    ///
    /// # Examples
    ///
    /// Parse a standard list of headers from a response:
    ///
    /// ```
    /// use std::array::IntoIter;
    /// use twilight_http_ratelimiting::RatelimitHeaders;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let iter = IntoIter::new([
    ///     ("x-ratelimit-bucket", "d721dea6054f6322373d361f98e5c38b".as_bytes()),
    ///     ("x-ratelimit-limit", "10".as_bytes()),
    ///     ("x-ratelimit-remaining", "9".as_bytes()),
    ///     ("x-ratelimit-reset", "1573795260.333".as_bytes()),
    ///     ("x-ratelimit-reset-after", "10.000".as_bytes()),
    /// ]);
    ///
    /// let headers = RatelimitHeaders::from_pairs(iter)?;
    /// assert!(matches!(
    ///     headers,
    ///     RatelimitHeaders::Present(p) if p.remaining() == 9,
    /// ));
    /// # Ok(()) }
    /// ```
    ///
    /// Parse a list of headers denoting that the user has been globally
    /// ratelimited:
    ///
    /// ```
    /// use std::array::IntoIter;
    /// use twilight_http_ratelimiting::RatelimitHeaders;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let headers = Vec::from([
    ///     ("retry-after", "487".as_bytes()),
    ///     ("x-ratelimit-global", "true".as_bytes()),
    /// ]);
    ///
    /// let headers = RatelimitHeaders::from_pairs(headers.into_iter())?;
    /// assert!(matches!(
    ///     headers,
    ///     RatelimitHeaders::GlobalLimited(g) if g.retry_after() == 487,
    /// ));
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// This method will error if a required header is missing or the header
    /// value is of an invalid type.
    pub fn from_pairs<'a>(
        headers: impl Iterator<Item = (&'a str, &'a [u8])>,
    ) -> Result<Self, HeaderParsingError> {
        let mut bucket = None;
        let mut global = false;
        let mut limit = None;
        let mut remaining = None;
        let mut reset = None;
        let mut reset_after = None;
        let mut retry_after = None;
        let mut scope = None;

        for (name, value) in headers {
            match name {
                HeaderName::BUCKET => {
                    bucket.replace(header_str(HeaderName::Bucket, value)?);
                }
                HeaderName::GLOBAL => {
                    global = header_bool(HeaderName::Global, value)?;
                }
                HeaderName::LIMIT => {
                    limit.replace(header_int(HeaderName::Limit, value)?);
                }
                HeaderName::REMAINING => {
                    remaining.replace(header_int(HeaderName::Remaining, value)?);
                }
                HeaderName::RESET => {
                    let reset_value = header_float(HeaderName::Reset, value)?;

                    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                    reset.replace((reset_value * 1000.).ceil() as u64);
                }
                HeaderName::RESET_AFTER => {
                    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
                    let reset_after_value =
                        (header_float(HeaderName::ResetAfter, value)? * 1000.).ceil() as u64;

                    reset_after.replace(reset_after_value);
                }
                HeaderName::RETRY_AFTER => {
                    let retry_after_value = header_int(HeaderName::RetryAfter, value)?;

                    retry_after.replace(retry_after_value);
                }
                HeaderName::SCOPE => {
                    let scope_value = header_str(HeaderName::Scope, value)?;
                    let scope_parsed = RatelimitScope::try_from(scope_value)?;

                    scope.replace(scope_parsed);
                }
                _ => continue,
            }
        }

        if global {
            let retry_after =
                retry_after.ok_or_else(|| HeaderParsingError::missing(HeaderName::RetryAfter))?;

            return Ok(RatelimitHeaders::GlobalLimited(GlobalLimited {
                retry_after,
                scope,
            }));
        }

        // If none of the values have been set then there are no ratelimit headers.
        // This means that the route is not ratelimited.
        if bucket.is_none()
            && limit.is_none()
            && remaining.is_none()
            && reset.is_none()
            && reset_after.is_none()
        {
            return Ok(RatelimitHeaders::None);
        }

        Ok(RatelimitHeaders::Present(Present {
            bucket: bucket.map(Into::into),
            limit: limit.ok_or_else(|| HeaderParsingError::missing(HeaderName::Limit))?,
            remaining: remaining
                .ok_or_else(|| HeaderParsingError::missing(HeaderName::Remaining))?,
            reset: reset.ok_or_else(|| HeaderParsingError::missing(HeaderName::Reset))?,
            reset_after: reset_after
                .ok_or_else(|| HeaderParsingError::missing(HeaderName::ResetAfter))?,
            scope,
        }))
    }
}

/// Parse a value as a boolean.
fn header_bool(name: HeaderName, value: &[u8]) -> Result<bool, HeaderParsingError> {
    let text = header_str(name, value)?;

    let end = text.parse().map_err(|source| HeaderParsingError {
        kind: HeaderParsingErrorType::Parsing {
            kind: HeaderType::Bool,
            name,
            value: text.to_owned(),
        },
        source: Some(Box::new(source)),
    })?;

    Ok(end)
}

/// Parse a value expected to be a float.
fn header_float(name: HeaderName, value: &[u8]) -> Result<f64, HeaderParsingError> {
    let text = header_str(name, value)?;

    let end = text.parse().map_err(|source| HeaderParsingError {
        kind: HeaderParsingErrorType::Parsing {
            kind: HeaderType::Float,
            name,
            value: text.to_owned(),
        },
        source: Some(Box::new(source)),
    })?;

    Ok(end)
}

/// Parse a value expected to be an integer.
fn header_int(name: HeaderName, value: &[u8]) -> Result<u64, HeaderParsingError> {
    let text = header_str(name, value)?;

    let end = text.parse().map_err(|source| HeaderParsingError {
        kind: HeaderParsingErrorType::Parsing {
            kind: HeaderType::Integer,
            name,
            value: text.to_owned(),
        },
        source: Some(Box::new(source)),
    })?;

    Ok(end)
}

/// Parse a value expected to be a UTF-8 valid string.
fn header_str(name: HeaderName, value: &[u8]) -> Result<&str, HeaderParsingError> {
    let text = str::from_utf8(value)
        .map_err(|source| HeaderParsingError::not_utf8(name, value.to_owned(), source))?;

    Ok(text)
}

#[cfg(test)]
mod tests {
    use super::{
        GlobalLimited, HeaderName, HeaderParsingError, HeaderParsingErrorType, HeaderType, Present,
        RatelimitHeaders,
    };
    use crate::headers::RatelimitScope;
    use http::header::{HeaderMap, HeaderName as HttpHeaderName, HeaderValue};
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{
        error::Error,
        fmt::{Debug, Display},
    };

    assert_fields!(HeaderParsingErrorType::Missing: name);
    assert_fields!(HeaderParsingErrorType::NotUtf8: name, value);
    assert_fields!(HeaderParsingErrorType::Parsing: kind, name, value);
    assert_impl_all!(
        HeaderName: Clone,
        Copy,
        Debug,
        Display,
        Eq,
        PartialEq,
        Send,
        Sync
    );
    assert_impl_all!(HeaderParsingErrorType: Debug, Send, Sync);
    assert_impl_all!(HeaderParsingError: Error, Send, Sync);
    assert_impl_all!(
        HeaderType: Clone,
        Copy,
        Debug,
        Display,
        Eq,
        PartialEq,
        Send,
        Sync
    );
    assert_impl_all!(GlobalLimited: Clone, Debug, Eq, PartialEq, Send, Sync);
    assert_impl_all!(Present: Clone, Debug, Eq, PartialEq, Send, Sync);
    assert_impl_all!(RatelimitHeaders: Clone, Debug, Send, Sync);

    #[test]
    fn global() -> Result<(), Box<dyn Error>> {
        let map = {
            let mut map = HeaderMap::new();
            map.insert(
                HttpHeaderName::from_static("x-ratelimit-global"),
                HeaderValue::from_static("true"),
            );
            map.insert(
                HttpHeaderName::from_static("retry-after"),
                HeaderValue::from_static("65"),
            );

            map
        };

        let iter = map.iter().map(|(k, v)| (k.as_str(), v.as_bytes()));
        let headers = RatelimitHeaders::from_pairs(iter)?;
        assert!(matches!(headers, RatelimitHeaders::GlobalLimited(g) if g.retry_after() == 65));

        Ok(())
    }

    #[test]
    fn global_with_scope() -> Result<(), Box<dyn Error>> {
        let map = {
            let mut map = HeaderMap::new();
            map.insert(
                HttpHeaderName::from_static("x-ratelimit-global"),
                HeaderValue::from_static("true"),
            );
            map.insert(
                HttpHeaderName::from_static("retry-after"),
                HeaderValue::from_static("65"),
            );
            map.insert(
                HttpHeaderName::from_static("x-ratelimit-scope"),
                HeaderValue::from_static("global"),
            );

            map
        };

        let iter = map.iter().map(|(k, v)| (k.as_str(), v.as_bytes()));
        let headers = RatelimitHeaders::from_pairs(iter)?;
        assert!(matches!(
            headers,
            RatelimitHeaders::GlobalLimited(ref global)
            if global.retry_after() == 65
        ));
        assert!(matches!(
            headers,
            RatelimitHeaders::GlobalLimited(global)
            if global.scope() == Some(RatelimitScope::Global)
        ));

        Ok(())
    }

    #[test]
    fn present() -> Result<(), Box<dyn Error>> {
        let map = {
            let mut map = HeaderMap::new();
            map.insert(
                HttpHeaderName::from_static("x-ratelimit-limit"),
                HeaderValue::from_static("10"),
            );
            map.insert(
                HttpHeaderName::from_static("x-ratelimit-remaining"),
                HeaderValue::from_static("9"),
            );
            map.insert(
                HttpHeaderName::from_static("x-ratelimit-reset"),
                HeaderValue::from_static("1470173023.123"),
            );
            map.insert(
                HttpHeaderName::from_static("x-ratelimit-reset-after"),
                HeaderValue::from_static("64.57"),
            );
            map.insert(
                HttpHeaderName::from_static("x-ratelimit-bucket"),
                HeaderValue::from_static("abcd1234"),
            );
            map.insert(
                HttpHeaderName::from_static("x-ratelimit-scope"),
                HeaderValue::from_static("shared"),
            );

            map
        };

        let iter = map.iter().map(|(k, v)| (k.as_str(), v.as_bytes()));
        let headers = RatelimitHeaders::from_pairs(iter)?;
        assert!(matches!(
            headers,
            RatelimitHeaders::Present(ref present)
            if present.bucket.as_deref() == Some("abcd1234")
        ));
        assert!(matches!(
            headers,
            RatelimitHeaders::Present(ref present)
            if present.limit == 10
        ));
        assert!(matches!(
            headers,
            RatelimitHeaders::Present(ref present)
            if present.remaining == 9
        ));
        assert!(matches!(
            headers,
            RatelimitHeaders::Present(ref present)
            if present.reset_after == 64_570
        ));
        assert!(matches!(
            headers,
            RatelimitHeaders::Present(ref present)
            if present.reset == 1_470_173_023_123
        ));
        assert!(matches!(
            headers,
            RatelimitHeaders::Present(present)
            if present.scope() == Some(RatelimitScope::Shared)
        ));

        Ok(())
    }

    #[test]
    fn name() {
        assert_eq!("x-ratelimit-bucket", HeaderName::BUCKET);
        assert_eq!("x-ratelimit-global", HeaderName::GLOBAL);
        assert_eq!("x-ratelimit-limit", HeaderName::LIMIT);
        assert_eq!("x-ratelimit-remaining", HeaderName::REMAINING);
        assert_eq!("x-ratelimit-reset-after", HeaderName::RESET_AFTER);
        assert_eq!("x-ratelimit-reset", HeaderName::RESET);
        assert_eq!("retry-after", HeaderName::RETRY_AFTER);
        assert_eq!("x-ratelimit-scope", HeaderName::SCOPE);
        assert_eq!(HeaderName::BUCKET, HeaderName::Bucket.name());
        assert_eq!(HeaderName::GLOBAL, HeaderName::Global.name());
        assert_eq!(HeaderName::LIMIT, HeaderName::Limit.name());
        assert_eq!(HeaderName::REMAINING, HeaderName::Remaining.name());
        assert_eq!(HeaderName::RESET_AFTER, HeaderName::ResetAfter.name());
        assert_eq!(HeaderName::RESET, HeaderName::Reset.name());
        assert_eq!(HeaderName::RETRY_AFTER, HeaderName::RetryAfter.name());
        assert_eq!(HeaderName::SCOPE, HeaderName::Scope.name());
    }

    #[test]
    fn type() {
        assert_eq!("bool", HeaderType::Bool.name());
        assert_eq!("float", HeaderType::Float.name());
        assert_eq!("integer", HeaderType::Integer.name());
        assert_eq!("string", HeaderType::String.name());
    }
}
