use crate::request::prelude::*;
use dawn_model::{
    guild::Role,
    id::{GuildId, RoleId},
};

pub struct UpdateRolePositions<'a> {
    fut: Option<Pending<'a, Vec<Role>>>,
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

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from((
            serde_json::to_vec(&self.roles)?,
            Route::UpdateRolePositions {
                guild_id: self.guild_id.0,
            },
        )))));

        Ok(())
    }
}

poll_req!(UpdateRolePositions<'_>, Vec<Role>);
