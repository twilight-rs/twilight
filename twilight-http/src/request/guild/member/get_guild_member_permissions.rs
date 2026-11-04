//! Request builder and response body for getting a guild member's permissions.

use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use serde::Deserialize;
use std::future::IntoFuture;
use twilight_model::{
    guild::Permissions,
    id::{
        Id,
        marker::{GuildMarker, UserMarker},
    },
};

/// Response body for the [`GetGuildMemberPermissions`] request.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct GetGuildMemberPermissionsResponseBody {
    /// Computed permissions.
    pub permissions: Permissions,
}

struct GetGuildMemberPermissionsPath {
    guild_id: Id<GuildMarker>,
    user_id: Id<UserMarker>,
}

/// Get a member's computed guild permissions, combining their roles with
/// `@everyone`.
#[must_use = "requests must be configured and executed"]
pub struct GetGuildMemberPermissions<'a> {
    http: &'a Client,
    path: GetGuildMemberPermissionsPath,
}

impl<'a> GetGuildMemberPermissions<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
    ) -> Self {
        Self {
            http,
            path: GetGuildMemberPermissionsPath { guild_id, user_id },
        }
    }
}

impl IntoFuture for GetGuildMemberPermissions<'_> {
    type Output = Result<Response<GetGuildMemberPermissionsResponseBody>, Error>;

    type IntoFuture = ResponseFuture<GetGuildMemberPermissionsResponseBody>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetGuildMemberPermissions<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let GetGuildMemberPermissionsPath { guild_id, user_id } = self.path;

        Ok(Request::from_route(&Route::GetGuildMemberPermissions {
            guild_id: guild_id.get(),
            user_id: user_id.get(),
        }))
    }
}
