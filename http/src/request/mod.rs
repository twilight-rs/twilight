macro_rules! poll_req {
    ($ty: ty, $ret: ty) => {
        impl std::future::Future for $ty {
            type Output = $crate::error::Result<$ret>;

            fn poll(
                mut self: std::pin::Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
            ) -> ::std::task::Poll<Self::Output> {
                loop {
                    if let Some(fut) = self.as_mut().fut.as_mut() {
                        return fut.as_mut().poll(cx);
                    }

                    if let Err(why) = self.as_mut().start() {
                        return std::task::Poll::Ready(Err(why));
                    }
                }
            }
        }
    };

    (opt, $ty: ty, $ret: ty) => {
        impl std::future::Future for $ty {
            type Output = $crate::error::Result<Option<$ret>>;

            fn poll(
                mut self: std::pin::Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
            ) -> ::std::task::Poll<Self::Output> {
                use crate::json_from_slice;
                use std::task::Poll;

                loop {
                    if let Some(fut) = self.as_mut().fut.as_mut() {
                        let bytes = match fut.as_mut().poll(cx) {
                            Poll::Ready(Ok(bytes)) => bytes,
                            Poll::Ready(Err(e))
                                if matches!(e.kind, crate::error::ErrorType::Response { status, .. } if status == hyper::StatusCode::NOT_FOUND) =>
                            {
                                return Poll::Ready(Ok(None));
                            }
                            Poll::Ready(Err(why)) => return Poll::Ready(Err(why)),
                            Poll::Pending => return Poll::Pending,
                        };

                        let mut bytes = bytes.as_ref().to_vec();
                        return Poll::Ready(json_from_slice(&mut bytes).map(Some).map_err(
                            |source| crate::Error {
                                kind: crate::error::ErrorType::Parsing {
                                    body: bytes.to_vec(),
                                },
                                source: Some(Box::new(source)),
                            },
                        ));
                    }

                    if let Err(why) = self.as_mut().start() {
                        return Poll::Ready(Err(why));
                    }
                }
            }
        }
    };
}

pub mod channel;
pub mod guild;
pub mod prelude;
pub mod template;
pub mod user;

mod audit_reason;
mod get_gateway;
mod get_gateway_authed;
mod get_user_application;
mod get_voice_regions;
mod multipart;
mod validate;

pub use self::{
    audit_reason::{AuditLogReason, AuditLogReasonError},
    get_gateway::GetGateway,
    get_gateway_authed::GetGatewayAuthed,
    get_user_application::GetUserApplicationInfo,
    get_voice_regions::GetVoiceRegions,
};

use self::multipart::Form;
use crate::{
    error::{Error, ErrorType, Result},
    routing::{Path, Route},
};
use bytes::Bytes;
use hyper::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Method as HyperMethod,
};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use std::{borrow::Cow, future::Future, pin::Pin};

type Pending<'a, T> = Pin<Box<dyn Future<Output = Result<T>> + Send + 'a>>;
type PendingOption<'a> = Pin<Box<dyn Future<Output = Result<Bytes>> + Send + 'a>>;

/// Request method.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum Method {
    /// DELETE method.
    Delete,
    /// GET method.
    Get,
    /// PATCH method.
    Patch,
    /// POST method.
    Post,
    /// PUT method.
    Put,
}

impl Method {
    pub(crate) fn into_hyper(self) -> HyperMethod {
        match self {
            Self::Delete => HyperMethod::DELETE,
            Self::Get => HyperMethod::GET,
            Self::Patch => HyperMethod::PATCH,
            Self::Post => HyperMethod::POST,
            Self::Put => HyperMethod::PUT,
        }
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
}

pub(crate) fn audit_header(reason: &str) -> Result<HeaderMap<HeaderValue>> {
    let header_name = HeaderName::from_static("x-audit-log-reason");
    let mut headers = HeaderMap::new();
    let encoded_reason = utf8_percent_encode(reason, NON_ALPHANUMERIC).to_string();
    let header_value = HeaderValue::from_str(&encoded_reason).map_err(|e| Error {
        kind: ErrorType::CreatingHeader {
            name: encoded_reason.clone(),
        },
        source: Some(Box::new(e)),
    })?;

    headers.insert(header_name, header_value);

    Ok(headers)
}

impl Request {
    /// Create a simple `Request` with basic information.
    ///
    /// Use the various `From` implementations for different combinations of
    /// configurations.
    pub fn new(
        body: Option<Vec<u8>>,
        headers: Option<HeaderMap<HeaderValue>>,
        route: Route,
    ) -> Self {
        let (method, path, path_str) = route.into_parts();

        Self {
            body,
            form: None,
            headers,
            method,
            path,
            path_str,
        }
    }
}

impl From<Route> for Request {
    fn from(route: Route) -> Self {
        let (method, path, path_str) = route.into_parts();

        Self {
            body: None,
            form: None,
            headers: None,
            method,
            path,
            path_str,
        }
    }
}

impl From<(Vec<u8>, Route)> for Request {
    fn from((body, route): (Vec<u8>, Route)) -> Self {
        let (method, path, path_str) = route.into_parts();

        Self {
            body: Some(body),
            form: None,
            headers: None,
            method,
            path,
            path_str,
        }
    }
}

impl From<(Form, Route)> for Request {
    fn from((form, route): (Form, Route)) -> Self {
        let (method, path, path_str) = route.into_parts();

        Self {
            body: None,
            form: Some(form),
            headers: None,
            method,
            path,
            path_str,
        }
    }
}

impl From<(Vec<u8>, Form, Route)> for Request {
    fn from((body, form, route): (Vec<u8>, Form, Route)) -> Self {
        let (method, path, path_str) = route.into_parts();

        Self {
            body: Some(body),
            form: Some(form),
            headers: None,
            method,
            path,
            path_str,
        }
    }
}

impl From<(HeaderMap<HeaderValue>, Route)> for Request {
    fn from((headers, route): (HeaderMap<HeaderValue>, Route)) -> Self {
        let (method, path, path_str) = route.into_parts();

        Self {
            body: None,
            form: None,
            headers: Some(headers),
            method,
            path,
            path_str,
        }
    }
}

impl From<(Vec<u8>, HeaderMap<HeaderValue>, Route)> for Request {
    fn from((body, headers, route): (Vec<u8>, HeaderMap<HeaderValue>, Route)) -> Self {
        let (method, path, path_str) = route.into_parts();

        Self {
            body: Some(body),
            form: None,
            headers: Some(headers),
            method,
            path,
            path_str,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Method;
    use hyper::Method as HyperMethod;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(Method: Clone, Copy, Debug, Eq, PartialEq);

    #[test]
    fn test_method_conversions() {
        assert_eq!(HyperMethod::DELETE, Method::Delete.into_hyper());
        assert_eq!(HyperMethod::GET, Method::Get.into_hyper());
        assert_eq!(HyperMethod::PATCH, Method::Patch.into_hyper());
        assert_eq!(HyperMethod::POST, Method::Post.into_hyper());
        assert_eq!(HyperMethod::PUT, Method::Put.into_hyper());
    }
}
