use crate::request::prelude::*;
use dawn_model::id::{GuildId, RoleId};

pub struct DeleteRole<'a> {
    fut: Option<Pending<'a, ()>>,
    guild_id: GuildId,
    http: &'a Client,
    role_id: RoleId,
}

impl<'a> DeleteRole<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId, role_id: RoleId) -> Self {
        Self {
            fut: None,
            guild_id,
            http,
            role_id,
        }
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.verify(Request::from(
            Route::DeleteRole {
                guild_id: self.guild_id.0,
                role_id: self.role_id.0,
            },
        ))));

        Ok(())
    }
}

poll_req!(DeleteRole<'_>, ());
