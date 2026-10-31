use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture},
};
use std::{collections::HashMap, future::IntoFuture};
use twilight_model::id::{
    Id,
    marker::{GuildMarker, RoleMarker},
};

struct GetGuildRoleMemberCountsPath {
    guild_id: Id<GuildMarker>,
}

#[must_use = "requests must be configured and executed"]
pub struct GetGuildRoleMemberCounts<'a> {
    http: &'a Client,
    path: GetGuildRoleMemberCountsPath,
}

impl<'a> GetGuildRoleMemberCounts<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self {
            http,
            path: GetGuildRoleMemberCountsPath { guild_id },
        }
    }
}

impl IntoFuture for GetGuildRoleMemberCounts<'_> {
    type Output = Result<Response<HashMap<Id<RoleMarker>, u64>>, Error>;

    type IntoFuture = ResponseFuture<HashMap<Id<RoleMarker>, u64>>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetGuildRoleMemberCounts<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let GetGuildRoleMemberCountsPath { guild_id } = self.path;

        let path = Path::builder()
            .resource("guilds")
            .id(guild_id)
            .resource("roles")
            .subresource("member-counts")
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Get, path))
    }
}
