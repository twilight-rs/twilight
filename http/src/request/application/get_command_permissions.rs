use crate::request::prelude::*;
use twilight_model::{
    application::command::permissions::GuildCommandPermissions,
    id::{ApplicationId, CommandId, GuildId},
};

/// Fetch all commands for a guild, by ID.
pub struct GetCommandPermissions<'a> {
    application_id: ApplicationId,
    command_id: CommandId,
    guild_id: GuildId,
    fut: Option<Pending<'a, GuildCommandPermissions>>,
    http: &'a Client,
}

impl<'a> GetCommandPermissions<'a> {
    pub(crate) fn new(
        http: &'a Client,
        application_id: ApplicationId,
        guild_id: GuildId,
        command_id: CommandId,
    ) -> Self {
        Self {
            application_id,
            command_id,
            guild_id,
            fut: None,
            http,
        }
    }

    fn start(&mut self) -> Result<()> {
        let request = Request::from_route(Route::GetCommandPermissions {
            application_id: self.application_id.0,
            command_id: self.command_id.0,
            guild_id: self.guild_id.0,
        });

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(GetCommandPermissions<'_>, GuildCommandPermissions);
