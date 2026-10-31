use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture, marker::ListBody},
};
use std::future::IntoFuture;
use twilight_model::{
    channel::Webhook,
    id::{Id, marker::GuildMarker},
};

struct GetGuildWebhooksPath {
    guild_id: Id<GuildMarker>,
}

/// Get the webhooks of a guild.
#[must_use = "requests must be configured and executed"]
pub struct GetGuildWebhooks<'a> {
    path: GetGuildWebhooksPath,
    http: &'a Client,
}

impl<'a> GetGuildWebhooks<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self {
            path: GetGuildWebhooksPath { guild_id },
            http,
        }
    }
}

impl IntoFuture for GetGuildWebhooks<'_> {
    type Output = Result<Response<ListBody<Webhook>>, Error>;

    type IntoFuture = ResponseFuture<ListBody<Webhook>>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetGuildWebhooks<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let GetGuildWebhooksPath { guild_id } = self.path;

        let path = Path::builder()
            .resource("guilds")
            .id(guild_id)
            .resource("webhooks")
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Get, path))
    }
}
