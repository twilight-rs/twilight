use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture, marker::ListBody},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    guild::Role,
    id::{Id, marker::GuildMarker},
};

struct GetGuildRolesPath {
    guild_id: Id<GuildMarker>,
}

/// Get the roles of a guild.
#[must_use = "requests must be configured and executed"]
pub struct GetGuildRoles<'a> {
    http: &'a Client,
    path: GetGuildRolesPath,
}

impl<'a> GetGuildRoles<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self {
            http,
            path: GetGuildRolesPath { guild_id },
        }
    }
}

impl IntoFuture for GetGuildRoles<'_> {
    type Output = Result<Response<ListBody<Role>>, Error>;

    type IntoFuture = ResponseFuture<ListBody<Role>>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetGuildRoles<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetGuildRoles {
            guild_id: self.path.guild_id.get(),
        }))
    }
}
