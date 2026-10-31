use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture},
};
use std::future::IntoFuture;
use twilight_model::guild::template::Template;

pub struct GetTemplatePath<'a> {
    template_code: &'a str,
}

/// Get a template by its code.
#[must_use = "requests must be configured and executed"]
pub struct GetTemplate<'a> {
    http: &'a Client,
    path: GetTemplatePath<'a>,
}

impl<'a> GetTemplate<'a> {
    pub(crate) const fn new(http: &'a Client, template_code: &'a str) -> Self {
        Self {
            http,
            path: GetTemplatePath { template_code },
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
        let path = Path::builder()
            .resource("guilds")
            .subresource("templates")
            .id(self.path.template_code)
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Get, path))
    }
}
