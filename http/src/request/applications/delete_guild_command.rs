use super::InteractionError;
use crate::request::prelude::*;
use twilight_model::id::{ApplicationId, CommandId, GuildId};

pub struct DeleteGuildCommand<'a> {
    application_id: ApplicationId,
    command_id: CommandId,
    fut: Option<Pending<'a, ()>>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> DeleteGuildCommand<'a> {
    pub(crate) fn new(
        http: &'a Client,
        application_id: Option<ApplicationId>,
        guild_id: GuildId,
        command_id: CommandId,
    ) -> Result<Self, InteractionError> {
        let application_id = application_id.ok_or(InteractionError::ApplicationIdNotPresent)?;

        Ok(Self {
            application_id,
            command_id,
            fut: None,
            guild_id,
            http,
        })
    }

    fn start(&mut self) -> Result<()> {
        let req = Request::from(Route::DeleteGuildCommand {
            application_id: self.application_id.0,
            command_id: self.command_id.0,
            guild_id: self.guild_id.0,
        });
        self.fut.replace(Box::pin(self.http.verify(req)));

        Ok(())
    }
}

poll_req!(DeleteGuildCommand<'_>, ());
