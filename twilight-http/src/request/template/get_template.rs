use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::guild::template::Template;

/// Get a template by its code.
#[must_use = "requests must be configured and executed"]
pub struct GetTemplate<'a> {
    http: &'a Client,
    template_code: &'a str,
}

impl<'a> GetTemplate<'a> {
    pub(crate) const fn new(http: &'a Client, template_code: &'a str) -> Self {
        Self {
            http,
            template_code,
        }
    }
}

impl IntoFuture for GetTemplate<'_> {
    type Output = Result<Response<Template>, Error>;

    type IntoFuture = ResponseFuture<Template>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetTemplate<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetTemplate {
            template_code: self.template_code,
        }))
    }
}
