use crate::request::prelude::*;
use twilight_model::applications::Command;
use twilight_model::applications::CommandOption;
use twilight_model::id::*;

/// Create a new global command.
///
/// The name must be between 3 and 32 characters in length, and the description
/// must be between 1 and 100 characters in length. Creating a command with the
/// same name as an already-existing global command will overwwrite the old
/// command. See [the discord docs] for more information.
///
/// [the discord docs]: https://discord.com/developers/docs/interactions/slash-commands#create-global-application-command
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
                id: None,
                application_id,
                name,
                description,
                options: vec![],
            },
            application_id,
            fut: None,
            http,
        }
    }

    /// Add a command option.
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
