use std::{future::Future, pin::Pin};

use http::{HeaderValue, header::HeaderMap};
use hyper::Body;

use crate::response::{BytesFuture, StatusCode};

#[derive(Debug)]
pub struct RawResponse {
    pub inner: hyper::Response<Body>,
}

impl RawResponse {
    fn new(inner: hyper::Response<Body>) -> Self {
        RawResponse { inner }
    }

    pub fn headers(&self) -> &HeaderMap<HeaderValue> {
        self.inner.headers()
    }

    pub fn headers_mut(&mut self) -> &mut HeaderMap<HeaderValue> {
        self.inner.headers_mut()
    }

    pub fn bytes(self, compressed: bool) -> BytesFuture {
        BytesFuture::from_hyper(self.inner.into_body(), compressed)
    }

    pub fn status(&self) -> StatusCode {
        // Convert the `hyper` status code into its raw form in order to return
        // our own.
        let raw = self.inner.status().as_u16();

        StatusCode::new(raw)
    }
}

pub struct RawResponseFuture {
    pub inner: hyper::client::ResponseFuture,
}

impl Future for RawResponseFuture {
    type Output = hyper::Result<RawResponse>;

    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        Pin::new(&mut self.inner).poll(cx).map_ok(RawResponse::new)
    }
}
