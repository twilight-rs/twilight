use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{marker::ListBody, Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    channel::Channel,
    id::{marker::GuildMarker, Id},
};

/// Get the channels in a guild.
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
