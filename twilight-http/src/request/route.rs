use super::{Method, path::Path};

/// HTTP route definition for an HTTP request builder.
///
/// All request builders must implement route information in order to be used
/// with Twilight's HTTP client.
pub trait Route {
    /// HTTP path and query parameter fields to be passed to [`path`].
    ///
    /// Fields should not include request body keys, which should be stored
    /// separate from path and query parameter fields.
    type Fields;

    /// Method of the HTTP request.
    const METHOD: Method;

    /// HTTP path and query parameters for the HTTP request.
    ///
    /// Paths must be built via [`PathBuilder`] to ensure path structure is
    /// valid for use with the Discord HTTP API.
    ///
    /// [`PathBuilder`]: crate::request::path::builder::PathBuilder
    fn path(fields: Self::Fields) -> Path;
}
