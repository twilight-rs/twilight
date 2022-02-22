mod chat_input;
mod message;
mod user;

pub use self::{
    chat_input::CreateGlobalChatInputCommand, message::CreateGlobalMessageCommand,
    user::CreateGlobalUserCommand,
};

use crate::{
    client::Client,
    error::Error as HttpError,
    request::{Payload, Request, RequestBuilder},
    routing::Route,
};
use serde::Serialize;
use twilight_model::id::{marker::ApplicationMarker, Id};
use twilight_validate::command::CommandValidationError;

/// Create a new global command.
///
/// You may either use the provided request builders for each command type:
/// [`chat_input`], [`message`], or [`user`], or you may use the [`payload`]
/// method via the [`Payload`] trait to provide your own Command.
///
/// [`chat_input`]: Self::chat_input
/// [`message`]: Self::message
/// [`payload`]: Self::payload
/// [`user`]: Self::user
#[must_use = "the command must have a type"]
pub struct CreateGlobalCommand<'a> {
    application_id: Id<ApplicationMarker>,
    http: &'a Client,
}

impl<'a> CreateGlobalCommand<'a> {
    pub(crate) const fn new(http: &'a Client, application_id: Id<ApplicationMarker>) -> Self {
        Self {
            application_id,
            http,
        }
    }

    /// Create a new chat input global command.
    ///
    /// The command name must only contain alphanumeric characters and lowercase
    /// variants must be used where possible. Special characters `-` and `_` are
    /// allowed. The description must be between 1 and 100 characters in length.
    ///
    /// Creating a command with the same name as an already-existing global
    /// command will overwrite the old command. See
    /// [Discord Docs/Create Global Application Command].
    ///
    /// # Errors
    ///
    /// Returns an error of type [`NameLengthInvalid`] or
    /// [`NameCharacterInvalid`] if the command name is invalid.
    ///
    /// Returns an error of type [`DescriptionInvalid`] if the
    /// command description is not between 1 and 100 characters.
    ///
    /// [`DescriptionInvalid`]: twilight_validate::command::CommandValidationErrorType::DescriptionInvalid
    /// [`NameCharacterInvalid`]: twilight_validate::command::CommandValidationErrorType::NameCharacterInvalid
    /// [`NameLengthInvalid`]: twilight_validate::command::CommandValidationErrorType::NameLengthInvalid
    /// [Discord Docs/Create Global Application Command]: https://discord.com/developers/docs/interactions/application-commands#create-global-application-command
    pub fn chat_input(
        self,
        name: &'a str,
        description: &'a str,
    ) -> Result<CreateGlobalChatInputCommand<'a>, CommandValidationError> {
        CreateGlobalChatInputCommand::new(self.http, self.application_id, name, description)
    }

    /// Create a new message global command.
    ///
    /// Creating a command with the same name as an already-existing global
    /// command will overwrite the old command. See
    /// [Discord Docs/Create Global Application Command].
    ///
    /// # Errors
    ///
    /// Returns an error of type [`NameLengthInvalid`] if the command name is
    /// not between 1 and 32 characters.
    ///
    /// [`NameLengthInvalid`]: twilight_validate::command::CommandValidationErrorType::NameLengthInvalid
    /// [Discord Docs/Create Global Application Command]: https://discord.com/developers/docs/interactions/application-commands#create-global-application-command
    pub fn message(
        self,
        name: &'a str,
    ) -> Result<CreateGlobalMessageCommand<'a>, CommandValidationError> {
        CreateGlobalMessageCommand::new(self.http, self.application_id, name)
    }

    /// Create a new user global command.
    ///
    /// Creating a command with the same name as an already-existing global
    /// command will overwrite the old command. See
    /// [Discord Docs/Create Global Application Command].
    ///
    /// # Errors
    ///
    /// Returns an error of type [`NameLengthInvalid`] if the command name is
    /// not between 1 and 32 characters.
    ///
    /// [`NameLengthInvalid`]: twilight_validate::command::CommandValidationErrorType::NameLengthInvalid
    /// [Discord Docs/Create Global Application Command]: https://discord.com/developers/docs/interactions/application-commands#create-global-application-command
    pub fn user(
        self,
        name: &'a str,
    ) -> Result<CreateGlobalUserCommand<'a>, CommandValidationError> {
        CreateGlobalUserCommand::new(self.http, self.application_id, name)
    }
}

impl<'a> Payload<'a> for CreateGlobalCommand<'a> {
    /// Supply a payload to a request builder, building the request.
    ///
    /// This could be a custom struct, or it could be a [`Command`] from the
    /// [Command Builder].
    ///
    /// [`Command`]: twilight_model::application::command::Command
    /// [Command Builder]: https://docs.rs/twilight-util/latest/twilight_util/builder/command/index.html
    fn payload(self, payload: &'a impl Serialize) -> Result<Request, HttpError> {
        Request::builder(&Route::CreateGlobalCommand {
            application_id: self.application_id.get(),
        })
        .json(payload)
        .map(RequestBuilder::build)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use twilight_http_ratelimiting::Method;
    use twilight_model::application::command::CommandType;

    const APPLICATION_ID: Id<ApplicationMarker> = Id::new(1);

    #[derive(Serialize)]
    struct CommandShim {
        description: String,
        name: String,
        #[serde(rename = "type")]
        kind: CommandType,
    }

    #[test]
    fn test_payload() -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new("token".into());

        let command = CommandShim {
            description: String::new(),
            name: String::from("user command"),
            kind: CommandType::User,
        };

        let request = client
            .interaction(APPLICATION_ID)
            .create_global_command()
            .payload(&command)?;

        assert_eq!(
            request.body,
            Some(br#"{"description":"","name":"user command","type":2}"#.to_vec())
        );
        assert_eq!(request.method, Method::Post);
        assert_eq!(
            request.path,
            Route::CreateGlobalCommand {
                application_id: APPLICATION_ID.get()
            }
            .to_string()
        );

        Ok(())
    }
}
