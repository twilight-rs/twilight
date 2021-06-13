//! Used for building commands to send to Discord.

pub mod permissions;

mod option;

pub use self::option::{
    BaseCommandOptionData, ChoiceCommandOptionData, CommandOption, CommandOptionChoice,
    CommandOptionType, OptionsCommandOptionData,
};

use crate::id::{ApplicationId, CommandId, GuildId};
use serde::{Deserialize, Serialize};

/// Data sent to discord to create a command.
///
/// [`CommandOption`]s that are required must be listed before optional ones.
/// Command names must be lower case, matching the Regex `^[\w-]{1,32}$`. Refer
/// to [the discord docs] for more information.
///
/// [the discord docs]: https://discord.com/developers/docs/interactions/slash-commands#applicationcommand
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
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<CommandId>,
    #[serde(default)]
    pub options: Vec<CommandOption>,
}
