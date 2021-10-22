pub mod create_global_command;
pub mod create_guild_command;

mod delete_global_command;
mod delete_guild_command;
mod get_command_permissions;
mod get_global_command;
mod get_global_commands;
mod get_guild_command;
mod get_guild_command_permissions;
mod get_guild_commands;
mod set_command_permissions;
mod set_global_commands;
mod set_guild_commands;
mod update_command_permissions;
mod update_global_command;
mod update_guild_command;

pub use self::{
    create_global_command::CreateGlobalCommand, create_guild_command::CreateGuildCommand,
    delete_global_command::DeleteGlobalCommand, delete_guild_command::DeleteGuildCommand,
    get_command_permissions::GetCommandPermissions, get_global_command::GetGlobalCommand,
    get_global_commands::GetGlobalCommands, get_guild_command::GetGuildCommand,
    get_guild_command_permissions::GetGuildCommandPermissions,
    get_guild_commands::GetGuildCommands, set_command_permissions::SetCommandPermissions,
    set_global_commands::SetGlobalCommands, set_guild_commands::SetGuildCommands,
    update_command_permissions::UpdateCommandPermissions,
    update_global_command::UpdateGlobalCommand, update_guild_command::UpdateGuildCommand,
};

use serde::Serialize;
use twilight_model::{
    application::command::{CommandOption, CommandType},
    id::ApplicationId,
};

/// Version of [`Command`] but with borrowed fields.
///
/// [`Command`]: twilight_model::application::command::Command
#[derive(Serialize)]
struct CommandBorrowed<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<ApplicationId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_permission: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    pub kind: CommandType,
    pub name: &'a str,
    #[serde(default)]
    pub options: Option<&'a [CommandOption]>,
}

#[cfg(test)]
mod tests {
    use super::CommandBorrowed;
    use twilight_model::{
        application::command::{BaseCommandOptionData, Command, CommandOption, CommandType},
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
            kind: CommandType::ChatInput,
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
            description: Some(&command.description),
            kind: CommandType::ChatInput,
            name: &command.name,
            options: Some(&command.options),
        };
    }
}
