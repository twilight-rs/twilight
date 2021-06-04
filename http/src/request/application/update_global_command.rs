use crate::request::prelude::*;
use twilight_model::{
    application::command::CommandOption,
    id::{ApplicationId, CommandId},
};

#[derive(Debug, Default, serde::Serialize)]
struct UpdateGlobalCommandFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<Vec<CommandOption>>,
}

/// Edit a global command, by ID.
///
/// You must specify a name and description. See [the discord docs] for more
/// information.
///
/// [the discord docs]: https://discord.com/developers/docs/interactions/slash-commands#edit-global-application-command
pub struct UpdateGlobalCommand<'a> {
    fields: UpdateGlobalCommandFields,
    command_id: CommandId,
    application_id: ApplicationId,
    fut: Option<Pending<'a, ()>>,
    http: &'a Client,
}

impl<'a> UpdateGlobalCommand<'a> {
    pub(crate) fn new(
        http: &'a Client,
        application_id: ApplicationId,
        command_id: CommandId,
    ) -> Self {
        Self {
            application_id,
            command_id,
            fields: UpdateGlobalCommandFields::default(),
            fut: None,
            http,
        }
    }

    /// Edit the name of the command.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.fields.name = Some(name.into());

        self
    }

    /// Edit the description of the command.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.fields.description = Some(description.into());

        self
    }

    /// Edit the command options of the command.
    pub fn push_command_option(mut self, option: CommandOption) -> Self {
        if let Some(ref mut arr) = self.fields.options {
            arr.push(option);
        } else {
            self.fields.options = Some(vec![option]);
        }

        self
    }

    fn start(&mut self) -> Result<()> {
        let request = Request::builder(Route::UpdateGlobalCommand {
            application_id: self.application_id.0,
            command_id: self.command_id.0,
        })
        .json(&self.fields)?;

        self.fut
            .replace(Box::pin(self.http.verify(request.build())));

        Ok(())
    }
}

poll_req!(UpdateGlobalCommand<'_>, ());
