//! Used for building commands to send to Discord.

pub mod permissions;

mod command_type;
mod option;

pub use self::{
    command_type::CommandType,
    option::{
        BaseCommandOptionData, ChoiceCommandOptionData, CommandOption, CommandOptionChoice,
        CommandOptionType, Number, OptionsCommandOptionData,
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
/// This struct has an [associated builder] in the [`twilight-util`] crate.
///
/// [`twilight-util`]: https://api.twilight.rs/twilight_util/index.html
/// [associated builder]: https://api.twilight.rs/twilight_util/builder/command/struct.CommandBuilder.html
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
