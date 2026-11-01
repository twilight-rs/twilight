//! Validated route information for HTTP requests.
//!
//! When working with the Discord HTTP REST API it is desirable to ensure we
//! are working with validated URL paths. Although "/guilds1/messages/2" is a
//! valid path in the HTTP specification, it is not one valid for the purposes
//! of working with the API. This module provides [`PathBuilder`] for building
//! paths out of segments and [`Path`], a wrapper ensuring its value has been
//! built as expected.
//!
//! # When to use paths
//!
//! Users will only need to manually use [`Path`]s when manually constructing
//! [`Request`]s where Twilight does not support an HTTP request endpoint.
//! For everyday use of the library the provided request builders automatically
//! construct paths.
//!
//! [`Request`]: super::Request

pub mod query_parameter;

mod path_builder;

pub use self::path_builder::{
    ActionMarker, IdMarker, Me, PathBuilder, PathBuilderError, PathBuilderErrorType, PathId,
    QueryMarker, ResourceMarker, SubresourceMarker,
};

use std::{
    error::Error,
    fmt::{Display, Error as FmtError, Formatter},
    str::FromStr,
};

/// Error when parsing a [`Path`].
#[derive(Debug)]
pub struct PathParseError {
    /// Type of error.
    pub(super) kind: PathParseErrorType,
    /// Source error if available.
    pub(super) source: Option<Box<dyn Error + Send + Sync>>,
}

impl PathParseError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &PathParseErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        self.source
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(self) -> (PathParseErrorType, Option<Box<dyn Error + Send + Sync>>) {
        (self.kind, self.source)
    }
}

impl Display for PathParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        match self.kind {
            PathParseErrorType::Length => f.write_str("path has too few or too many segments"),
            PathParseErrorType::SegmentInvalid => f.write_str("path contains an invalid segment"),
        }
    }
}

impl Error for PathParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|source| &**source as _)
    }
}

/// Type of [`PathParseError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum PathParseErrorType {
    /// Segment of a path is an invalid length.
    ///
    /// This occurs when a segment is too short, i.e. an empty string.
    Length,
    /// Segment of a path is invalid.
    ///
    /// This occurs when a type of segment is invalid, such as when two IDs are
    /// consecutive in the path or there are too many consecutive nested
    /// resources. An example is `"foo/123/bar/baz/qux/quz"`, which contains too
    /// many consecutive resources.
    SegmentInvalid,
}

/// Validated path within the API.
///
/// Paths are not prefixed with `/`, meaning a path might be in the shape of
/// `"channels/123/messages/456"`.
///
/// Paths may be parsed via [`FromStr`].
///
/// # Examples
///
/// ```rust
/// use twilight_http::request::path::Path;
/// use twilight_model::id::{Id, marker::ChannelMarker};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let path = Path::builder()
///     .resource("channels")
///     .id(Id::<ChannelMarker>::new(1))
///     .build()?;
///
/// // The Path can be retrieved as a string slice
/// let as_string_slice: &str = path.as_ref();
///
/// // It implements Display and can be formatted.
/// println!("Path: {}", path);
///
/// // And it can be compared against strings:
/// assert_eq!("channels/1", path);
/// # Ok(()) }
/// ```
#[derive(Clone, Debug)]
pub struct Path {
    /// Inner built or validated path.
    inner: String,
}

impl Path {
    /// Create a path from a validated string or from a [`PathBuilder`].
    ///
    /// The string must be validated beforehand.
    const fn new(inner: String) -> Self {
        Self { inner }
    }

    /// Create a builder to construct a valid path.
    pub const fn builder() -> PathBuilder<()> {
        PathBuilder::new()
    }
}

impl AsRef<str> for Path {
    fn as_ref(&self) -> &str {
        &self.inner
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        Display::fmt(&self.inner, f)
    }
}

impl FromStr for Path {
    type Err = PathParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const TERMINATORS: &[char] = &['/', '?'];

