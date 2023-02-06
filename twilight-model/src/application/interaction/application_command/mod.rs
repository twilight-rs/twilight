//! [`APPLICATION_COMMAND`] interaction.
//!
//! [`APPLICATION_COMMAND`]: crate::application::interaction::InteractionType::APPLICATION_COMMAND

mod option;
mod resolved;

pub use self::{
    option::{CommandDataOption, CommandOptionValue},
    resolved::{CommandInteractionDataResolved, InteractionChannel, InteractionMember},
};

use crate::{
    application::command::CommandType,
    id::{
        marker::{CommandMarker, GenericMarker, GuildMarker},
        Id,
    },
};
use serde::{Deserialize, Serialize};

/// Data received when an [`APPLICATION_COMMAND`] or
/// [`APPLICATION_COMMAND_AUTOCOMPLETE`] interaction is executed.
///
/// See [Discord Docs/Application Command Data Structure].
///
/// [`APPLICATION_COMMAND`]: crate::application::interaction::InteractionType::APPLICATION_COMMAND
/// [`APPLICATION_COMMAND_AUTOCOMPLETE`]: crate::application::interaction::InteractionType::APPLICATION_COMMAND_AUTOCOMPLETE
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
    pub resolved: Option<CommandInteractionDataResolved>,
    /// If this is a user or message command, the ID of the targeted user/message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<Id<GenericMarker>>,
}
