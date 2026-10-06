use crate::{
    client::Client,
    error::Error,
    request::{Method, Path, Request, Route, TryIntoRequest},
    response::{Response, ResponseFuture},
};
use std::future::IntoFuture;
use twilight_model::guild::template::Template;

pub struct GetTemplateFields<'a> {
    template_code: &'a str,
}

/// Get a template by its code.
#[must_use = "requests must be configured and executed"]
pub struct GetTemplate<'a> {
    fields: GetTemplateFields<'a>,
    http: &'a Client,
}

impl<'a> GetTemplate<'a> {
    pub(crate) const fn new(http: &'a Client, template_code: &'a str) -> Self {
        Self {
            fields: GetTemplateFields { template_code },
            http,
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

impl<'a> Route for GetTemplate<'a> {
    type Fields = GetTemplateFields<'a>;

    const METHOD: Method = Method::Get;

    fn path(fields: Self::Fields) -> Path {
        Path::builder()
            .resource("guilds")
            .subresource("templates")
            .string_id(fields.template_code)
            .build()
    }
}

impl TryIntoRequest for GetTemplate<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route_new::<Self>(self.fields))
    }
}
