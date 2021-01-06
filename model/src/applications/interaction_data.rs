use super::command::CommandData;
use serde::{Deserialize, Serialize};

/// An enum containing extra data corresponding to which interaction type was
/// received.
///
/// There is no direct doc equivalent.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(untagged)]
pub enum InteractionData {
    Ping,
    ApplicationCommand(CommandData),
}

impl InteractionData {
    pub fn name(&self) -> &'static str {
        match self {
            InteractionData::Ping => "Ping",
            InteractionData::ApplicationCommand(_) => "ApplicationCommand",
        }
    }
}
