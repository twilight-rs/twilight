use crate::{
    client::Client,
    error::Error,
    request::{Method, Path, Request, Route, TryIntoRequest},
    response::{Response, ResponseFuture, marker::EmptyBody},
};
use std::future::IntoFuture;
use twilight_model::id::{Id, marker::GuildMarker};

pub struct DeleteTemplateFields<'a> {
    guild_id: Id<GuildMarker>,
    template_code: &'a str,
}

/// Delete a template by ID and code.
#[must_use = "requests must be configured and executed"]
pub struct DeleteTemplate<'a> {
    fields: DeleteTemplateFields<'a>,
    http: &'a Client,
}

impl<'a> DeleteTemplate<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        template_code: &'a str,
    ) -> Self {
        Self {
            fields: DeleteTemplateFields {
                guild_id,
                template_code,
            },
            http,
        }
    }
}

impl IntoFuture for DeleteTemplate<'_> {
    type Output = Result<Response<EmptyBody>, Error>;

    type IntoFuture = ResponseFuture<EmptyBody>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl<'a> Route for DeleteTemplate<'a> {
    type Fields = DeleteTemplateFields<'a>;

    const METHOD: Method = Method::Delete;

    fn path(fields: Self::Fields) -> Path {
        Path::builder()
            .resource("guilds")
            .id(fields.guild_id)
            .resource("templates")
            .string_id(fields.template_code)
            .build()
    }
}

impl TryIntoRequest for DeleteTemplate<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route_new::<Self>(self.fields))
    }
}
