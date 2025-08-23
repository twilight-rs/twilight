use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    guild::Ban,
    id::{
        Id,
        marker::{GuildMarker, UserMarker},
    },
};

/// Get information about a ban of a guild.
///
/// Includes the user banned and the reason.
#[must_use = "requests must be configured and executed"]
pub struct GetBan<'a> {
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    user_id: Id<UserMarker>,
}

impl<'a> GetBan<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
    ) -> Self {
        Self {
            guild_id,
            http,
            user_id,
        }
    }
}

impl IntoFuture for GetBan<'_> {
    type Output = Result<Response<Ban>, Error>;

    type IntoFuture = ResponseFuture<Ban>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetBan<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetBan {
            guild_id: self.guild_id.get(),
            user_id: self.user_id.get(),
        }))
    }
}
