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
    get_guild_commands::GetGuildCommands, set_global_commands::SetGlobalCommands,
    set_guild_commands::SetGuildCommands, update_command_permissions::UpdateCommandPermissions,
    update_global_command::UpdateGlobalCommand, update_guild_command::UpdateGuildCommand,
};

use serde::Serialize;
use std::collections::HashMap;
use twilight_model::{
    application::command::{CommandOption, CommandType},
    guild::Permissions,
    id::{marker::ApplicationMarker, Id},
};

/// Version of [`Command`] but with borrowed fields.
///
/// [`Command`]: twilight_model::application::command::Command
#[derive(Serialize)]
struct CommandBorrowed<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<Id<ApplicationMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_member_permissions: Option<Permissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dm_permission: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_localizations: Option<&'a HashMap<String, String>>,
    #[serde(rename = "type")]
    pub kind: CommandType,
    pub name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_localizations: Option<&'a HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
    #[serde(default)]
    pub options: Option<&'a [CommandOption]>,
}

#[cfg(test)]
mod tests {
    use super::CommandBorrowed;
    use std::collections::HashMap;
    use twilight_model::{
        application::command::{Command, CommandType},
        guild::Permissions,
        id::Id,
    };

    /// Test to convert a `Command` to a `CommandBorrowed`.
    ///
    /// Notably the point of this is to ensure that if a field is added to
    /// `Command` or a type is changed then the destructure of it and creation
    /// of `CommandBorrowed` will fail.
    #[test]
    #[allow(deprecated)]
    fn command_borrowed_from_command() {
        let command = Command {
            application_id: Some(Id::new(1)),
            contexts: None,
            default_member_permissions: Some(Permissions::ADMINISTRATOR),
            dm_permission: Some(true),
            description: "command description".to_owned(),
            description_localizations: Some(HashMap::from([(
                "en-US".to_owned(),
                "command description".to_owned(),
            )])),
            guild_id: Some(Id::new(2)),
            id: Some(Id::new(3)),
            integration_types: None,
            kind: CommandType::ChatInput,
            name: "command name".to_owned(),
            name_localizations: Some(HashMap::from([(
                "en-US".to_owned(),
                "command name".to_owned(),
            )])),
            nsfw: Some(true),
            options: Vec::new(),
            version: Id::new(1),
        };
        _ = CommandBorrowed {
            application_id: command.application_id,
            default_member_permissions: command.default_member_permissions,
            dm_permission: command.dm_permission,
            description: Some(&command.description),
            description_localizations: command.description_localizations.as_ref(),
            kind: CommandType::ChatInput,
            name: &command.name,
            name_localizations: command.name_localizations.as_ref(),
            nsfw: command.nsfw,
            options: Some(&command.options),
        };
    }
}
