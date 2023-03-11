use std::{pin::Pin, future::Future, task::{Context, Poll}};

use http::{HeaderMap, HeaderValue};

use crate::{response::{BytesFuture, StatusCode}, Error, error::ErrorType};

#[derive(Debug)]
pub struct HttpClient {
    pub(crate) reqwest: reqwest::Client,
}

impl HttpClient {
    pub fn new() -> Self {
        HttpClient { reqwest: reqwest::Client::new() }
    }

    pub fn request(&self, req: RawRequest) -> RawResponseFuture {
        let inner = Box::pin(self.reqwest.execute(req.req));
        RawResponseFuture { inner }
    }
}

pub struct RawRequest {
    pub(crate) req: ::reqwest::Request,
}

pub struct RawResponseFuture {
    inner: Pin<Box<dyn Future<Output = Result<reqwest::Response, reqwest::Error>>>>,
}

impl Future for RawResponseFuture {
    type Output = Result<RawResponse, Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map_ok(RawResponse::new)
            .map_err(|source| Error {
                kind: ErrorType::RequestError,
                source: Some(Box::new(source)),
            })
    }
}

#[derive(Debug)]
pub struct RawResponse {
    pub inner: reqwest::Response,
}

impl RawResponse {
    fn new(inner: reqwest::Response) -> Self {
        RawResponse { inner }
    }

    pub fn headers(&self) -> &HeaderMap<HeaderValue> {
        self.inner.headers()
    }

    pub fn headers_mut(&mut self) -> &mut HeaderMap<HeaderValue> {
        self.inner.headers_mut()
    }

    pub fn bytes(self, compressed: bool) -> BytesFuture {
        BytesFuture::from_reqwest(self.inner)
    }

    pub fn status(&self) -> StatusCode {
        // Convert the `hyper` status code into its raw form in order to return
        // our own.
        let raw = self.inner.status().as_u16();

        StatusCode::new(raw)
    }
}
