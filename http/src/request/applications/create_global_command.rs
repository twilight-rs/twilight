use crate::request::prelude::*;
use twilight_model::applications::Command;
use twilight_model::applications::CommandOption;
use twilight_model::id::*;

pub struct CreateGlobalCommand<'a> {
    command: Command,
    application_id: ApplicationId,
    fut: Option<Pending<'a, ()>>,
    http: &'a Client,
}

impl<'a> CreateGlobalCommand<'a> {
    pub(crate) fn new(
        http: &'a Client,
        application_id: ApplicationId,
        name: String,
        description: String,
    ) -> Self {
        Self {
            command: Command {
                application_id,
                command_id: None,
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
            Route::CreateGlobalCommand {
                application_id: self.application_id.0,
            },
        ));
        self.fut.replace(Box::pin(self.http.verify(req)));

        Ok(())
    }
}

poll_req!(CreateGlobalCommand<'_>, ());
