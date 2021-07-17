mod create_followup_message;
mod create_global_command;
mod create_guild_command;
mod delete_followup_message;
mod delete_global_command;
mod delete_guild_command;
mod delete_original_response;
mod get_command_permissions;
mod get_global_commands;
mod get_guild_command_permissions;
mod get_guild_commands;
mod get_original_response;
mod interaction_callback;
mod set_command_permissions;
mod set_global_commands;
mod set_guild_commands;
mod update_command_permissions;
mod update_followup_message;
mod update_global_command;
mod update_guild_command;
mod update_original_response;

pub use self::{
    create_followup_message::CreateFollowupMessage,
    create_global_command::CreateGlobalCommand,
    create_guild_command::CreateGuildCommand,
    delete_followup_message::DeleteFollowupMessage,
    delete_global_command::DeleteGlobalCommand,
    delete_guild_command::DeleteGuildCommand,
    delete_original_response::DeleteOriginalResponse,
    get_command_permissions::GetCommandPermissions,
    get_global_commands::GetGlobalCommands,
    get_guild_command_permissions::GetGuildCommandPermissions,
    get_guild_commands::GetGuildCommands,
    get_original_response::GetOriginalResponse,
    interaction_callback::InteractionCallback,
    set_command_permissions::SetCommandPermissions,
    set_global_commands::SetGlobalCommands,
    set_guild_commands::SetGuildCommands,
    update_command_permissions::UpdateCommandPermissions,
    update_followup_message::{
        UpdateFollowupMessage, UpdateFollowupMessageError, UpdateFollowupMessageErrorType,
    },
    update_global_command::UpdateGlobalCommand,
    update_guild_command::UpdateGuildCommand,
    update_original_response::{
        UpdateOriginalResponse, UpdateOriginalResponseError, UpdateOriginalResponseErrorType,
    },
};

use serde::Serialize;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::{application::command::CommandOption, id::ApplicationId};

/// The error created if the creation of interaction fails.
#[derive(Debug)]
pub struct InteractionError {
    pub(crate) kind: InteractionErrorType,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum InteractionErrorType {
    /// Application id was not set on the client.
    ApplicationIdNotPresent,
    /// Command name validation failed.
    CommandNameValidationFailed,
    /// Command description validation failed.
    CommandDescriptionValidationFailed,
    /// Required command options have to be passed before optional ones.
    CommandOptionsRequiredFirst {
        /// Index of the option that failed validation.
        index: usize,
    },
    /// More than 10 permission overwrites were set.
    TooManyCommandPermissions,
    /// Too many commands have been provided.
    ///
    /// The maximum number of commands is defined by
    /// [`InteractionError::GUILD_COMMAND_LIMIT`].
    TooManyCommands,
}

impl InteractionError {
    /// Maximum number of commands an application may have in an individual
    /// guild.
    pub const GUILD_COMMAND_LIMIT: usize = 100;

    /// Maximum number of permission overwrites an application may have in an
    /// individual guild command.
    pub const GUILD_COMMAND_PERMISSION_LIMIT: usize = 10;

    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &InteractionErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[allow(clippy::unused_self)]
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        None
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(self) -> (InteractionErrorType, Option<Box<dyn Error + Send + Sync>>) {
        (self.kind, None)
    }
}

impl Display for InteractionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.kind {
            InteractionErrorType::ApplicationIdNotPresent => {
                f.write_str("application id not present")
            }
            InteractionErrorType::CommandNameValidationFailed => {
                f.write_str("command name must be between 3 and 32 characters")
            }
            InteractionErrorType::CommandDescriptionValidationFailed => {
                f.write_str("command description must be between 1 and 100 characters")
            }
            InteractionErrorType::CommandOptionsRequiredFirst { .. } => {
                f.write_str("optional command options must be added after required")
            }
            InteractionErrorType::TooManyCommandPermissions => {
                f.write_str("more than ")?;
                Display::fmt(&InteractionError::GUILD_COMMAND_PERMISSION_LIMIT, f)?;

                f.write_str(" permission overwrites were set")
            }
            InteractionErrorType::TooManyCommands => {
                f.write_str("more than ")?;
                Display::fmt(&InteractionError::GUILD_COMMAND_LIMIT, f)?;

                f.write_str(" commands were set")
            }
        }
    }
}

impl Error for InteractionError {}

/// Version of [`Command`] but with borrowed fields.
///
/// [`Command`]: twilight_model::application::command::Command
#[derive(Serialize)]
struct CommandBorrowed<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<ApplicationId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_permission: Option<bool>,
    pub description: &'a str,
    pub name: &'a str,
    #[serde(default)]
    pub options: Option<&'a [CommandOption]>,
}

#[cfg(test)]
mod tests {
    use super::CommandBorrowed;
    use twilight_model::{
        application::command::{BaseCommandOptionData, Command, CommandOption},
        id::{ApplicationId, CommandId, GuildId},
    };

    /// Test to convert a `Command` to a `CommandBorrowed`.
    ///
    /// Notably the point of this is to ensure that if a field is added to
    /// `Command` or a type is changed then the destructure of it and creation
    /// of `CommandBorrowed` will fail.
    #[test]
    fn test_command_borrowed_from_command() {
        let command = Command {
            application_id: Some(ApplicationId::new(1).expect("non zero")),
            default_permission: Some(true),
            description: "command description".to_owned(),
            guild_id: Some(GuildId::new(2).expect("non zero")),
            name: "command name".to_owned(),
            id: Some(CommandId::new(3).expect("non zero")),
            options: Vec::from([CommandOption::Boolean(BaseCommandOptionData {
                description: "command description".to_owned(),
                name: "command name".to_owned(),
                required: true,
            })]),
        };

        let _ = CommandBorrowed {
            application_id: command.application_id,
            default_permission: command.default_permission,
            description: &command.description,
            name: &command.name,
            options: Some(&command.options),
        };
    }
}
