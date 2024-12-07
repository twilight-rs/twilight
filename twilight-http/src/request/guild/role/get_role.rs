use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    guild::Role,
    id::{
        marker::{GuildMarker, RoleMarker},
        Id,
    },
};

/// Get a role of a guild.
#[must_use = "requests must be configured and executed"]
pub struct GetRole<'a> {
    guild_id: Id<GuildMarker>,
    role_id: Id<RoleMarker>,
    http: &'a Client,
}

impl<'a> GetRole<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        role_id: Id<RoleMarker>,
    ) -> Self {
        Self {
            guild_id,
            role_id,
            http,
        }
    }
}

impl IntoFuture for GetRole<'_> {
    type Output = Result<Response<Role>, Error>;

    type IntoFuture = ResponseFuture<Role>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetRole<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetRole {
            guild_id: self.guild_id.get(),
            role_id: self.role_id.get(),
        }))
    }
}
