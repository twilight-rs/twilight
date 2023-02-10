use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    guild::GuildPreview,
    id::{marker::GuildMarker, Id},
};

/// For public guilds, get the guild preview.
///
/// This works even if the user is not in the guild.
#[must_use = "requests must be configured and executed"]
pub struct GetGuildPreview<'a> {
    guild_id: Id<GuildMarker>,
    http: &'a Client,
}

impl<'a> GetGuildPreview<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self { guild_id, http }
    }
}

impl IntoFuture for GetGuildPreview<'_> {
    type Output = Result<Response<GuildPreview>, Error>;

    type IntoFuture = ResponseFuture<GuildPreview>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetGuildPreview<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetGuildPreview {
            guild_id: self.guild_id.get(),
        }))
    }
}
