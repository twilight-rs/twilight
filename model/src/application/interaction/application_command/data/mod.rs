mod resolved;

pub use self::resolved::{CommandInteractionDataResolved, InteractionChannel, InteractionMember};

use crate::id::CommandId;
use serde::{Deserialize, Serialize};

/// Data received when an [`ApplicationCommand`] interaction is executed.
///
/// Refer to [the discord docs] for more information.
///
/// [`ApplicationCommand`]: crate::application::interaction::Interaction::ApplicationCommand
/// [the discord docs]: https://discord.com/developers/docs/interactions/application-commands#interaction-applicationcommandinteractiondata
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CommandData {
    /// ID of the command.
    pub id: CommandId,
    /// Name of the command.
    pub name: String,
    /// List of parsed options specified by the user.
    #[serde(default)]
    pub options: Vec<CommandDataOption>,
    /// Data sent if any of the options are discord types.
    pub resolved: Option<CommandInteractionDataResolved>,
}

/// Data received when a user fills in a command option.
///
/// Note: user, channel, role, and mentionable option types will be returned as
/// a [`String`] option here.
///
/// Refer to [the discord docs] for more information.
///
/// [the discord docs]: https://discord.com/developers/docs/interactions/application-commands#interaction-applicationcommandinteractiondataoption
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CommandDataOption {
    String {
        name: String,
        value: String,
    },
    Integer {
        name: String,
        value: i64,
    },
    Boolean {
        name: String,
        value: bool,
    },
    SubCommand {
        name: String,
        #[serde(default)]
        options: Vec<CommandDataOption>,
    },
}

impl CommandDataOption {
    pub const fn kind(&self) -> &'static str {
        match self {
            CommandDataOption::String { .. } => "String",
            CommandDataOption::Integer { .. } => "Integer",
            CommandDataOption::Boolean { .. } => "Boolean",
            CommandDataOption::SubCommand { .. } => "SubCommand",
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::String { name, .. }
            | Self::Integer { name, .. }
            | Self::Boolean { name, .. }
            | Self::SubCommand { name, .. } => name,
        }
    }
}
