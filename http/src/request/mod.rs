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
                use std::task::Poll;

                loop {
                    if let Some(fut) = self.as_mut().fut.as_mut() {
                        let bytes = match fut.as_mut().poll(cx) {
                            Poll::Ready(Ok(bytes)) => bytes,
                            Poll::Ready(Err(crate::Error::Response { status, .. }))
                                if status == isahc::http::StatusCode::NOT_FOUND =>
                            {
                                return Poll::Ready(Ok(None));
                            }
                            Poll::Ready(Err(why)) => return Poll::Ready(Err(why)),
                            Poll::Pending => return Poll::Pending,
                        };

                        return Poll::Ready(serde_json::from_slice(&bytes).map(Some).map_err(
                            |source| crate::Error::Parsing {
                                body: bytes.to_vec(),
                                source,
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
pub mod user;

mod get_gateway;
mod get_gateway_authed;
mod get_voice_regions;
mod multipart;
mod validate;

pub use self::{
    get_gateway::GetGateway, get_gateway_authed::GetGatewayAuthed,
    get_voice_regions::GetVoiceRegions,
};

use crate::{
    error::{Error, Result},
    routing::{Path, Route},
};
use common_multipart_rfc7578::client::multipart::Form;
use isahc::http::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Method,
};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use std::fmt::{Debug, Formatter, Result as FmtResult};

use std::{borrow::Cow, future::Future, pin::Pin};

type Pending<'a, T> = Pin<Box<dyn Future<Output = Result<T>> + Send + 'a>>;
type PendingOption<'a> = Pin<Box<dyn Future<Output = Result<Vec<u8>>> + Send + 'a>>;

pub struct Request {
    pub body: Option<Vec<u8>>,
    pub form: Option<Form<'static>>,
    pub headers: Option<HeaderMap<HeaderValue>>,
    pub method: Method,
    pub path: Path,
    pub path_str: Cow<'static, str>,
}

pub(crate) fn audit_header(reason: &str) -> Result<HeaderMap<HeaderValue>> {
    let header_name = HeaderName::from_static("x-audit-log-reason");
    let mut headers = HeaderMap::new();
    let encoded_reason = utf8_percent_encode(reason, NON_ALPHANUMERIC).to_string();
    let header_value =
        HeaderValue::from_str(&encoded_reason).map_err(|e| Error::CreatingHeader {
            name: encoded_reason.clone(),
            source: e,
        })?;

    headers.insert(header_name, header_value);

    Ok(headers)
}

impl Request {
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

impl Debug for Request {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("Request")
            .field("body", &self.body)
            .field("headers", &self.headers)
            .field("method", &self.method)
            .field("path", &self.path)
            .field("path_str", &self.path_str)
            .finish()
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

impl From<(Vec<u8>, Form<'static>, Route)> for Request {
    fn from((body, form, route): (Vec<u8>, Form<'static>, Route)) -> Self {
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
