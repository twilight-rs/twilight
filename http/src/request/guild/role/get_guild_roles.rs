use crate::request::prelude::*;
use dawn_model::{guild::Role, id::GuildId};

pub struct GetGuildRoles<'a> {
    fut: Option<Pending<'a, Vec<Role>>>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetGuildRoles<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self {
            fut: None,
            guild_id,
            http,
        }
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from(
            Route::GetGuildRoles {
                guild_id: self.guild_id.0,
            },
        ))));

        Ok(())
    }
}

poll_req!(GetGuildRoles<'_>, Vec<Role>);
