use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture},
};
use std::future::IntoFuture;
use twilight_model::{
    guild::Member,
    id::{
        Id,
        marker::{GuildMarker, UserMarker},
    },
};

struct GetMemberPath {
    guild_id: Id<GuildMarker>,
    user_id: Id<UserMarker>,
}

/// Get a member of a guild, by id.
#[must_use = "requests must be configured and executed"]
pub struct GetMember<'a> {
    http: &'a Client,
    path: GetMemberPath,
}

impl<'a> GetMember<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
    ) -> Self {
        Self {
            http,
            path: GetMemberPath { guild_id, user_id },
        }
    }
}

impl IntoFuture for GetMember<'_> {
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

impl TryIntoRequest for GetMember<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let GetMemberPath { guild_id, user_id } = self.path;

        let path = Path::builder()
            .resource("guilds")
            .id(guild_id)
            .resource("members")
            .id(user_id)
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Get, path))
    }
}
