use crate::{
    Error,
    client::Client,
    request::{Method, Path, Request, Route, TryIntoRequest},
    response::{Response, ResponseFuture},
};
use std::future::IntoFuture;
use twilight_model::{
    guild::Member,
    id::{Id, marker::GuildMarker},
};

pub(in crate::request) struct GetCurrentUserGuildMemberPath {
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

impl Route for GetCurrentUserGuildMember<'_> {
    type Fields = GetCurrentUserGuildMemberPath;

    const METHOD: Method = Method::Get;

    fn path(fields: Self::Fields) -> Path {
        Path::builder()
            .resource("users")
            .me()
            .resource("guilds")
            .id(fields.guild_id)
            .build()
    }
}

impl TryIntoRequest for GetCurrentUserGuildMember<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route_new::<Self>(self.path))
    }
}
