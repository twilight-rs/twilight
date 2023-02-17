use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{marker::ListBody, Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    guild::template::Template,
    id::{marker::GuildMarker, Id},
};

/// Get a list of templates in a guild, by ID.
#[must_use = "requests must be configured and executed"]
pub struct GetTemplates<'a> {
    guild_id: Id<GuildMarker>,
    http: &'a Client,
}

impl<'a> GetTemplates<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self { guild_id, http }
    }
}

impl IntoFuture for GetTemplates<'_> {
    type Output = Result<Response<ListBody<Template>>, Error>;

    type IntoFuture = ResponseFuture<ListBody<Template>>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetTemplates<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetTemplates {
            guild_id: self.guild_id.get(),
        }))
    }
}
