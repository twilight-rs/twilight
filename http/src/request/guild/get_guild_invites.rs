use crate::request::prelude::*;
use twilight_model::{id::GuildId, invite::Invite};

/// Get information about the invites of a guild.
pub struct GetGuildInvites<'a> {
    fut: Option<Pending<'a, Vec<Invite>>>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetGuildInvites<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self {
            fut: None,
            guild_id,
            http,
        }
    }

    fn start(&mut self) -> Result<()> {
        let request = Request::from_route(Route::GetGuildInvites {
            guild_id: self.guild_id.0,
        });

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(GetGuildInvites<'_>, Vec<Invite>);
