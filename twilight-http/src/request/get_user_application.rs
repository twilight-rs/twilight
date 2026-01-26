#[cfg(not(target_os = "wasi"))]
use crate::response::{Response, ResponseFuture};
use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::oauth::Application;

#[must_use = "requests must be configured and executed"]
pub struct GetUserApplicationInfo<'a> {
    http: &'a Client,
}

impl<'a> GetUserApplicationInfo<'a> {
    pub(crate) const fn new(http: &'a Client) -> Self {
        Self { http }
    }
}

#[cfg(not(target_os = "wasi"))]
impl IntoFuture for GetUserApplicationInfo<'_> {
    type Output = Result<Response<Application>, Error>;

    type IntoFuture = ResponseFuture<Application>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetUserApplicationInfo<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetCurrentUserApplicationInfo))
    }
}
