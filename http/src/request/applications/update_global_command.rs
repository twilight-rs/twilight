use super::InteractionError;
use crate::request::prelude::*;
use twilight_model::{
    applications::command::{Command, CommandOption},
    id::{ApplicationId, CommandId},
};

/// Edit a global command, by ID.
///
/// You must specify a name and description. See [the discord docs] for more
/// information.
///
/// [the discord docs]: https://discord.com/developers/docs/interactions/slash-commands#edit-global-application-command
pub struct UpdateGlobalCommand<'a> {
    command: Command,
    application_id: ApplicationId,
    fut: Option<Pending<'a, ()>>,
    http: &'a Client,
}

impl<'a> UpdateGlobalCommand<'a> {
    pub(crate) fn new(
        http: &'a Client,
        application_id: Option<ApplicationId>,
        command_id: CommandId,
        name: String,
        description: String,
    ) -> Result<Self, InteractionError> {
        let application_id = application_id.ok_or(InteractionError::ApplicationIdNotPresent)?;

        Ok(Self {
            command: Command {
                id: Some(command_id),
                application_id: Some(application_id),
                name,
                description,
                options: vec![],
            },
            application_id,
            fut: None,
            http,
        })
    }

    /// Add a command option.
    pub fn push_command_option(mut self, option: CommandOption) -> Self {
        self.command.options.push(option);

        self
    }

    fn start(&mut self) -> Result<()> {
        let req = Request::from((
            crate::json_to_vec(&self.command)?,
            Route::UpdateGlobalCommand {
                application_id: self.application_id.0,
                // TODO: Figure out if this is how we want to do it,
                // similar to the same question in the update guild
                // command file.
                command_id: self.command.id.unwrap().0,
            },
        ));
        self.fut.replace(Box::pin(self.http.verify(req)));

        Ok(())
    }
}

poll_req!(UpdateGlobalCommand<'_>, ());
