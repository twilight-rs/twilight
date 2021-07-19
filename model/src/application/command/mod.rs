//! Used for building commands to send to Discord.

pub mod permissions;

mod command_type;
mod option;

pub use self::{
    command_type::CommandType,
    option::{
        BaseCommandOptionData, ChoiceCommandOptionData, CommandOption, CommandOptionChoice,
        CommandOptionType, OptionsCommandOptionData,
    },
};

use crate::id::{ApplicationId, CommandId, GuildId};
use serde::{Deserialize, Serialize};

/// Data sent to discord to create a command.
///
/// [`CommandOption`]s that are required must be listed before optional ones.
/// Command names must be lower case, matching the Regex `^[\w-]{1,32}$`. Refer
/// to [the discord docs] for more information.
///
/// [the discord docs]: https://discord.com/developers/docs/interactions/application-commands#applicationcommand
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Command {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<ApplicationId>,
    /// Guild ID of the command, if not global.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_permission: Option<bool>,
    /// Description of the command.
    ///
    /// For [`User`] and [`Message`] commands, this will be an empty string.
    ///
    /// [`User`]: CommandType::User
    /// [`Message`]: CommandType::Message
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<CommandId>,
    #[serde(rename = "type")]
    pub kind: CommandType,
    #[serde(default)]
    pub options: Vec<CommandOption>,
}

/// Builder to create a [`Command`].
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CommandBuilder(Command);

impl CommandBuilder {
    /// Creates a builder to construct a [`Command`].
    pub const fn new(name: String, description: String) -> Self {
        Self(Command {
            application_id: None,
            guild_id: None,
            name,
            default_permission: None,
            description,
            id: None,
            options: Vec::new(),
        })
    }

    /// Consume the builder, returning a [`Command`].
    #[allow(clippy::missing_const_for_fn)]
    pub fn build(self) -> Command {
        self.0
    }

    /// Sets the application id of the command.
    ///
    /// Defaults to [`None`].
    pub const fn application_id(mut self, application_id: ApplicationId) -> Self {
        self.0.application_id = Some(application_id);

        self
    }

    /// Sets the guild id of the command.
    ///
    /// Defaults to [`None`].
    pub const fn guild_id(mut self, guild_id: GuildId) -> Self {
        self.0.guild_id = Some(guild_id);

        self
    }

    /// Sets the default permission of the command.
    ///
    /// Defaults to [`None`].
    pub const fn default_permission(mut self, default_permission: bool) -> Self {
        self.0.default_permission = Some(default_permission);

        self
    }

    /// Sets the id of the command.
    ///
    /// Defaults to [`None`].
    pub const fn id(mut self, id: CommandId) -> Self {
        self.0.id = Some(id);

        self
    }

    /// Sets the options of the command.
    ///
    /// Defaults to an empty list.
    pub fn options(mut self, options: Vec<CommandOption>) -> Self {
        self.0.options = options;

        self
    }
}
