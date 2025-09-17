//! Endpoint parameters for the rate limiter.
//!
//! The [`Ratelimiter`] uses [`Endpoint`] to associate permits with endpoints.

use std::hash::{Hash as _, Hasher};

/// HTTP request [method].
///
/// [method]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum Method {
    /// Delete a resource.
    Delete,
    /// Retrieve a resource.
    Get,
    /// Update a resource.
    Patch,
    /// Create a resource.
    Post,
    /// Replace a resource.
    Put,
}

impl Method {
    /// Name of the method.
    pub const fn name(self) -> &'static str {
        match self {
            Method::Delete => "DELETE",
            Method::Get => "GET",
            Method::Patch => "PATCH",
            Method::Post => "POST",
            Method::Put => "PUT",
        }
    }
}

/// Ratelimit endpoint.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Endpoint {
    /// Method of the endpoint.
    pub method: Method,
    /// Path of the endpoint.
    pub path: String,
}

impl Endpoint {
    /// Normalizes the path.
    #[must_use]
    fn path(&self) -> &[u8] {
        let path = self.path.as_bytes();
        if path.first().is_some_and(|&b| b == b'/') {
            &path[1..]
        } else {
            path
        }
    }

    /// Whether the path is an interaction path.
    pub(crate) fn is_interaction(&self) -> bool {
        self.path().starts_with(b"webhooks") || self.path().starts_with(b"interactions")
    }

    /// Feeds the top level components of this path into the given [`Hasher`].
    pub(crate) fn hash_components(&self, state: &mut impl Hasher) {
        let mut segments = self.path().split(|&s| s == b'/');
        match segments.next().unwrap_or_default() {
            b"channels" | b"guilds" => {
                if let Some(s) = segments.next() {
                    s.hash(state);
                }
            }
            b"webhooks" => {
                if let Some(s) = segments.next() {
                    s.hash(state);
                }
                if let Some(s) = segments.next() {
                    s.hash(state);
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Method;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(Method: Clone, Copy, Debug, Eq, PartialEq);

    #[test]
    fn method_conversions() {
        assert_eq!("DELETE", Method::Delete.name());
        assert_eq!("GET", Method::Get.name());
        assert_eq!("PATCH", Method::Patch.name());
        assert_eq!("POST", Method::Post.name());
        assert_eq!("PUT", Method::Put.name());
    }
}
