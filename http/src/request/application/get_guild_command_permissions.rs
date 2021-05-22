use crate::request::prelude::*;
use twilight_model::{
    application::command::permissions::GuildCommandPermissions,
    id::{ApplicationId, GuildId},
};

/// Fetch command permissions for all commands from the current application in a guild.
pub struct GetGuildCommandPermissions<'a> {
    application_id: ApplicationId,
    guild_id: GuildId,
    fut: Option<Pending<'a, Vec<GuildCommandPermissions>>>,
    http: &'a Client,
}

impl<'a> GetGuildCommandPermissions<'a> {
    pub(crate) fn new(http: &'a Client, application_id: ApplicationId, guild_id: GuildId) -> Self {
        Self {
            application_id,
            guild_id,
            fut: None,
            http,
        }
    }

    fn start(&mut self) -> Result<()> {
        let request = Request::from_route(Route::GetGuildCommandPermissions {
            application_id: self.application_id.0,
            guild_id: self.guild_id.0,
        });

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(GetGuildCommandPermissions<'_>, Vec<GuildCommandPermissions>);
