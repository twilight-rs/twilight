mod chat_input;
mod message;
mod user;

pub use self::{
    chat_input::CreateGuildChatInputCommand, message::CreateGuildMessageCommand,
    user::CreateGuildUserCommand,
};

use super::super::{InteractionError, InteractionErrorType};
use crate::{request::validate_inner, Client};
use twilight_model::id::{ApplicationId, GuildId};

/// Create a new command in a guild.
///
/// The name must be between 1 and 32 characters in length. Creating a guild
/// command with the same name as an already-existing guild command in the same
/// guild will overwrite the old command. See [the Discord Docs/Create Guild Application Command] for more
/// information.
///
/// [the Discord Docs/Create Guild Application Command]: https://discord.com/developers/docs/interactions/application-commands#create-guild-application-command
#[must_use = "the command must have a type"]
pub struct CreateGuildCommand<'a> {
    application_id: ApplicationId,
    guild_id: GuildId,
    http: &'a Client,
    name: &'a str,
}

impl<'a> CreateGuildCommand<'a> {
    pub(crate) fn new(
        http: &'a Client,
        application_id: ApplicationId,
        guild_id: GuildId,
        name: &'a str,
    ) -> Result<Self, InteractionError> {
        if !validate_inner::command_name(name) {
            return Err(InteractionError {
                kind: InteractionErrorType::CommandNameValidationFailed,
            });
        }

        Ok(Self {
            application_id,
            guild_id,
            http,
            name,
        })
    }

    /// Create a chat input command in a guild.
    ///
    /// The description must be between 1 and 100 characters in length. Creating
    /// a guild command with the same name as an already-existing guild command
    /// in the same guild will overwrite the old command. See [the Discord Docs/Create Guild Application Command]
    ///
    /// # Errors
    ///
    /// Returns an [`InteractionErrorType::CommandDescriptionValidationFailed`]
    /// error type if the command description is not between 1 and
    /// 100 characters.
    ///
    /// [the Discord Docs/Create Guild Application Command]: https://discord.com/developers/docs/interactions/application-commands#create-guild-application-command
    pub fn chat_input(
        self,
        description: &'a str,
    ) -> Result<CreateGuildChatInputCommand<'a>, InteractionError> {
        CreateGuildChatInputCommand::new(
            self.http,
            self.application_id,
            self.guild_id,
            self.name,
            description,
        )
    }

    /// Create a message command in a guild.
    ///
    /// Creating a guild command with the same name as an already-existing guild
    /// command in the same guild will overwrite the old command. See [the
    /// Discord Docs/Create Guild Application Command].
    ///
    /// [the Discord Docs/Create Guild Application Command]: https://discord.com/developers/docs/interactions/application-commands#create-guild-application-command
    pub const fn message(self) -> CreateGuildMessageCommand<'a> {
        CreateGuildMessageCommand::new(self.http, self.application_id, self.guild_id, self.name)
    }

    /// Create a user command in a guild.
    ///
    /// Creating a guild command with the same name as an already-existing guild
    /// command in the same guild will overwrite the old command. See [the
    /// Discord Docs/Create Guild Application Command].
    ///
    /// [the Discord Docs/Create Guild Application Command]: https://discord.com/developers/docs/interactions/application-commands#create-guild-application-command
    pub const fn user(self) -> CreateGuildUserCommand<'a> {
        CreateGuildUserCommand::new(self.http, self.application_id, self.guild_id, self.name)
    }
}
