use std::convert::{TryFrom, TryInto};
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::applications::command::CommandCallbackData;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Payload used for responding to an interaction.
///
/// Refer to [the discord docs] for more information.
///
/// [the discord docs]: https://discord.com/developers/docs/interactions/slash-commands#interaction-response
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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
    pub fn kind(&self) -> InteractionResponseType {
        match self {
            InteractionResponse::Pong => InteractionResponseType::Pong,
            InteractionResponse::Acknowledge => InteractionResponseType::Acknowledge,
            InteractionResponse::ChannelMessage(_) => InteractionResponseType::ChannelMessage,
            InteractionResponse::ChannelMessageWithSource(_) => {
                InteractionResponseType::ChannelMessageWithSource
            }
            InteractionResponse::AckWithSource => InteractionResponseType::AckWithSource,
        }
    }

    // data is intentionally not exported because it's highly likely that
    // CommandCallbackData will not be the only additional data contained in a
    // response.
    fn data(&self) -> Option<&CommandCallbackData> {
        match self {
            InteractionResponse::ChannelMessage(d)
            | InteractionResponse::ChannelMessageWithSource(d) => Some(d),
            InteractionResponse::Pong
            | InteractionResponse::Acknowledge
            | InteractionResponse::AckWithSource => None,
        }
    }
}

impl<'de> Deserialize<'de> for InteractionResponse {
    fn deserialize<D>(deserializer: D) -> Result<InteractionResponse, D::Error>
    where
        D: Deserializer<'de>,
    {
        let envelope = InteractionResponseEnvelope::deserialize(deserializer)?;
        envelope.try_into().map_err(serde::de::Error::custom)
    }
}

impl Serialize for InteractionResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        InteractionResponseEnvelope {
            kind: self.kind(),
            data: self.data().cloned()
        }
        .serialize(serializer)
    }
}

impl<'a> TryFrom<InteractionResponseEnvelope> for InteractionResponse {
    type Error = InteractionResponseEnvelopeParseError;

    fn try_from(envelope: InteractionResponseEnvelope) -> Result<Self, Self::Error> {
        let i = match envelope.kind {
            InteractionResponseType::Pong => InteractionResponse::Pong,
            InteractionResponseType::Acknowledge => InteractionResponse::Acknowledge,
            InteractionResponseType::ChannelMessage => {
                InteractionResponse::ChannelMessage(envelope.data.ok_or(
                    InteractionResponseEnvelopeParseError::MissingData(envelope.kind),
                )?)
            }
            InteractionResponseType::ChannelMessageWithSource => {
                InteractionResponse::ChannelMessageWithSource(envelope.data.ok_or(
                    InteractionResponseEnvelopeParseError::MissingData(envelope.kind),
                )?)
            }
            InteractionResponseType::AckWithSource => InteractionResponse::AckWithSource,
        };

        Ok(i)
    }
}

#[derive(Debug)]
enum InteractionResponseEnvelopeParseError {
    MissingData(InteractionResponseType),
}

impl Display for InteractionResponseEnvelopeParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::MissingData(kind) => {
                write!(f, "data not present, but required for {}", kind.name())
            }
        }
    }
}

/// Raw payload sent when responding to an interaction.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
struct InteractionResponseEnvelope {
    #[serde(rename = "type")]
    pub kind: InteractionResponseType,
    pub data: Option<CommandCallbackData>,
}

/// Contains the possible response type integers for an interaction.
#[derive(
    Clone, Copy, Debug, Deserialize_repr, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize_repr,
)]
#[repr(u8)]
pub enum InteractionResponseType {
    Pong = 1,
    Acknowledge = 2,
    ChannelMessage = 3,
    ChannelMessageWithSource = 4,
    AckWithSource = 5,
}

impl InteractionResponseType {
    pub fn name(self) -> &'static str {
        match self {
            InteractionResponseType::Pong => "Pong",
            InteractionResponseType::Acknowledge => "Acknowledge",
            InteractionResponseType::ChannelMessage => "ChannelMessage",
            InteractionResponseType::ChannelMessageWithSource => "ChannelMessageWithSource",
            InteractionResponseType::AckWithSource => "AckWithSource",
        }
    }
}