        let s = s.trim_start_matches('/');

        if s.is_empty() {
            return Err(Self::Err {
                kind: PathParseErrorType::Length,
                source: None,
            });
        }

        let mut state = PathParseState::new();

        let segments = s.split_inclusive(TERMINATORS);
        let mut previous_terminator = None;

        for mut segment in segments {
            let maybe_terminator = TERMINATORS
                .iter()
                .find(|possible_terminator| segment.ends_with(**possible_terminator))
                .copied();

            if let Some(terminator) = maybe_terminator {
                segment = segment.trim_end_matches(terminator);
            }

            if let Some(segment_type) =
                PathSegmentType::parse(&mut state, previous_terminator, segment)
            {
                if !state.push(segment_type) {
                    return Err(Self::Err {
                        kind: PathParseErrorType::Length,
                        source: None,
                    });
                }
            } else {
                return Err(Self::Err {
                    kind: PathParseErrorType::SegmentInvalid,
                    source: None,
                });
            }

            previous_terminator = maybe_terminator;
        }

        Ok(Self {
            inner: s.to_owned(),
        })
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl PartialEq<Path> for &str {
    fn eq(&self, other: &Path) -> bool {
        *self == other.inner
    }
}

impl PartialEq<&str> for Path {
    fn eq(&self, other: &&str) -> bool {
        self.inner == *other
    }
}

/// State for parsing a [Path].
///
/// Segments are positional and can only occur after certain segments, so the
/// history of previous segments is necessary when parsing the next segment.
#[derive(Debug)]
struct PathParseState {
    /// History of segments within a path.
    segments: [Option<PathSegmentType>; Self::MAX_LEN],
}

impl PathParseState {
    /// Maximum number of path segment types allowed.
    ///
    /// The current value is likely more than enough. 12, for example, allows
    /// a path such as:
    /// "foo/123/bar/baz/qux/456/another/resource/789/again/action?query=string"
    const MAX_LEN: usize = 12;

    /// Create a new state for parsing a [Path].
    const fn new() -> Self {
        Self {
            segments: [None; _],
        }
    }

    /// Get the number of path segment types in the history.
    const fn len(&self) -> usize {
        let mut index = 0;

        while index < Self::MAX_LEN {
            if self.segments[index].is_none() {
                break;
            }

            index += 1;
        }

        index
    }

    /// Push a path segment type to the end of the segment history.
    ///
    /// Returns whether the path segment type was added.
    #[must_use = "whether the path segment type was added must be handled"]
    const fn push(&mut self, segment_type: PathSegmentType) -> bool {
        let len = self.len();

        if len == Self::MAX_LEN {
            false
        } else {
            self.segments[len] = Some(segment_type);

            true
        }
    }

    /// Retrieve the most recent path segment types necessary for determining
    /// which type the next path segment might be.
    fn recent(&self) -> [Option<PathSegmentType>; 3] {
        let len = self.len();

        [
            self.segments.get(len.wrapping_sub(1)).and_then(|x| *x),
            self.segments.get(len.wrapping_sub(2)).and_then(|x| *x),
            self.segments.get(len.wrapping_sub(3)).and_then(|x| *x),
        ]
    }
}

impl FromIterator<PathSegmentType> for PathParseState {
    fn from_iter<T: IntoIterator<Item = PathSegmentType>>(iter: T) -> Self {
        let mut this = Self::new();

        for item in iter {
            assert!(
                this.push(item),
                "iterator must be at most {} elements long",
                Self::MAX_LEN
            );
        }

        this
    }
}

/// Type of a segment within a path.
///
/// Path segments aren't surfaced to the user, but are necessary for determining
/// whether the shape of a path is valid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PathSegmentType {
    /// Path segment is a code.
    ///
    /// Codes are special. They can't be discerned from resources and
    /// subresources; instead, they have to be differentiated from these based
    /// on surrounding context after-the-fact.
    ///
    /// This segment can occur after:
    /// - Subresource
    /// - Id
    Code,
    /// Path segment is an ID.
    ///
    /// This segment can occur after:
    /// - Resource
    /// - Subresource
    Id,
    /// Path segment is a query.
    ///
    /// There are no valid segments after this.
    ///
    /// This segment can occur after:
    /// - Code
    /// - Id
    /// - Resource
    /// - Subresource
    Query,
    /// Path segment is a resource.
    ///
    /// This segment can occur after:
    /// - Id
    Resource,
    /// Path segment is a subresource.
    ///
    /// This segment can occur after:
    /// - Resource
    Subresource,
}

