use super::InteractionError;
use crate::request::prelude::*;
use twilight_model::{
    applications::command::Command,
    id::{ApplicationId, GuildId},
};

/// Set commands in a guild
///
/// This will set the commands availible in a guild to a list of commands,
/// this method is idempotent which means it can be used every time the bot
/// starts without any issues with ratelimits if there are no changes.
pub struct SetGuildCommands<'a> {
    commands: Vec<Command>,
    application_id: ApplicationId,
    guild_id: GuildId,
    fut: Option<Pending<'a, ()>>,
    http: &'a Client,
}

impl<'a> SetGuildCommands<'a> {
    pub(crate) fn new(
        http: &'a Client,
        application_id: Option<ApplicationId>,
        guild_id: GuildId,
        commands: Vec<Command>,
    ) -> Result<Self, InteractionError> {
        let application_id = application_id.ok_or(InteractionError::ApplicationIdNotPresent)?;

        Ok(Self {
            commands,
            application_id,
            guild_id,
            fut: None,
            http,
        })
    }

    fn start(&mut self) -> Result<()> {
        let req = Request::from((
            crate::json_to_vec(&self.commands)?,
            Route::SetGuildCommands {
                application_id: self.application_id.0,
                guild_id: self.guild_id.0,
            },
        ));
        self.fut.replace(Box::pin(self.http.verify(req)));

        Ok(())
    }
}

poll_req!(SetGuildCommands<'_>, ());
