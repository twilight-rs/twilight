//! Validated HTTP paths for working with Discord API [`Route`]s.
//!
//! When working with the Discord HTTP REST API it is desirable to ensure we
//! are working with validated URL paths. Although "/guilds1/messages/2" is a
//! valid path in the HTTP specification, it is not one valid for the purposes
//! of working with the API. This module provides [`PathBuilder`] for building
//! paths out of segments and [`Path`], a wrapper ensuring its value has been
//! built as expected.
//!
//! [`Route`]: super::Route

pub mod query_parameter;

mod builder;

pub use self::builder::{
    ActionMarker, IdMarker, PathBuilder, QueryMarker, ResourceMarker, SubresourceMarker,
};

use std::fmt::{Display, Error as FmtError, Formatter};

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
/// let path = Path::builder()
///     .resource("channels")
///     .id(Id::<ChannelMarker>::new(1))
///     .build();
///
/// // The Path can be retrieved as a string slice
/// let as_string_slice: &str = path.as_ref();
///
/// // It implements Display and can be formatted.
/// println!("Path: {}", path);
///
/// // And it can be compared against strings:
/// assert_eq!("channels/1", path);
/// ```
#[derive(Clone, Debug)]
pub struct Path {
    inner: String,
}

impl Path {
    pub fn builder() -> PathBuilder<()> {
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
