use crate::{
    Error,
    client::Client,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture},
};
use std::future::IntoFuture;
use twilight_model::{
    guild::Member,
    id::{Id, marker::GuildMarker},
};

pub struct GetCurrentUserGuildMemberPath {
    guild_id: Id<GuildMarker>,
}

/// Get information about the current user in a guild.
#[must_use = "requests must be configured and executed"]
pub struct GetCurrentUserGuildMember<'a> {
    http: &'a Client,
    path: GetCurrentUserGuildMemberPath,
}

impl<'a> GetCurrentUserGuildMember<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self {
            http,
            path: GetCurrentUserGuildMemberPath { guild_id },
        }
    }
}

impl IntoFuture for GetCurrentUserGuildMember<'_> {
    type Output = Result<Response<Member>, Error>;

    type IntoFuture = ResponseFuture<Member>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetCurrentUserGuildMember<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let path = Path::builder()
            .resource("users")
            .me()
            .resource("guilds")
            .id(self.path.guild_id)
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Get, path))
    }
}
