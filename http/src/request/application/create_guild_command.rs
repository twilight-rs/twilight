use crate::request::prelude::*;
use twilight_model::{
    application::command::{Command, CommandOption},
    id::{ApplicationId, GuildId},
};

/// Create a new command in a guild.
///
/// The name must be between 3 and 32 characters in length, and the description
/// must be between 1 and 100 characters in length. Creating a guild command
/// with the same name as an already-existing guild command in the same guild
/// will overwrite the old command. See [the discord docs] for more information.
///
/// [the discord docs]: https://discord.com/developers/docs/interactions/slash-commands#create-guild-application-command
pub struct CreateGuildCommand<'a> {
    command: Command,
    application_id: ApplicationId,
    guild_id: GuildId,
    fut: Option<Pending<'a, ()>>,
    http: &'a Client,
}

impl<'a> CreateGuildCommand<'a> {
    pub(crate) fn new(
        http: &'a Client,
        application_id: ApplicationId,
        guild_id: GuildId,
        name: String,
        description: String,
    ) -> Self {
        Self {
            command: Command {
                application_id: Some(application_id),
                name,
                default_permission: None,
                description,
                id: None,
                options: vec![],
            },
            application_id,
            guild_id,
            fut: None,
            http,
        }
    }

    /// Add a command option.
    pub fn push_command_option(mut self, option: CommandOption) -> Self {
        self.command.options.push(option);

        self
    }

    /// Whether the command is enabled by default when the app is added to a guild
    pub fn default_permission(mut self, default: bool) -> Self {
        self.command.default_permission.replace(default);

        self
    }

    fn start(&mut self) -> Result<()> {
        let request = Request::builder(Route::CreateGuildCommand {
            application_id: self.application_id.0,
            guild_id: self.guild_id.0,
        })
        .json(&self.command)?;

        self.fut
            .replace(Box::pin(self.http.verify(request.build())));

        Ok(())
    }
}

poll_req!(CreateGuildCommand<'_>, ());
