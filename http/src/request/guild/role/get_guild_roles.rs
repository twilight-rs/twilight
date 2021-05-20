use crate::request::prelude::*;
use twilight_model::{guild::Role, id::GuildId};

/// Get the roles of a guild.
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
        let request = Request::from_route(Route::GetGuildRoles {
            guild_id: self.guild_id.0,
        });

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(GetGuildRoles<'_>, Vec<Role>);
