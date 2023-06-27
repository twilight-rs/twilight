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
use worker_sys::web_sys::WorkerGlobalScope;

use crate::{error::ErrorType, Error, response::{BytesFuture, StatusCode}};

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
        let inner = fetch_with_request(req.request);
        RawResponseFuture { inner: Box::new(inner) }
    }
}

// todo(erk): This struct needs to contain a enum since we need to
//            read all the data into containers that is Send (+Sync)
//            since the worker::Response struct is not.

#[derive(Debug)]
pub struct RawResponseFuture {
    inner: Box<dyn Future<Output = Result<Response, ::worker::Error>>>,
}

impl RawResponseFuture {
    fn inner(&mut self)
             -> &mut impl Future<Output = Result<Response, ::worker::Error>> {
        self.inner
    }
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
        BytesFuture::from_worker(self.resp)
    }

    pub fn status(&self) -> StatusCode {
        StatusCode::new(self.resp.status_code())
    }

}

async fn fetch_with_request(request: Request) -> Result<Response, Error> {
    worker::Fetch::Request(request).send().await
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

// fn to_response(resp: JsValue) -> Result<Response, Error> {
//     let edge_response: Response = resp.dyn_into().map_err(|_source| Error {
//         kind: ErrorType::RequestError,
//         source: None,
//     })?;
//     Ok(edge_response.into())
// }
