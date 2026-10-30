use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture, marker::ListBody},
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
        let GetGuildIntegrationsPath { guild_id } = self.path;

        let path = Path::builder()
            .resource("guilds")
            .id(guild_id)
            .resource("integrations")
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Get, path))
    }
}
