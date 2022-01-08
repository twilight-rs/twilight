mod chat_input;
mod message;
mod user;

pub use self::{
    chat_input::CreateGlobalChatInputCommand, message::CreateGlobalMessageCommand,
    user::CreateGlobalUserCommand,
};

use super::super::{InteractionError, InteractionErrorType};
use crate::{request::validate_inner, Client};
use twilight_model::id::ApplicationId;

/// Create a new global command.
///
/// The name must be between 1 and 32 characters in length. Creating a command
/// with the same name as an already-existing global command will overwrite the
/// old command. See [the Discord docs] for more information.
///
/// [the Discord docs]: https://discord.com/developers/docs/interactions/application-commands#create-global-application-command
#[must_use = "the command must have a type"]
pub struct CreateGlobalCommand<'a> {
    application_id: ApplicationId,
    http: &'a Client,
    name: &'a str,
}

impl<'a> CreateGlobalCommand<'a> {
    pub(crate) fn new(
        http: &'a Client,
        application_id: ApplicationId,
        name: &'a str,
    ) -> Result<Self, InteractionError> {
        if !validate_inner::command_name(name) {
            return Err(InteractionError {
                kind: InteractionErrorType::CommandNameValidationFailed,
            });
        }

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
    /// overwrite the old command. See [the Discord docs] for more information.
    ///
    /// # Errors
    ///
    /// Returns an [`InteractionErrorType::CommandDescriptionValidationFailed`]
    /// error type if the command description is not between 1 and
    /// 100 characters.
    ///
    /// [the Discord docs]: https://discord.com/developers/docs/interactions/application-commands#create-global-application-command
    pub fn chat_input(
        self,
        description: &'a str,
    ) -> Result<CreateGlobalChatInputCommand<'a>, InteractionError> {
        CreateGlobalChatInputCommand::new(self.http, self.application_id, self.name, description)
    }

    /// Create a new message global command.
    ///
    /// Creating a command with the same name as an already-existing global
    /// command will overwrite the old command. See [the Discord docs] for more
    /// information.
    ///
    /// [the Discord docs]: https://discord.com/developers/docs/interactions/application-commands#create-global-application-command
    pub const fn message(self) -> CreateGlobalMessageCommand<'a> {
        CreateGlobalMessageCommand::new(self.http, self.application_id, self.name)
    }

    /// Create a new user global command.
    ///
    /// Creating a command with the same name as an already-existing global
    /// command will overwrite the old command. See [the Discord docs] for more
    /// information.
    ///
    /// [the Discord docs]: https://discord.com/developers/docs/interactions/application-commands#create-global-application-command
    pub const fn user(self) -> CreateGlobalUserCommand<'a> {
        CreateGlobalUserCommand::new(self.http, self.application_id, self.name)
    }
}
