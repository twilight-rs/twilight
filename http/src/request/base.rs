use super::{Form, Method};
use crate::{
    error::Error,
    routing::{Path, Route},
};
use hyper::header::{HeaderMap, HeaderName, HeaderValue};
use serde::Serialize;
use std::borrow::Cow;

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
/// }"#.to_vec();
///
/// let request = Request::builder(Route::CreateMessage {
///     channel_id: 1,
/// }).body(body).build();
/// ```
#[derive(Debug)]
pub struct RequestBuilder(Request);

impl RequestBuilder {
    /// Create a new request builder.
    #[must_use = "request has not been fully built"]
    pub fn new(route: Route) -> Self {
        Self(Request::from_route(route))
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
    /// use twilight_http::{request::{Method, RequestBuilder}, routing::Path};
    ///
    /// let method = Method::Post;
    /// let path_and_query = "channels/123/pins".to_owned();
    /// let ratelimit_path = Path::from_str(&path_and_query)?;
    ///
    /// let _request = RequestBuilder::raw(
    ///     method,
    ///     ratelimit_path,
    ///     path_and_query,
    /// ).build();
    /// # Ok(()) }
    /// ```
    #[must_use = "request has not been fully built"]
    pub const fn raw(method: Method, path: Path, path_and_query: String) -> Self {
        Self(Request {
            body: None,
            form: None,
            headers: None,
            method,
            path,
            path_str: Cow::Owned(path_and_query),
            use_authorization_token: true,
        })
    }

    /// Consume the builder, returning the built request.
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "request information is not useful on its own and must be acted on"]
    pub fn build(self) -> Request {
        self.0
    }

    /// Set the contents of the body.
    #[must_use = "request has not been fully built"]
    pub fn body(mut self, body: Vec<u8>) -> Self {
        self.0.body.replace(body);

        self
    }

    /// Set the multipart form.
    #[must_use = "request has not been fully built"]
    pub fn form(mut self, form: Form) -> Self {
        self.0.form.replace(form);

        self
    }

    /// Set the headers to add.
    #[must_use = "request has not been fully built"]
    pub fn headers(mut self, iter: impl Iterator<Item = (HeaderName, HeaderValue)>) -> Self {
        self.0.headers.replace(iter.collect());

        self
    }

    /// Set the body, to be serialized as JSON.
    ///
    /// # Errors
    ///
    /// Returns an [`ErrorType::Json`] error type if the value could not be
    /// serialized as JSON.
    ///
    /// [`ErrorType::Json`]: crate::error::ErrorType::Json
    #[must_use = "request has not been fully built"]
    pub fn json(self, to: &impl Serialize) -> Result<Self, Error> {
        let bytes = crate::json::to_vec(to).map_err(Error::json)?;

        Ok(self.body(bytes))
    }

    /// Whether to use the client's authorization token in the request, if one
    /// is set.
    ///
    /// This is primarily useful for executing webhooks.
    pub const fn use_authorization_token(mut self, use_authorization_token: bool) -> Self {
        self.0.use_authorization_token = use_authorization_token;

        self
    }
}

#[derive(Debug)]
pub struct Request {
    /// The body of the request, if any.
    pub body: Option<Vec<u8>>,
    /// The multipart form of the request, if any.
    pub form: Option<Form>,
    /// The headers to set in the request, if any.
    pub headers: Option<HeaderMap<HeaderValue>>,
    /// The method of the request.
    pub method: Method,
    /// The ratelimiting bucket path.
    pub path: Path,
    /// The URI path to request.
    pub path_str: Cow<'static, str>,
    /// Whether to use the client's authorization token in the request.
    pub(crate) use_authorization_token: bool,
}

