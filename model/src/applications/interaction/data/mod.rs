mod command;

pub use self::command::{CommandData, CommandDataOption, CommandInteractionDataResolved};

use serde::{Deserialize, Serialize};

/// Used during deserializing to contain the different data types.
///
/// There is no direct doc equivalent.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
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
