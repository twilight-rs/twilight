use crate::{
    client::Client,
    error::Error,
    request::{IntoRequest, Request},
    response::{marker::ListBody, ResponseFuture},
    routing::Route,
};
use twilight_model::{
    guild::Role,
    id::{GuildId, RoleId},
};

/// Modify the position of the roles.
///
/// The minimum amount of roles to modify, is a swap between two roles.
#[must_use = "requests must be configured and executed"]
pub struct UpdateRolePositions<'a> {
    guild_id: GuildId,
    http: &'a Client,
    roles: &'a [(RoleId, u64)],
}

impl<'a> UpdateRolePositions<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: GuildId,
        roles: &'a [(RoleId, u64)],
    ) -> Self {
        Self {
            guild_id,
            http,
            roles,
        }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<ListBody<Role>> {
        let http = self.http;

        match self.into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl IntoRequest for UpdateRolePositions<'_> {
    fn into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::UpdateRolePositions {
            guild_id: self.guild_id.get(),
        });

        request = request.json(&self.roles)?;

        Ok(request.build())
    }
}
