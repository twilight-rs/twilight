use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture},
};
use std::future::IntoFuture;
use twilight_model::{
    guild::Ban,
    id::{
        Id,
        marker::{GuildMarker, UserMarker},
    },
};

struct GetBanPath {
    guild_id: Id<GuildMarker>,
    user_id: Id<UserMarker>,
}

/// Get information about a ban of a guild.
///
/// Includes the user banned and the reason.
#[must_use = "requests must be configured and executed"]
pub struct GetBan<'a> {
    http: &'a Client,
    path: GetBanPath,
}

impl<'a> GetBan<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
    ) -> Self {
        Self {
            http,
            path: GetBanPath { guild_id, user_id },
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
        let GetBanPath { guild_id, user_id } = self.path;

        let path = Path::builder()
            .resource("guilds")
            .id(guild_id)
            .resource("bans")
            .id(user_id)
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Get, path))
    }
}
