use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture, marker::ListBody},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    channel::Channel,
    id::{Id, marker::GuildMarker},
};

/// Get the channels in a guild.
///
/// # Channel Obfuscation
///
/// Channels the user does not have access to will not be returned via this
/// endpoint once Discord finalizes the rollout of Channel Obfuscation on or
/// around November 16th, 2026.
#[must_use = "requests must be configured and executed"]
pub struct GetGuildChannels<'a> {
    guild_id: Id<GuildMarker>,
    http: &'a Client,
}

impl<'a> GetGuildChannels<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self { guild_id, http }
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
        Ok(Request::from_route(&Route::GetChannels {
            guild_id: self.guild_id.get(),
        }))
    }
}
