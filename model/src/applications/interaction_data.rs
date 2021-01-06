use super::InteractionDataOption;
use crate::id::CommandId;
use serde::{Deserialize, Serialize};

/// InteractionData is an enum containing extra data corresponding to which
/// interaction type was received.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(untagged)]
pub enum InteractionData {
    Ping,
    ApplicationCommand(CommandInteractionData),
}

impl InteractionData {
    pub fn name(&self) -> &'static str {
        match self {
            InteractionData::Ping => "Ping",
            InteractionData::ApplicationCommand(_) => "ApplicationCommand",
        }
    }
}

/// CommandInteractionData is the data received when an ApplicationCommand
/// interaction is executed.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, Default)]
pub struct CommandInteractionData {
    pub id: CommandId,
    pub name: String,
    #[serde(default)]
    pub options: Vec<InteractionDataOption>,
}
