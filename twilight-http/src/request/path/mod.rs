//! Validated route information for HTTP requests.
//!
//! When working with the Discord HTTP REST API it is desirable to ensure we
//! are working with validated URL paths. Although "/guilds1/messages/2" is a
//! valid path in the HTTP specification, it is not one valid for the purposes
//! of working with the API. This module provides [`PathBuilder`] for building
//! paths out of segments and [`Path`], a wrapper ensuring its value has been
//! built as expected.

pub mod query_parameter;

mod path_builder;

pub use self::path_builder::{
    ActionMarker, IdMarker, PathBuilder, PathBuilderError, PathBuilderErrorType, PathId,
    QueryMarker, ResourceMarker, SubresourceMarker,
};

use std::{
    error::Error,
    fmt::{Display, Error as FmtError, Formatter},
    str::FromStr,
};

#[derive(Debug)]
pub struct PathParseError {
    pub(super) kind: PathParseErrorType,
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
    Length,
    SegmentInvalid,
}

/// Validated path within the API.
///
/// The Path is not prefixed with `/`.
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
    inner: String,
}

impl Path {
    pub(crate) const fn new(inner: String) -> Self {
        Self { inner }
    }

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
                        kind: PathParseErrorType::SegmentInvalid,
                        source: None,
                    });
                }
            } else {
                return Err(Self::Err {
                    kind: PathParseErrorType::Length,
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

struct PathParseState {
    inner: [Option<PathSegmentType>; Self::MAX_LEN],
}

impl PathParseState {
    const MAX_LEN: usize = 12;

    const fn new() -> Self {
        Self { inner: [None; _] }
    }

    #[expect(dead_code)]
    fn last_mut(&mut self) -> Option<&mut PathSegmentType> {
        let len = self.len();

        if let Some(x) = self.inner.get_mut(len) {
            x.as_mut()
        } else {
            None
        }
    }

    const fn len(&self) -> usize {
        let mut index = 0;

        while index < Self::MAX_LEN {
            if self.inner[index].is_none() {
                break;
            }

            index += 1;
        }

        index
    }

    const fn push(&mut self, segment_type: PathSegmentType) -> bool {
        let len = self.len();

        if len == Self::MAX_LEN {
            false
        } else {
            self.inner[len] = Some(segment_type);

            true
        }
    }

    fn recent(&self) -> [Option<PathSegmentType>; 3] {
        let len = self.len();

        [
            self.inner.get(len.wrapping_sub(1)).and_then(|x| *x),
            self.inner.get(len.wrapping_sub(2)).and_then(|x| *x),
            self.inner.get(len.wrapping_sub(3)).and_then(|x| *x),
        ]
    }
}

impl FromIterator<PathSegmentType> for PathParseState {
    fn from_iter<T: IntoIterator<Item = PathSegmentType>>(iter: T) -> Self {
        let mut this = Self::new();

        for item in iter {
            this.push(item);
        }

        this
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PathSegmentType {
    /// Path segment is an action.
    ///
    /// Valid segments after this:
    /// - Query
    Action,
    /// Path segment is a code.
    ///
    /// Codes are special. They can't be discerned from resources and
    /// subresources; instead, they have to be differentiated from these based
    /// on surrounding context after-the-fact.
    ///
    /// Valid segments after this:
    /// - Action
    /// - Query
    /// - Resource
    #[allow(dead_code)]
    Code,
    /// Path segment is an ID.
    ///
    /// Valid segments after this:
    /// - Action
    /// - Query
    /// - Resource
    Id,
    /// Path segment is a query.
    ///
    /// There are no valid segments after this.
    Query,
    /// Path segment is a resource.
    ///
    /// Valid segments after this:
    /// - Action
    /// - ID (integer, Id<T>, "@me")
    /// - Query
    /// - Subresource
    Resource,
    /// Path segment is a subresource.
    ///
    /// Valid segments after this:
    /// - Action
    /// - ID
    /// - Query
    Subresource,
}

impl PathSegmentType {
    fn is_id(value: &str) -> bool {
        u64::from_str(value).is_ok() || value == "@me"
    }

    fn parse(
        state: &mut PathParseState,
        previous_terminator: Option<char>,
        value: &str,
    ) -> Option<Self> {
        if value.is_empty() {
            return None;
        }

        match (previous_terminator, state.recent()) {
            (Some('/'), [Some(Self::Resource), Some(Self::Subresource), _]) => {
                (!Self::is_id(value)).then_some(Self::Action)
            }
            (Some('/'), [Some(Self::Subresource), Some(Self::Resource), _]) => {
                Self::is_id(value).then_some(Self::Id)
            }
            (
                Some('?'),
                [Some(Self::Action | Self::Id), Some(_), _]
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
    use super::{PathParseState, PathSegmentType};

    const SLASH: char = '/';

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

    #[ignore]
    #[test]
    fn path_segment_type_parse_secondary_id() {
        let mut state = PathParseState::from_iter([
            PathSegmentType::Resource,
            PathSegmentType::Id,
            PathSegmentType::Resource,
            PathSegmentType::Subresource,
        ]);

        assert_eq!(
            Some(PathSegmentType::Id),
            PathSegmentType::parse(&mut state, Some(SLASH), "1")
        );

        // Also confirm that the Resource has become a Code and the Subresource
        // has become a Resource.
        assert_eq!(
            [
                Some(PathSegmentType::Id),
                Some(PathSegmentType::Resource),
                Some(PathSegmentType::Code),
            ],
            state.recent()
        );
    }
}
