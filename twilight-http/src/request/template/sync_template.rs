use crate::{
    client::Client,
    error::Error,
    request::{Method, Path, Request, Route, TryIntoRequest},
    response::{Response, ResponseFuture},
};
use std::future::IntoFuture;
use twilight_model::{
    guild::template::Template,
    id::{Id, marker::GuildMarker},
};

pub struct SyncTemplateFields<'a> {
    guild_id: Id<GuildMarker>,
    template_code: &'a str,
}

/// Sync a template to the current state of the guild, by ID and code.
#[must_use = "requests must be configured and executed"]
pub struct SyncTemplate<'a> {
    fields: SyncTemplateFields<'a>,
    http: &'a Client,
}

impl<'a> SyncTemplate<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        template_code: &'a str,
    ) -> Self {
        Self {
            fields: SyncTemplateFields {
                guild_id,
                template_code,
            },
            http,
        }
    }
}

impl IntoFuture for SyncTemplate<'_> {
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

impl<'a> Route for SyncTemplate<'a> {
    type Fields = SyncTemplateFields<'a>;

    const METHOD: Method = Method::Put;

    fn path(fields: Self::Fields) -> Path {
        Path::builder()
            .resource("guilds")
            .id(fields.guild_id)
            .resource("templates")
            .string_id(fields.template_code)
            .build()
    }
}

impl TryIntoRequest for SyncTemplate<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route_new::<Self>(self.fields))
    }
}
