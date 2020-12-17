use super::{InteractionData, InteractionType};
use crate::guild::PartialMember;
use crate::id::*;
use serde::{Deserialize, Serialize};

use std::fmt::{Formatter, Result as FmtResult};
use std::{
    convert::{TryFrom, TryInto},
    fmt::Display,
};

/*
 * # Interaction
 *
 * | Field          | Type                              |
 * |----------------|-----------------------------------|
 * | id             | snowflake                         |
 * | type           | InteractionType                   |
 * | data?\*        | ApplicationCommandInteractionData |
 * | guild_id       | snowflake                         |
 * | channel_id     | snowflake                         |
 * | member         | GuildMember                       |
 * | token          | string                            |
 */

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Interaction {
    WithGuildId(GuildInteraction),
    Global(BaseInteraction),
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct BaseInteraction {
    pub id: InteractionId,
    #[serde(rename = "type")]
    pub kind: InteractionType,
    pub token: String,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GuildInteraction {
    pub data: InteractionData,
    pub guild_id: GuildId,
    pub channel_id: ChannelId,
    pub member: PartialMember,
    #[serde(flatten)]
    pub interaction: BaseInteraction,
}

impl<'a> TryFrom<InteractionEnvelope> for Interaction {
    type Error = InteractionEnvelopeParseError;

    fn try_from(mut envelope: InteractionEnvelope) -> Result<Self, Self::Error> {
        let data = envelope.data()?;
        let base_interaction = BaseInteraction {
            id: envelope.id,
            kind: envelope.kind.try_into()?,
            token: envelope.token,
        };

        match data {
            InteractionData::Ping => Ok(Interaction::Global(base_interaction)),
            InteractionData::ApplicationCommand(cmd) => {
                Ok(Interaction::WithGuildId(GuildInteraction {
                    guild_id: envelope.guild_id.unwrap(),
                    channel_id: envelope.channel_id.unwrap(),
                    member: envelope.member.unwrap(),
                    data: InteractionData::ApplicationCommand(cmd),
                    interaction: base_interaction,
                }))
            }
        }
    }
}

/// Raw incoming payload from gateway/http.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct InteractionEnvelope {
    pub id: InteractionId,
    #[serde(rename = "type")]
    pub kind: u8,
    pub guild_id: Option<GuildId>,
    pub channel_id: Option<ChannelId>,
    pub member: Option<PartialMember>,
    pub token: String,

    data: Option<InteractionData>,
}

#[derive(Debug)]
pub enum InteractionEnvelopeParseError {
    DecodeError(Box<dyn std::error::Error>),
    MissingData,
    UnknownType(u8),
    DataMismatch {
        wanted: InteractionData,
        got: InteractionData,
    },
}

impl Display for InteractionEnvelopeParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::DecodeError(why) => write!(f, "decode interaction: {}", why),
            Self::MissingData => f.write_str("the data field was not present"),
            Self::UnknownType(kind) => write!(f, "unknown interaction type: {}", kind),
            Self::DataMismatch { wanted, got } => write!(
                f,
                "invalid data enum: wanted {} got {}",
                wanted.name(),
                got.name()
            ),
        }
    }
}

impl std::error::Error for InteractionEnvelopeParseError {}

impl InteractionEnvelope {
    pub fn data(&mut self) -> Result<InteractionData, InteractionEnvelopeParseError> {
        match self
            .kind
            .try_into()
            .map_err(|_| InteractionEnvelopeParseError::UnknownType(self.kind))?
        {
            InteractionType::Ping => Ok(InteractionData::Ping),
            InteractionType::ApplicationCommand => {
                let data = self
                    .data
                    .take()
                    .ok_or(InteractionEnvelopeParseError::MissingData)?;

                match data {
                    InteractionData::ApplicationCommand(_) => Ok(data),
                    _ => Err(InteractionEnvelopeParseError::DataMismatch {
                        got: data,
                        wanted: InteractionData::ApplicationCommand(Default::default()),
                    }),
                }
            }
        }
    }
}
