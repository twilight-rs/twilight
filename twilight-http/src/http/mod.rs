use crate::{Error, error::ErrorType};

#[cfg(not(target_arch = "wasm32"))]
mod hyper;
#[cfg(not(target_arch = "wasm32"))]
pub use super::hyper::{HttpClient, RawRequest, RawResponse, RawResponseFuture};

#[cfg(target_arch = "wasm32")]
mod reqwest;
#[cfg(target_arch = "wasm32")]
pub use self::reqwest::{HttpClient, RawRequest, RawResponseFuture, RawResponse};

// mod worker;

// pub use self::worker::{HttpClient, RawRequestBuilder, RawRequest, RawResponseFuture, RawResponse};

use http::{Method, HeaderMap, HeaderValue, Uri};

pub struct RawRequestBuilder {
    method: Method,
    uri: Uri,
    headers: HeaderMap<HeaderValue>,
    body: Vec<u8>,
}

impl RawRequestBuilder {
    pub fn new() -> Self {
        RawRequestBuilder {
            method: Method::GET,
            uri: Uri::default(),
            headers: HeaderMap::default(),
            body: Vec::new(),
        }
    }

    pub fn method(mut self, method: Method) -> Self {
        self.method = method;

        self
    }

    pub fn uri(mut self, uri: &str) -> Result<Self, Error> {
        let parsed = Uri::try_from(uri).map_err(|source| {
            Error {
                kind: ErrorType::BuildingRequest,
                source: Some(Box::new(source)),
            }
        })?;

        self.uri = parsed;

        Ok(self)
    }

    pub fn headers_mut(&mut self) -> Option<&mut HeaderMap<HeaderValue>> {
        Some(&mut self.headers)
    }

    pub fn body(mut self, body: Vec<u8>) -> Self {
        self.body = body;

        self
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(super) fn build(self) -> Result<tmp::RawRequest, Error> {
        let mut builder = ::hyper::Request::builder().method(self.method).uri(self.uri);
        if let Some(headers) = builder.headers_mut() {
            *headers = self.headers;
        }
        let hyper = builder.body(::hyper::Body::from(self.body)).map_err(|source| {
            Error {
                kind: ErrorType::BuildingRequest,
                source: Some(Box::new(source)),
            }
        })?;
        Ok(tmp::RawRequest { hyper })
    }

    #[cfg(target_arch = "wasm32")]
    pub(super) fn build(self) -> Result<reqwest::RawRequest, Error> {
        let url = ::reqwest::Url::try_from(self.uri.to_string().as_str()).unwrap();
        let mut req = ::reqwest::Request::new(self.method, url);
        *req.headers_mut() = self.headers;
        *req.body_mut() = Some(self.body.into());

        Ok(reqwest::RawRequest { req })
    }
}
