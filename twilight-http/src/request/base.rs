use super::{Form, Method, path::Path};
use crate::{error::Error, routing::Route};
use http::header::{HeaderMap, HeaderName, HeaderValue};
use serde::Serialize;

/// Builder to create a customized request.
///
/// # Examples
///
/// Create a request to create a message with a content of "test" in a
/// channel with an ID of 1:
///
/// ```
/// use twilight_http::{request::Request, routing::Route};
///
/// let body = br#"{
///     "content": "test"
/// }"#
/// .to_vec();
///
/// let request = Request::builder(&Route::CreateMessage { channel_id: 1 })
///     .body(body)
///     .build();
/// ```
#[derive(Debug)]
#[must_use = "request has not been fully built"]
pub struct RequestBuilder(Result<Request, Error>);

impl RequestBuilder {
    /// Create a new request builder.
    // Clippy: We may not want this to be const in the future.
    #[expect(clippy::missing_const_for_fn)]
    pub fn new(method: Method, path: Path) -> Self {
        Self(Ok(Request {
            body: None,
            form: None,
            headers: None,
            method,
            path,
            use_authorization_token: true,
        }))
    }

    pub fn new_from_route(route: &Route<'_>) -> Self {
        Self(Ok(Request::from_route(route)))
    }

    /// Consume the builder, returning the built request.
    ///
    /// # Errors
    ///
    /// Returns an [`ErrorType::Json`] error type JSON input could not be
    /// serialized.
    ///
    /// [`ErrorType::Json`]: crate::error::ErrorType::Json
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "request information is not useful on its own and must be acted on"]
    pub fn build(self) -> Result<Request, Error> {
        self.0
    }

    /// Set the contents of the body.
    pub fn body(mut self, body: Vec<u8>) -> Self {
        if let Ok(request) = self.0.as_mut() {
            request.body = Some(body);
        }

        self
    }

    /// Set the the multipart form from an existing boundary and buffer.
    pub fn multipart(mut self, boundary: [u8; 15], buffer: Vec<u8>) -> Self {
        if let Ok(request) = self.0.as_mut() {
            request.form = Some(Form::from_parts(boundary, buffer));
        }

        self
    }

    /// Set the headers to add.
    pub fn headers(mut self, iter: impl Iterator<Item = (HeaderName, HeaderValue)>) -> Self {
        if let Ok(request) = self.0.as_mut() {
            request.headers.replace(iter.collect());
        }

        self
    }

    /// Set the body, to be serialized as JSON.
    pub fn json(mut self, to: &impl Serialize) -> Self {
        self.0 = self.0.and_then(|mut request| {
            let bytes = crate::json::to_vec(to).map_err(Error::json)?;
            request.body = Some(bytes);

            Ok(request)
        });

        self
    }

    /// Set the multipart form.
    pub fn form(mut self, form: Form) -> Self {
        if let Ok(request) = self.0.as_mut() {
            request.form = Some(form);
        }

        self
    }

    /// Whether to use the client's authorization token in the request, if one
    /// is set.
    ///
    /// This is primarily useful for executing webhooks.
    pub const fn use_authorization_token(mut self, use_authorization_token: bool) -> Self {
        if let Ok(request) = self.0.as_mut() {
            request.use_authorization_token = use_authorization_token;
        }

        self
    }
}

#[derive(Clone, Debug)]
pub struct Request {
    pub(crate) body: Option<Vec<u8>>,
    pub(crate) form: Option<Form>,
    pub(crate) headers: Option<HeaderMap<HeaderValue>>,
    pub(crate) method: Method,
    pub(crate) path: Path,
    pub(crate) use_authorization_token: bool,
}

impl Request {
    /// Create a new request builder.
    ///
    /// # Examples
    ///
    /// Create a request to create a message with a content of "test" in a
    /// channel with an ID of 1:
    ///
    /// ```
    /// use twilight_http::{request::Request, routing::Route};
    ///
    /// let body = br#"{
    ///     "content": "test"
    /// }"#
    /// .to_vec();
    ///
    /// let request = Request::builder(&Route::CreateMessage { channel_id: 1 })
    ///     .body(body)
    ///     .build();
    /// ```
    pub fn builder(method: Method, path: Path) -> RequestBuilder {
        RequestBuilder::new(method, path)
    }

    pub fn builder_from_route(route: &Route<'_>) -> RequestBuilder {
        RequestBuilder::new_from_route(route)
    }

    /// # Panics
    ///
    /// Now panics. Doesn't matter, this will be removed
    pub fn from_route(route: &Route<'_>) -> Self {
        use std::str::FromStr as _;

        Self {
            body: None,
            form: None,
            headers: None,
            method: route.method(),
            path: Path::from_str(&route.to_string()).expect("TODO"),
            use_authorization_token: true,
        }
    }

    /// Body of the request, if any.
    pub fn body(&self) -> Option<&[u8]> {
        self.body.as_deref()
    }

    /// Multipart form of the request, if any.
    pub const fn form(&self) -> Option<&Form> {
        self.form.as_ref()
    }

    /// Headers to set in the request, if any.
    pub const fn headers(&self) -> Option<&HeaderMap<HeaderValue>> {
        self.headers.as_ref()
    }

    /// Method when sending the request.
    pub const fn method(&self) -> Method {
        self.method
    }

    /// Immutable reference to the path.
    pub const fn path(&self) -> &Path {
        &self.path
    }

    /// Whether to use the client's authorization token in the request.
    pub const fn use_authorization_token(&self) -> bool {
        self.use_authorization_token
    }
}

#[cfg(test)]
mod tests {
    use super::{Request, RequestBuilder};
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(RequestBuilder: Debug, Send, Sync);
    assert_impl_all!(Request: Clone, Debug, Send, Sync);
}