impl Request {
    /// Create a simple `Request` with basic information.
    ///
    /// Use the [`RequestBuilder`] if you need to set a combination of
    /// configurations in the request.
    // `Route`'s methods have been changed to no longer consume itself, so we
    // could pass the route by reference but we need to avoid breakage.
    #[allow(clippy::needless_pass_by_value)]
    #[deprecated(since = "0.4.0", note = "Use `Request::builder` instead")]
    pub fn new(
        body: Option<Vec<u8>>,
        headers: Option<HeaderMap<HeaderValue>>,
        route: Route,
    ) -> Self {
        Self {
            body,
            form: None,
            headers,
            method: route.method(),
            path: route.path(),
            path_str: Cow::Owned(route.display().to_string()),
            use_authorization_token: true,
        }
    }

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
    /// }"#.to_vec();
    ///
    /// let request = Request::builder(Route::CreateMessage {
    ///     channel_id: 1,
    /// }).body(body).build();
    /// ```
    pub fn builder(route: Route) -> RequestBuilder {
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
    /// let request = Request::from_route(Route::GetMessage {
    ///     channel_id: 1,
    ///     message_id: 2,
    /// });
    /// ```
    ///
    /// [`builder`]: Self::builder
    // `Route`'s methods have been changed to no longer consume itself, so we
    // could pass the route by reference but we need to avoid breakage.
    #[allow(clippy::needless_pass_by_value)]
    pub fn from_route(route: Route) -> Self {
        Self {
            body: None,
            form: None,
            headers: None,
            method: route.method(),
            path: route.path(),
            path_str: Cow::Owned(route.display().to_string()),
            use_authorization_token: true,
        }
    }

    /// Whether to use the client's authorization token in the request.
    pub const fn use_authorization_token(&self) -> bool {
        self.use_authorization_token
    }
}

impl From<Route> for Request {
    fn from(route: Route) -> Self {
        Self {
            body: None,
            form: None,
            headers: None,
            method: route.method(),
            path: route.path(),
            path_str: Cow::Owned(route.display().to_string()),
            use_authorization_token: true,
        }
    }
}

impl From<(Vec<u8>, Route)> for Request {
    fn from((body, route): (Vec<u8>, Route)) -> Self {
        Self {
            body: Some(body),
            form: None,
            headers: None,
            method: route.method(),
            path: route.path(),
            path_str: Cow::Owned(route.display().to_string()),
            use_authorization_token: true,
        }
    }
}

impl From<(Form, Route)> for Request {
    fn from((form, route): (Form, Route)) -> Self {
        Self {
            body: None,
            form: Some(form),
            headers: None,
            method: route.method(),
            path: route.path(),
            path_str: Cow::Owned(route.display().to_string()),
            use_authorization_token: true,
        }
    }
}

impl From<(Vec<u8>, Form, Route)> for Request {
    fn from((body, form, route): (Vec<u8>, Form, Route)) -> Self {
        Self {
            body: Some(body),
            form: Some(form),
            headers: None,
            method: route.method(),
            path: route.path(),
            path_str: Cow::Owned(route.display().to_string()),
            use_authorization_token: true,
        }
    }
}

impl From<(HeaderMap<HeaderValue>, Route)> for Request {
    fn from((headers, route): (HeaderMap<HeaderValue>, Route)) -> Self {
        Self {
            body: None,
            form: None,
            headers: Some(headers),
            method: route.method(),
            path: route.path(),
            path_str: Cow::Owned(route.display().to_string()),
            use_authorization_token: true,
        }
    }
}

impl From<(Vec<u8>, HeaderMap<HeaderValue>, Route)> for Request {
    fn from((body, headers, route): (Vec<u8>, HeaderMap<HeaderValue>, Route)) -> Self {
        Self {
            body: Some(body),
            form: None,
            headers: Some(headers),
            method: route.method(),
            path: route.path(),
            path_str: Cow::Owned(route.display().to_string()),
            use_authorization_token: true,
        }
    }
}

impl From<(Form, HeaderMap<HeaderValue>, Route)> for Request {
    fn from((form, headers, route): (Form, HeaderMap<HeaderValue>, Route)) -> Self {
        Self {
            body: None,
            form: Some(form),
            headers: Some(headers),
            method: route.method(),
            path: route.path(),
            path_str: Cow::Owned(route.display().to_string()),
            use_authorization_token: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{super::Method, RequestBuilder};
    use crate::routing::Path;
    use static_assertions::assert_impl_all;
    use std::{error::Error, fmt::Debug, str::FromStr};

    assert_impl_all!(RequestBuilder: Debug, Send, Sync);

    /// Test the default request values from [`RequestBuilder::raw`].
    #[test]
    fn test_builder_raw() -> Result<(), Box<dyn Error>> {
        let path_and_query = "guilds".to_owned();
        let path = Path::from_str(&path_and_query)?;

        let builder = RequestBuilder::raw(Method::Post, path, path_and_query);
        assert!(builder.0.body.is_none());
        assert!(builder.0.form.is_none());
        assert!(builder.0.headers.is_none());
        assert_eq!(Method::Post, builder.0.method);
        assert_eq!(Path::Guilds, builder.0.path);
        assert_eq!("guilds", builder.0.path_str.as_ref());
        assert!(builder.0.use_authorization_token);

        Ok(())
    }
}
