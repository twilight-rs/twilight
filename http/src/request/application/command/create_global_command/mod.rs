mod chat_input;
mod message;
mod user;

pub use self::{
    chat_input::CreateGlobalChatInputCommand, message::CreateGlobalMessageCommand,
    user::CreateGlobalUserCommand,
};

use crate::Client;
use twilight_model::id::{marker::ApplicationMarker, Id};
use twilight_validate::command::{name as validate_name, CommandValidationError};

/// Create a new global command.
///
/// The name must be between 1 and 32 characters in length. Creating a command
/// with the same name as an already-existing global command will overwrite the
/// old command. See [the discord docs] for more information.
///
/// [the discord docs]: https://discord.com/developers/docs/interactions/application-commands#create-global-application-command
#[must_use = "the command must have a type"]
pub struct CreateGlobalCommand<'a> {
    application_id: Id<ApplicationMarker>,
    http: &'a Client,
    name: &'a str,
}

impl<'a> CreateGlobalCommand<'a> {
    pub(crate) fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        name: &'a str,
    ) -> Result<Self, CommandValidationError> {
        validate_name(name)?;

        Ok(Self {
            application_id,
            http,
            name,
        })
    }

    /// Create a new chat input global command.
    ///
    /// The description must be between 1 and 100 characters in length. Creating
    /// a command with the same name as an already-existing global command will
    /// overwrite the old command. See [the discord docs] for more information.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`DescriptionInvalid`] error type if the
    /// command description is not between 1 and 100 characters.
    ///
    /// [`DescriptionInvalid`]: twilight_validate::command::CommandValidationErrorType::DescriptionInvalid
    /// [the discord docs]: https://discord.com/developers/docs/interactions/application-commands#create-global-application-command
    pub fn chat_input(
        self,
        description: &'a str,
    ) -> Result<CreateGlobalChatInputCommand<'a>, CommandValidationError> {
        CreateGlobalChatInputCommand::new(self.http, self.application_id, self.name, description)
    }

    /// Create a new message global command.
    ///
    /// Creating a command with the same name as an already-existing global
    /// command will overwrite the old command. See [the discord docs] for more
    /// information.
    ///
    /// [the discord docs]: https://discord.com/developers/docs/interactions/application-commands#create-global-application-command
    pub const fn message(self) -> CreateGlobalMessageCommand<'a> {
        CreateGlobalMessageCommand::new(self.http, self.application_id, self.name)
    }

    /// Create a new user global command.
    ///
    /// Creating a command with the same name as an already-existing global
    /// command will overwrite the old command. See [the discord docs] for more
    /// information.
    ///
    /// [the discord docs]: https://discord.com/developers/docs/interactions/application-commands#create-global-application-command
    pub const fn user(self) -> CreateGlobalUserCommand<'a> {
        CreateGlobalUserCommand::new(self.http, self.application_id, self.name)
    }
}
