use super::{Form, Method};
use crate::{
    error::Error,
    routing::{Path, Route},
};
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
    pub fn new(route: &Route<'_>) -> Self {
        Self(Ok(Request::from_route(route)))
    }

    /// Create a request with raw information about the method, ratelimiting
    /// path, and URL path and query.
    ///
    /// The path and query should not include the leading slash as that is
    /// prefixed by the client. In the URL
    /// `https://discord.com/api/vX/channels/123/pins` the "path and query"
    /// is considered to be `channels/123/pins`.
    ///
    /// # Examples
    ///
    /// Create a request from a method and the URL path and query
    /// `channels/123/pins`:
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use std::str::FromStr;
    /// use twilight_http::{
    ///     request::{Method, RequestBuilder},
    ///     routing::Path,
    /// };
    ///
    /// let method = Method::Post;
    /// let path_and_query = "channels/123/pins".to_owned();
    /// let ratelimit_path = Path::from_str(&path_and_query)?;
    ///
    /// let _request = RequestBuilder::raw(method, ratelimit_path, path_and_query).build();
    /// # Ok(()) }
    /// ```
    pub const fn raw(method: Method, ratelimit_path: Path, path_and_query: String) -> Self {
        Self(Ok(Request {
            body: None,
            form: None,
            headers: None,
            method,
            path: path_and_query,
            ratelimit_path,
            use_authorization_token: true,
        }))
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

    /// Set the multipart form.
    #[allow(clippy::missing_const_for_fn)]
    pub fn form(mut self, form: Form) -> Self {
        if let Ok(request) = self.0.as_mut() {
            request.form = Some(form);
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

    /// Whether to use the client's authorization token in the request, if one
    /// is set.
    ///
    /// This is primarily useful for executing webhooks.
    pub fn use_authorization_token(mut self, use_authorization_token: bool) -> Self {
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
    pub(crate) path: String,
    pub(crate) ratelimit_path: Path,
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
    pub fn builder(route: &Route<'_>) -> RequestBuilder {
        RequestBuilder::new(route)
    }

    /// Create a request from only its route information.
    ///
    /// If you need to set additional configurations like the body then use
    /// [`builder`].
    ///
    /// # Examples
    ///
    /// Create a request to get a message with an ID of 2 in a channel with an
    /// ID of 1:
    ///
    /// ```
    /// use twilight_http::{request::Request, routing::Route};
    ///
    /// let request = Request::from_route(&Route::GetMessage {
    ///     channel_id: 1,
    ///     message_id: 2,
    /// });
    /// ```
    ///
    /// [`builder`]: Self::builder
    pub fn from_route(route: &Route<'_>) -> Self {
        Self {
            body: None,
            form: None,
            headers: None,
            method: route.method(),
            path: route.to_string(),
            ratelimit_path: route.to_path(),
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

    /// String path of the full URL.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Path used for ratelimiting.
    pub const fn ratelimit_path(&self) -> &Path {
        &self.ratelimit_path
    }

    /// Whether to use the client's authorization token in the request.
    pub const fn use_authorization_token(&self) -> bool {
        self.use_authorization_token
    }
}

#[cfg(test)]
mod tests {
    use super::RequestBuilder;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(RequestBuilder: Debug, Send, Sync);
}
