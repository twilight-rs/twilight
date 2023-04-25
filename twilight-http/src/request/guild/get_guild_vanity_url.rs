use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    guild::VanityUrl,
    id::{marker::GuildMarker, Id},
};

/// Get a guild's vanity url, if there is one.
#[must_use = "requests must be configured and executed"]
pub struct GetGuildVanityUrl<'a> {
    guild_id: Id<GuildMarker>,
    http: &'a Client,
}

impl<'a> GetGuildVanityUrl<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self { guild_id, http }
    }
}

impl IntoFuture for GetGuildVanityUrl<'_> {
    type Output = Result<Response<VanityUrl>, Error>;

    type IntoFuture = ResponseFuture<VanityUrl>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetGuildVanityUrl<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetGuildVanityUrl {
            guild_id: self.guild_id.get(),
        }))
    }
}
