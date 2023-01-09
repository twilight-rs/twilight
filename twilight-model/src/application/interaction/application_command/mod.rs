//! [`ApplicationCommand`] interaction.
//!
//! [`ApplicationCommand`]: crate::application::interaction::InteractionType::ApplicationCommand

mod option;

pub use self::option::{CommandDataOption, CommandOptionValue};

use crate::{
    application::{command::CommandType, interaction::resolved::InteractionDataResolved},
    id::{
        marker::{CommandMarker, GenericMarker, GuildMarker},
        Id,
    },
};
use serde::{Deserialize, Serialize};

/// Data received when an [`ApplicationCommand`] or [`ApplicationCommandAutocomplete`]
/// interaction is executed.
///
/// See [Discord Docs/Application Command Data Structure].
///
/// [`ApplicationCommand`]: crate::application::interaction::InteractionType::ApplicationCommand
/// [`ApplicationCommandAutocomplete`]: crate::application::interaction::InteractionType::ApplicationCommandAutocomplete
/// [Discord Docs/Application Command Data Structure]: https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-application-command-data-structure
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CommandData {
    /// ID of the guild the command is registered to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    /// ID of the command.
    pub id: Id<CommandMarker>,
    /// Name of the command.
    pub name: String,
    /// Type of the command.
    #[serde(rename = "type")]
    pub kind: CommandType,
    /// List of options specified by the user.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<CommandDataOption>,
    /// Resolved data from the interaction's options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved: Option<InteractionDataResolved>,
    /// If this is a user or message command, the ID of the targeted user/message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<Id<GenericMarker>>,
}
