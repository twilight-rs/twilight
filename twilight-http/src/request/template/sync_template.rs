use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    guild::template::Template,
    id::{marker::GuildMarker, Id},
};

/// Sync a template to the current state of the guild, by ID and code.
#[must_use = "requests must be configured and executed"]
pub struct SyncTemplate<'a> {
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    template_code: &'a str,
}

impl<'a> SyncTemplate<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        template_code: &'a str,
    ) -> Self {
        Self {
            guild_id,
            http,
            template_code,
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

impl TryIntoRequest for SyncTemplate<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::SyncTemplate {
            guild_id: self.guild_id.get(),
            template_code: self.template_code,
        }))
    }
}
