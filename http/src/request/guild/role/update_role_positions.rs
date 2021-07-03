use crate::{
    client::Client,
    error::Error,
    request::{PendingResponse, Request},
    response::marker::ListBody,
    routing::Route,
};
use twilight_model::{
    guild::Role,
    id::{GuildId, RoleId},
};

/// Modify the position of the roles.
///
/// The minimum amount of roles to modify, is a swap between two roles.
pub struct UpdateRolePositions<'a> {
    fut: Option<PendingResponse<'a, ListBody<Role>>>,
    guild_id: GuildId,
    http: &'a Client,
    roles: Vec<(RoleId, u64)>,
}

impl<'a> UpdateRolePositions<'a> {
    pub(crate) fn new(
        http: &'a Client,
        guild_id: GuildId,
        roles: impl Iterator<Item = (RoleId, u64)>,
    ) -> Self {
        Self {
            fut: None,
            guild_id,
            http,
            roles: roles.collect(),
        }
    }

    fn start(&mut self) -> Result<(), Error> {
        let request = Request::builder(Route::UpdateRolePositions {
            guild_id: self.guild_id.0,
        })
        .json(&self.roles)?
        .build();

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(UpdateRolePositions<'_>, ListBody<Role>);
