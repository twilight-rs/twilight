use super::{InteractionData, InteractionType};
use crate::guild::PartialMember;
use crate::id::*;
use serde::{self, Deserialize, Deserializer, Serialize};
use std::convert::{TryFrom, TryInto};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// The payload received when a user executes an interaction.
///
/// Refer to [the discord docs] for more information.
///
/// [the discord docs]: https://discord.com/developers/docs/interactions/slash-commands#interaction
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Interaction {
    /// Global interactions do not originate from a guild.
    Global(BaseInteraction),
    /// Guild interactions originate from within a guild.
    Guild(GuildInteraction),
}

impl<'de> Deserialize<'de> for Interaction {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let envelope = InteractionEnvelope::deserialize(deserializer)?;
        envelope.try_into().map_err(serde::de::Error::custom)
    }
}

/// Common fields between different [`Interaction`](crate::applications::Interaction) types.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct BaseInteraction {
    pub id: InteractionId,
    #[serde(rename = "type")]
    pub kind: InteractionType,
    pub token: String,
}

/// The payload received when an interaction originated from a guild.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GuildInteraction {
    /// The guild the interaction was triggered from.
    pub guild_id: GuildId,
    /// The channel the interaction was triggered from.
    pub channel_id: ChannelId,
    /// The member that triggered the interaction.
    pub member: PartialMember,
    /// The data corresponding to the InteractionType.
    pub data: InteractionData,
    /// Common interaction fields.
    #[serde(flatten)]
    pub interaction: BaseInteraction,
}

impl<'a> TryFrom<InteractionEnvelope> for Interaction {
    type Error = InteractionEnvelopeParseError;

    fn try_from(mut envelope: InteractionEnvelope) -> Result<Self, Self::Error> {
        let data = envelope.data()?;
        let base_interaction = BaseInteraction {
            id: envelope.id,
            kind: envelope.kind,
            token: envelope.token,
        };

        match data {
            InteractionData::Ping => Ok(Interaction::Global(base_interaction)),
            InteractionData::ApplicationCommand(cmd) => Ok(Interaction::Guild(GuildInteraction {
                guild_id: envelope.guild_id.unwrap(),
                channel_id: envelope.channel_id.unwrap(),
                member: envelope.member.unwrap(),
                data: InteractionData::ApplicationCommand(cmd),
                interaction: base_interaction,
            })),
        }
    }
}

/// The raw interaction payload received from Discord. It is checked and parsed
/// into an [`Interaction`](crate::applications::Interaction).
///
/// Only used internally.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
struct InteractionEnvelope {
    pub id: InteractionId,
    #[serde(rename = "type")]
    pub kind: InteractionType,
    pub guild_id: Option<GuildId>,
    pub channel_id: Option<ChannelId>,
    pub member: Option<PartialMember>,
    pub token: String,

    data: Option<InteractionData>,
}

#[derive(Debug)]
enum InteractionEnvelopeParseError {
    MissingData(InteractionType),
    DataMismatch {
        wanted: InteractionData,
        got: InteractionData,
    },
}

impl Display for InteractionEnvelopeParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::MissingData(kind) => {
                write!(f, "data not present, but required for {}", kind.name())
            }
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
        match self.kind {
            InteractionType::Ping => Ok(InteractionData::Ping),
            InteractionType::ApplicationCommand => {
                let data = self
                    .data
                    .take()
                    .ok_or(InteractionEnvelopeParseError::MissingData(self.kind))?;

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

#[cfg(test)]
mod test {
    use crate::applications::CommandData;
    use crate::applications::*;
    use crate::guild::PartialMember;
    use crate::guild::Permissions;
    use crate::id::*;
    use crate::user::User;
    use crate::user::UserFlags;

    #[test]
    fn test_interaction() {
        let json = r#"{
    "type": 2,
    "token": "A_UNIQUE_TOKEN",
    "member": {
        "user": {
            "id": "53908232506183680",
            "username": "Mason",
            "avatar": "a_d5efa99b3eeaa7dd43acca82f5692432",
            "discriminator": "1337",
            "public_flags": 131141
        },
        "roles": ["539082325061836999"],
        "premium_since": null,
        "permissions": "2147483647",
        "pending": false,
        "nick": null,
        "mute": false,
        "joined_at": "2017-03-13T19:19:14.040000+00:00",
        "is_pending": false,
        "deaf": false
    },
    "id": "786008729715212338",
    "guild_id": "290926798626357999",
    "data": {
        "options": [{
            "name": "cardname",
            "value": "The Gitrog Monster"
        }],
        "name": "cardsearch",
        "id": "771825006014889984"
    },
    "channel_id": "645027906669510667"
}"#;

        let expected = Interaction::Guild(GuildInteraction {
            data: InteractionData::ApplicationCommand(CommandData {
                options: vec![CommandDataOption::String {
                    name: "cardname".to_string(),
                    value: "The Gitrog Monster".to_string(),
                }],
                name: "cardsearch".to_string(),
                id: 771825006014889984.into(),
            }),
            guild_id: 290926798626357999.into(),
            channel_id: 645027906669510667.into(),
            member: PartialMember {
                user: Some(User {
                    id: UserId(53908232506183680),
                    name: "Mason".to_string(),
                    avatar: Some("a_d5efa99b3eeaa7dd43acca82f5692432".to_string()),
                    discriminator: 1337.to_string(),
                    public_flags: UserFlags::from_bits(131141),
                    bot: false,
                    email: None,
                    flags: None,
                    locale: None,
                    mfa_enabled: None,
                    premium_type: None,
                    system: None,
                    verified: None,
                }),
                roles: vec![539082325061836999.into()],
                permissions: Permissions::from_bits(2147483647),
                premium_since: None,
                nick: None,
                mute: false,
                joined_at: Some("2017-03-13T19:19:14.040000+00:00".to_string()),
                deaf: false,
            },
            interaction: BaseInteraction {
                id: 786008729715212338.into(),
                kind: InteractionType::ApplicationCommand,
                token: "A_UNIQUE_TOKEN".to_string(),
            },
        });

        let actual = serde_json::from_str::<Interaction>(&json).unwrap();

        assert_eq!(expected, actual);
    }
}
