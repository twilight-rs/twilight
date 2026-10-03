use super::{Method, path::Path};

/// HTTP route definition for an HTTP request builder.
pub trait Route {
    type Fields;

    /// Method of the HTTP request.
    const METHOD: Method;

    /// HTTP path and query parameters for the HTTP request.
    fn path(fields: Self::Fields) -> Path;
}
