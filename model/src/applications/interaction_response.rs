use super::CommandCallbackData;
use serde::{Deserialize, Serialize, Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// InteractionResponse is the payload for responding to an interaction.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq)]
pub enum InteractionResponse {
    /// Used when responding to an interaction of type Ping.
    Pong,
    /// Acknowledges an interaction without sending a message.
    Acknowledge,
    /// Responds to an interaction with a regular message.
    ChannelMessage(CommandCallbackData),
    /// Responds to an interaction with a message showing the original command.
    ChannelMessageWithSource(CommandCallbackData),
    /// Acknowledges an interaction, showing the original command.
    AckWithSource,
}

impl InteractionResponse {
    fn kind(&self) -> InteractionResponseType {
        match self {
            InteractionResponse::Pong => InteractionResponseType::Pong,
            InteractionResponse::Acknowledge => InteractionResponseType::Acknowledge,
            InteractionResponse::ChannelMessage(_) => InteractionResponseType::ChannelMessage,
            InteractionResponse::ChannelMessageWithSource(_) => {
                InteractionResponseType::ChannelMessageWithSource
            }
            InteractionResponse::AckWithSource => InteractionResponseType::ACKWithSource,
        }
    }

    fn data(&self) -> Option<CommandCallbackData> {
        match self {
            InteractionResponse::Pong => None,
            InteractionResponse::Acknowledge => None,
            InteractionResponse::ChannelMessage(d) => Some(d.clone()),
            InteractionResponse::ChannelMessageWithSource(d) => Some(d.clone()),
            InteractionResponse::AckWithSource => None,
        }
    }
}

impl Serialize for InteractionResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        InteractionResponseEnvelope {
            kind: self.kind(),
            data: self.data(),
        }
        .serialize(serializer)
    }
}

/// InteractionResponseEnvelope is the raw payload sent when responding to an
/// interaction.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
struct InteractionResponseEnvelope {
    #[serde(rename = "type")]
    pub kind: InteractionResponseType,
    pub data: Option<CommandCallbackData>,
}

/// InteractionResponseType denotes the possible response types for an
/// interaction.
#[derive(
    Clone, Copy, Debug, Deserialize_repr, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize_repr,
)]
#[repr(u8)]
enum InteractionResponseType {
    Pong = 1,
    Acknowledge = 2,
    ChannelMessage = 3,
    ChannelMessageWithSource = 4,
    ACKWithSource = 5,
}
