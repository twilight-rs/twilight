use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture, marker::ListBody},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    guild::GuildIntegration,
    id::{Id, marker::GuildMarker},
};

struct GetGuildIntegrationsPath {
    guild_id: Id<GuildMarker>,
}

/// Get the guild's integrations.
///
/// This endpoint returns a maximum of 50 integrations. If a guild has more
/// integrations then they can't be accessed.
#[must_use = "requests must be configured and executed"]
pub struct GetGuildIntegrations<'a> {
    http: &'a Client,
    path: GetGuildIntegrationsPath,
}

impl<'a> GetGuildIntegrations<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self {
            http,
            path: GetGuildIntegrationsPath { guild_id },
        }
    }
}

impl IntoFuture for GetGuildIntegrations<'_> {
    type Output = Result<Response<ListBody<GuildIntegration>>, Error>;

    type IntoFuture = ResponseFuture<ListBody<GuildIntegration>>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetGuildIntegrations<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetGuildIntegrations {
            guild_id: self.path.guild_id.get(),
        }))
    }
}
