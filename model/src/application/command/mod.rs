//! Used for building commands to send to Discord.

pub mod permissions;

mod command_type;
mod option;

pub use self::{
    command_type::CommandType,
    option::{
        BaseCommandOptionData, ChannelCommandOptionData, ChoiceCommandOptionData, CommandOption,
        CommandOptionChoice, CommandOptionType, CommandOptionValue, Number,
        NumberCommandOptionData, OptionsCommandOptionData,
    },
};

use crate::id::{
    marker::{ApplicationMarker, CommandMarker, CommandVersionMarker, GuildMarker},
    Id,
};
use serde::{Deserialize, Serialize};

/// Data sent to Discord to create a command.
///
/// [`CommandOption`]s that are required must be listed before optional ones.
/// Command names must be lower case, matching the Regex `^[\w-]{1,32}$`. See
/// [the Discord Docs/Application Command Object].
///
/// This struct has an [associated builder] in the [`twilight-util`] crate.
///
/// [`twilight-util`]: https://docs.rs/twilight-util/latest/index.html
/// [associated builder]: https://docs.rs/twilight-util/latest/builder/command/struct.CommandBuilder.html
/// [the Discord Docs/Application Command Object]: https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-structure
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Command {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<Id<ApplicationMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_permission: Option<bool>,
    /// Description of the command.
    ///
    /// For [`User`] and [`Message`] commands, this will be an empty string.
    ///
    /// [`User`]: CommandType::User
    /// [`Message`]: CommandType::Message
    pub description: String,
    /// Guild ID of the command, if not global.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id<CommandMarker>>,
    #[serde(rename = "type")]
    pub kind: CommandType,
    pub name: String,
    #[serde(default)]
    pub options: Vec<CommandOption>,
    /// Autoincrementing version identifier.
    pub version: Id<CommandVersionMarker>,
}