impl PathSegmentType {
    /// Determine whether a value is an ID.
    ///
    /// An ID is one of:
    ///
    /// - An integer; or
    /// - "@me", a sentinel ID referring to the current user
    fn is_id(value: &str) -> bool {
        u64::from_str(value).is_ok() || value == "@me"
    }

    /// Parse a segment based on the previous segment and terminator.
    ///
    /// Segment types can only occur after certain other types of segments. For
    /// example, a Subresource can only occur after a Resource. Similarly, a
    /// segment which is an ID value should be an `Id` segment type, which can
    /// only occur after certain segments. Knowing the recent history of
    /// segments is necessary to determine what type of segment a value is.
    fn parse(
        state: &mut PathParseState,
        previous_terminator: Option<char>,
        value: &str,
    ) -> Option<Self> {
        if value.is_empty() {
            return None;
        }

        match (previous_terminator, state.recent()) {
            (Some('/'), [Some(Self::Subresource), Some(Self::Resource), _]) => {
                Some(if Self::is_id(value) {
                    Self::Id
                } else {
                    Self::Code
                })
            }
            (
                Some('?'),
                [Some(Self::Code | Self::Id), Some(_), _]
                | [Some(Self::Resource), _, _]
                | [Some(Self::Subresource), Some(Self::Resource), _],
            ) => Some(Self::Query),
            (
                Some('/'),
                [Some(Self::Id), Some(Self::Subresource | Self::Resource), _] | [None, None, None],
            ) => (!Self::is_id(value)).then_some(Self::Resource),
            (Some('/'), [Some(Self::Resource), _, _]) => {
                if Self::is_id(value) {
                    Some(Self::Id)
                } else {
                    Some(Self::Subresource)
                }
            }
            (None, [None, None, None]) => {
                // Root. Must be a Resource.

                // Ensure we don't have an ID.
                if Self::is_id(value) {
                    return None;
                }

                Some(Self::Resource)
            }
            (_, [None, None | Some(_), Some(_)] | [None, Some(_), None]) => {
                unreachable!()
            }
            (_, [_, _, _]) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Path, PathParseState, PathSegmentType};
    use std::str::FromStr as _;

    const SLASH: char = '/';

    #[test]
    fn path_parse_resource_subresource_action() {
        // assert!(Path::from_str("foo/bar/baz").is_ok());
        Path::from_str("foo/bar/baz").unwrap();
    }

    #[test]
    fn path_segment_type_parse_root_resource() {
        let mut state = PathParseState::new();

        assert_eq!(
            Some(PathSegmentType::Resource),
            PathSegmentType::parse(&mut state, None, "foo"),
        );
    }

    #[test]
    fn path_segment_type_parse_resource_id() {
        let mut state = PathParseState::from_iter([PathSegmentType::Resource]);

        assert_eq!(
            Some(PathSegmentType::Id),
            PathSegmentType::parse(&mut state, Some(SLASH), "1"),
        );
    }

    #[test]
    fn path_segment_type_parse_resource_id_resource() {
        let mut state = PathParseState::from_iter([PathSegmentType::Resource, PathSegmentType::Id]);

        assert_eq!(
            Some(PathSegmentType::Resource),
            PathSegmentType::parse(&mut state, Some(SLASH), "foo"),
        );
    }
}
