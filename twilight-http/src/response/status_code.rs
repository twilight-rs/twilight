use std::fmt::{Display, Formatter, Result as FmtResult};

/// Status code of a response.
///
/// Constants are defined for the response codes that Discord defines in their [response code table].
///
/// # Comparing the status code
///
/// Status codes can easily be compared with status code integers due to
/// implementing `PartialEq<u16>`. This is equivalent to checking against the
/// value returned by [`StatusCode::get`].
///
/// [response code table]: https://discord.com/developers/docs/topics/opcodes-and-status-codes#http-http-response-codes
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct StatusCode(u16);

impl StatusCode {
    /// Create a new status code from a raw status code.
    pub(crate) const fn new(code: u16) -> Self {
        // We don't need to do any checking that the status code is valid since
        // the value always comes from `hyper`'s status code implementation.
        Self(code)
    }

    /// Raw status code value.
    #[must_use = "status code must be used to be useful"]
    pub const fn get(self) -> u16 {
        self.0
    }

    /// Whether the status code is informational.
    ///
    /// This is defined as being between `[100, 200)`.
    #[must_use = "whether a status code is informational must be used"]
    pub const fn is_informational(self) -> bool {
        self.in_range(100, 200)
    }

    /// Whether the status code is a success.
    ///
    /// This is defined as being between `[200, 300)`.
    #[must_use = "whether a status code is a success must be used"]
    pub const fn is_success(self) -> bool {
        self.in_range(200, 300)
    }

    /// Whether the status code is a redirection.
    ///
    /// This is defined as being between `[300, 400)`.
    #[must_use = "whether a status code is redirectional must be used"]
    pub const fn is_redirection(self) -> bool {
        self.in_range(300, 400)
    }

    /// Whether the status code is a client error.
    ///
    /// This is defined as being between `[400, 500)`.
    #[must_use = "whether a status code is a client error must be used"]
    pub const fn is_client_error(self) -> bool {
        self.in_range(400, 500)
    }

    /// Whether the status code is a server error.
    ///
    /// This is defined as being between `[500, 600)`.
    #[must_use = "whether a status code is a server error must be used"]
    pub const fn is_server_error(self) -> bool {
        self.in_range(500, 600)
    }

    /// Whether the status code is within a range.
    ///
    /// The range is defined as `[min, max)`.
    const fn in_range(self, min: u16, max: u16) -> bool {
        self.0 >= min && self.0 < max
    }

    /// 200 (OK) The request completed successfully.
    pub const OK: StatusCode = StatusCode::new(200);

    /// 201 (CREATED) The entity was created successfully.
    pub const CREATED: StatusCode = StatusCode::new(201);

    /// 204 (NO CONTENT) The request completed successfully but returned no content.
    pub const NO_CONTENT: StatusCode = StatusCode::new(204);

    /// 304 (NOT MODIFIED) The entity was not modified (no action was taken).
    pub const NOT_MODIFIED: StatusCode = StatusCode::new(304);

    /// 400 (BAD REQUEST) The request was improperly formatted, or the server couldn't understand it.
    pub const BAD_REQUEST: StatusCode = StatusCode::new(400);

    /// 401 (UNAUTHORIZED) The Authorization header was missing or invalid.
    pub const UNAUTHORIZED: StatusCode = StatusCode::new(401);

    /// 403 (FORBIDDEN) The Authorization token you passed did not have permission to the resource.
    pub const FORBIDDEN: StatusCode = StatusCode::new(403);

    /// 404 (NOT FOUND) The resource at the location specified doesn't exist.
    pub const NOT_FOUND: StatusCode = StatusCode::new(404);

    /// 405 (METHOD NOT ALLOWED) The HTTP method used is not valid for the location specified.
    pub const METHOD_NOT_ALLOWED: StatusCode = StatusCode::new(405);

    /// 429 (TOO MANY REQUESTS) You are being rate limited, see Rate Limits.
    pub const TOO_MANY_REQUESTS: StatusCode = StatusCode::new(429);

    /// 502 (GATEWAY UNAVAILABLE) There was not a gateway available to process your request. Wait a bit and retry.
    pub const GATEWAY_UNAVAILABLE: StatusCode = StatusCode::new(502);
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.0.fmt(f)
    }
}

impl PartialEq<u16> for StatusCode {
    fn eq(&self, other: &u16) -> bool {
        self.get() == *other
    }
}

impl PartialEq<StatusCode> for u16 {
    fn eq(&self, other: &StatusCode) -> bool {
        *self == other.get()
    }
}

#[cfg(test)]
mod tests {
    use super::StatusCode;
    use static_assertions::assert_impl_all;
    use std::{
        fmt::{Debug, Display},
        hash::Hash,
    };

    assert_impl_all!(
        StatusCode: Clone,
        Copy,
        Debug,
        Display,
        Eq,
        Hash,
        PartialEq,
        PartialOrd,
        Ord,
        Send,
        Sync
    );

    #[test]
    fn eq_with_integer() {
        assert_eq!(200_u16, StatusCode::new(200));
        assert_eq!(StatusCode::new(404), 404_u16);
    }

    /// Test that [`StatusCode::get`] returns the raw value of the status code.
    ///
    /// Notably what we want to test here is that it's not repeatedly returning
    /// the same value (as if it were hardcoded), and that it's instead
    /// returning the provided value.
    #[test]
    fn get() {
        assert_eq!(200, StatusCode::new(200).get());
        assert_eq!(403, StatusCode::new(403).get());
        assert_eq!(404, StatusCode::new(404).get());
    }

    #[test]
    fn ranges() {
        assert!(StatusCode::new(100).is_informational());
        assert!(StatusCode::new(199).is_informational());
        assert!(StatusCode::new(200).is_success());
        assert!(StatusCode::new(299).is_success());
        assert!(StatusCode::new(300).is_redirection());
        assert!(StatusCode::new(399).is_redirection());
        assert!(StatusCode::new(400).is_client_error());
        assert!(StatusCode::new(499).is_client_error());
        assert!(StatusCode::new(500).is_server_error());
        assert!(StatusCode::new(599).is_server_error());
    }
}
