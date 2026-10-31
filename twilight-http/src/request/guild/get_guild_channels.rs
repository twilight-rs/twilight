use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture, marker::ListBody},
};
use std::future::IntoFuture;
use twilight_model::{
    channel::Channel,
    id::{Id, marker::GuildMarker},
};

struct GetGuildChannelsPath {
    guild_id: Id<GuildMarker>,
}

/// Get the channels in a guild.
///
/// # Channel Obfuscation
///
/// Channels the user does not have access to will not be returned via this
/// endpoint once Discord finalizes the rollout of Channel Obfuscation on or
/// around November 16th, 2026.
#[must_use = "requests must be configured and executed"]
pub struct GetGuildChannels<'a> {
    path: GetGuildChannelsPath,
    http: &'a Client,
}

impl<'a> GetGuildChannels<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self {
            path: GetGuildChannelsPath { guild_id },
            http,
        }
    }
}

impl IntoFuture for GetGuildChannels<'_> {
    type Output = Result<Response<ListBody<Channel>>, Error>;

    type IntoFuture = ResponseFuture<ListBody<Channel>>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetGuildChannels<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let GetGuildChannelsPath { guild_id } = self.path;

        let path = Path::builder()
            .resource("guilds")
            .id(guild_id)
            .resource("channels")
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Get, path))
    }
}
