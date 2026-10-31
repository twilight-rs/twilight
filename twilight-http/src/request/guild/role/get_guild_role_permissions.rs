//! Request builder and response body for getting a guild role's permissions.

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
        marker::{GuildMarker, RoleMarker},
    },
};

/// Response body for the [`GetGuildRolePermissions`] request.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct GetGuildRolePermissionsResponseBody {
    /// Computed permissions.
    pub permissions: Permissions,
}

struct GetGuildRolePermissionsPath {
    guild_id: Id<GuildMarker>,
    role_id: Id<RoleMarker>,
}

/// Get the combined guild permissions of the specified role and `@everyone`.
///
/// This calculation does not include other roles or member-specific
/// restrictions.
#[must_use = "requests must be configured and executed"]
pub struct GetGuildRolePermissions<'a> {
    http: &'a Client,
    path: GetGuildRolePermissionsPath,
}

impl<'a> GetGuildRolePermissions<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        role_id: Id<RoleMarker>,
    ) -> Self {
        Self {
            http,
            path: GetGuildRolePermissionsPath { guild_id, role_id },
        }
    }
}

impl IntoFuture for GetGuildRolePermissions<'_> {
    type Output = Result<Response<GetGuildRolePermissionsResponseBody>, Error>;

    type IntoFuture = ResponseFuture<GetGuildRolePermissionsResponseBody>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetGuildRolePermissions<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let GetGuildRolePermissionsPath { guild_id, role_id } = self.path;

        Ok(Request::from_route(&Route::GetGuildRolePermissions {
            guild_id: guild_id.get(),
            role_id: role_id.get(),
        }))
    }
}
