mod chat_input;
mod message;
mod user;

pub use self::{
    chat_input::CreateGuildChatInputCommand, message::CreateGuildMessageCommand,
    user::CreateGuildUserCommand,
};

use crate::Client;
use twilight_model::id::{
    marker::{ApplicationMarker, GuildMarker},
    Id,
};
use twilight_validate::command::{name as validate_name, CommandValidationError};

/// Create a new command in a guild.
///
/// The name must be between 1 and 32 characters in length. Creating a guild
/// command with the same name as an already-existing guild command in the same
/// guild will overwrite the old command. See [the discord docs] for more
/// information.
///
/// [the discord docs]: https://discord.com/developers/docs/interactions/application-commands#create-guild-application-command
#[must_use = "the command must have a type"]
pub struct CreateGuildCommand<'a> {
    application_id: Id<ApplicationMarker>,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    name: &'a str,
}

impl<'a> CreateGuildCommand<'a> {
    pub(crate) fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        guild_id: Id<GuildMarker>,
        name: &'a str,
    ) -> Result<Self, CommandValidationError> {
        validate_name(name)?;

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
    /// in the same guild will overwrite the old command. See [the discord docs]
    /// for more information.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`DescriptionInvalid`] error type if the
    /// command description is not between 1 and 100 characters.
    ///
    /// [`DescriptionInvalid`]: twilight_validate::command::CommandValidationErrorType::DescriptionInvalid
    /// [the discord docs]: https://discord.com/developers/docs/interactions/application-commands#create-guild-application-command
    pub fn chat_input(
        self,
        description: &'a str,
    ) -> Result<CreateGuildChatInputCommand<'a>, CommandValidationError> {
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
    /// discord docs] for more information.
    ///
    /// [the discord docs]: https://discord.com/developers/docs/interactions/application-commands#create-guild-application-command
    pub const fn message(self) -> CreateGuildMessageCommand<'a> {
        CreateGuildMessageCommand::new(self.http, self.application_id, self.guild_id, self.name)
    }

    /// Create a user command in a guild.
    ///
    /// Creating a guild command with the same name as an already-existing guild
    /// command in the same guild will overwrite the old command. See [the
    /// discord docs] for more information.
    ///
    /// [the discord docs]: https://discord.com/developers/docs/interactions/application-commands#create-guild-application-command
    pub const fn user(self) -> CreateGuildUserCommand<'a> {
        CreateGuildUserCommand::new(self.http, self.application_id, self.guild_id, self.name)
    }
}
