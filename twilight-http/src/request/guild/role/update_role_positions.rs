use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{marker::ListBody, Response, ResponseFuture},
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

/// Modify the position of the roles.
///
/// The minimum amount of roles to modify, is a swap between two roles.
#[must_use = "requests must be configured and executed"]
pub struct UpdateRolePositions<'a> {
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    roles: &'a [(Id<RoleMarker>, u64)],
}

impl<'a> UpdateRolePositions<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        roles: &'a [(Id<RoleMarker>, u64)],
    ) -> Self {
        Self {
            guild_id,
            http,
            roles,
        }
    }
}

impl IntoFuture for UpdateRolePositions<'_> {
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

impl TryIntoRequest for UpdateRolePositions<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::UpdateRolePositions {
            guild_id: self.guild_id.get(),
        });

        request = request.json(&self.roles)?;

        Ok(request.build())
    }
}
