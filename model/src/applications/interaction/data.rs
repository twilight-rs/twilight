use crate::id::CommandId;
use serde::{Deserialize, Serialize};

/// Used during deserializing to contain the different data types.
///
/// There is no direct doc equivalent.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(untagged)]
pub enum InteractionData {
    Ping,
    ApplicationCommand(CommandData),
}

impl InteractionData {
    pub fn kind(&self) -> &'static str {
        match self {
            InteractionData::Ping => "Ping",
            InteractionData::ApplicationCommand(_) => "ApplicationCommand",
        }
    }
}

/// Data received when an [`ApplicationCommand`] interaction is executed.
///
/// Refer to [the discord docs] for more information.
///
/// [`ApplicationCommand`]: crate::applications::interaction::Interaction::ApplicationCommand
/// [the discord docs]: https://discord.com/developers/docs/interactions/slash-commands#interaction-applicationcommandinteractiondata
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, Default)]
pub struct CommandData {
    pub id: CommandId,
    pub name: String,
    #[serde(default)]
    pub options: Vec<CommandDataOption>,
}

/// Data received when a user fills in a command option.
///
/// Note: user, channel, and role option types will be returned as a String
/// option here.
///
/// Refer to [the discord docs] for more information.
///
/// [the discord docs]: https://discord.com/developers/docs/interactions/slash-commands#interaction-applicationcommandinteractiondataoption
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
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
    pub fn kind(&self) -> &'static str {
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
