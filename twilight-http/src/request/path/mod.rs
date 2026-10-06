//! Validated HTTP paths for working with [`Route`]s.
//!
//! When working with the Discord HTTP REST API it is desirable to ensure we
//! are working with validated URL paths. Although "/guilds1/messages/2" is a
//! valid path in the HTTP specification, it is not one valid for the purposes
//! of working with the API. This module provides [`PathBuilder`] for building
//! paths out of segments and [`Path`], a wrapper ensuring its value has been
//! built as expected.

pub mod builder;

mod query_parameter;

pub use self::query_parameter::QueryParameter;

pub(crate) use self::query_parameter::QueryParameterDisplay;

use self::builder::PathBuilder;
use std::fmt::{Display, Error as FmtError, Formatter};

#[derive(Debug)]
pub struct Path {
    inner: String,
}

impl Path {
    pub fn builder() -> PathBuilder<()> {
        PathBuilder::new()
    }

    pub fn get(&self) -> &str {
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
