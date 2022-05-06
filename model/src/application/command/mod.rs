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
use std::collections::HashMap;

/// Data sent to Discord to create a command.
///
/// [`CommandOption`]s that are required must be listed before optional ones.
/// Command names must be lower case, matching the Regex `^[\w-]{1,32}$`. See
/// [Discord Docs/Application Command Object].
///
/// This struct has an [associated builder] in the [`twilight-util`] crate.
///
/// [`twilight-util`]: https://docs.rs/twilight-util/latest/index.html
/// [associated builder]: https://docs.rs/twilight-util/latest/twilight_util/builder/command/struct.CommandBuilder.html
/// [Discord Docs/Application Command Object]: https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-structure
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
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
    /// Localization dictionary for the `description` field.
    ///
    /// See [Discord Docs/Localization].
    ///
    /// [Discord Docs/Localization]: https://discord.com/developers/docs/interactions/application-commands#localization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_localizations: Option<HashMap<String, String>>,
    /// Guild ID of the command, if not global.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id<CommandMarker>>,
    #[serde(rename = "type")]
    pub kind: CommandType,
    pub name: String,
    /// Localization dictionary for the `name` field.
    ///
    /// Keys should be valid locales. See [Discord Docs/Locales],
    /// [Discord Docs/Localization].
    ///
    /// [Discord Docs/Locales]: https://discord.com/developers/docs/reference#locales
    /// [Discord Docs/Localization]: https://discord.com/developers/docs/interactions/application-commands#localization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_localizations: Option<HashMap<String, String>>,
    #[serde(default)]
    pub options: Vec<CommandOption>,
    /// Autoincrementing version identifier.
    pub version: Id<CommandVersionMarker>,
}
