use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use http::{HeaderMap, HeaderValue, Method, Uri};
use worker::{
    js_sys,
    wasm_bindgen::{JsCast, JsValue},
    wasm_bindgen_futures::JsFuture,
    Headers, Request, RequestInit, Response,
};

use crate::{error::ErrorType, Error, response::{BytesFuture, StatusCode}};

#[derive(Debug)]
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
        let parsed = Uri::try_from(uri).map_err(|source| Error {
            kind: ErrorType::BuildingRequest,
            source: Some(Box::new(source)),
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

    pub fn build(self) -> Result<RawRequest, Error> {
        let mut init = RequestInit::new();
        let body = std::str::from_utf8(&self.body).map_err(|source| Error {
            kind: ErrorType::BuildingRequest,
            source: Some(Box::new(source)),
        })?;
        init.body = Some(JsValue::from_str(body));

        let headers = Headers::from(&self.headers);
        init.headers = headers;

        let method = worker::Method::from(self.method.to_string());
        init.method = method;

        let request =
            Request::new_with_init(&self.uri.to_string(), &init).map_err(|_source| Error {
                kind: ErrorType::BuildingRequest,
                source: None,
            })?;

        Ok(RawRequest { request })
    }
}

#[derive(Debug)]
pub struct RawRequest {
    request: Request,
}

#[derive(Debug)]
pub struct HttpClient {}

impl HttpClient {
    pub fn new() -> Self {
        HttpClient {}
    }

    pub fn request(&self, req: RawRequest) -> RawResponseFuture {
        let inner = fetch_with_request(&req.request);
        RawResponseFuture { inner }
    }
}

// todo(erk): This struct needs to contain a enum since we need to
//            read all the data into containers that is Send (+Sync)
//            since the worker::Response struct is not.

#[derive(Debug)]
pub struct RawResponseFuture {
    inner: JsFuture,
}

impl Future for RawResponseFuture {
    type Output = Result<RawResponse, Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|jsval| from_result(jsval))
    }
}

#[derive(Debug)]
pub struct RawResponse {
    resp: Response,
}

impl RawResponse {
    fn new(resp: Response) -> Self {
        RawResponse { resp }
    }

    pub fn headers(&self) -> &HeaderMap<HeaderValue> {
        todo!()
    }

    pub fn headers_mut(&mut self) -> &mut HeaderMap<HeaderValue> {
        todo!()
    }

    pub fn bytes(self, compressed: bool) -> BytesFuture {
        todo!()
    }

    pub fn status(&self) -> StatusCode {
        todo!()
    }

}

use worker_sys::{RequestInit as EdgeRequestInit, Response as EdgeResponse, WorkerGlobalScope};
fn fetch_with_request(request: &Request) -> JsFuture {
    let mut init = EdgeRequestInit::new();

    let worker: WorkerGlobalScope = js_sys::global().unchecked_into();
    let req = request.inner();
    let promise = worker.fetch_with_request_and_init(req, &init);
    JsFuture::from(promise)
}

fn from_result(res: Result<JsValue, JsValue>) -> Result<RawResponse, Error> {
    match res {
        Ok(r) => to_response(r).map(RawResponse::new),
        Err(_why) => Err(Error {
            kind: ErrorType::RequestError,
            source: None,
        }),
    }
}

fn to_response(resp: JsValue) -> Result<Response, Error> {
    let edge_response: EdgeResponse = resp.dyn_into().map_err(|_source| Error {
        kind: ErrorType::RequestError,
        source: None,
    })?;
    Ok(edge_response.into())
}
