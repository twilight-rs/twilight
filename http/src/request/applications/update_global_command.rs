use crate::request::prelude::*;
use twilight_model::applications::Command;
use twilight_model::applications::CommandOption;
use twilight_model::id::*;

pub struct UpdateGlobalCommand<'a> {
    command: Command,
    application_id: ApplicationId,
    fut: Option<Pending<'a, ()>>,
    http: &'a Client,
}

impl<'a> UpdateGlobalCommand<'a> {
    pub(crate) fn new(
        http: &'a Client,
        application_id: ApplicationId,
        command_id: CommandId,
        name: String,
        description: String,
    ) -> Self {
        Self {
            command: Command {
                application_id,
                command_id: Some(command_id),
                name,
                description,
                options: vec![],
            },
            application_id,
            fut: None,
            http,
        }
    }

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
                command_id: self.command.command_id.unwrap().0,
            },
        ));
        self.fut.replace(Box::pin(self.http.verify(req)));

        Ok(())
    }
}

poll_req!(UpdateGlobalCommand<'_>, ());
