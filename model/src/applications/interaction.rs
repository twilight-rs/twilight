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

#[cfg(test)]
mod test {
    use crate::applications::*;
    use crate::user::User;
    use crate::guild::Permissions;
    use crate::user::UserFlags;
    use crate::guild::PartialMember;
    use crate::applications::interaction_data::ApplicationCommandInteractionData;
    use crate::id::*;

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

        let expected = Interaction::WithGuildId(GuildInteraction {
            data: InteractionData::ApplicationCommand(ApplicationCommandInteractionData {
                options: vec![InteractionDataOption::String {
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
                nick: None,
                mute: false,
                joined_at: Some("2017-03-13T19:19:14.040000+00:00".to_string()),
                deaf: false
            },
            interaction: BaseInteraction {
                id: 786008729715212338.into(),
                kind: InteractionType::ApplicationCommand,
                token: "A_UNIQUE_TOKEN".to_string(),
            }
        });

        let actual = serde_json::from_str::<Interaction>(&json).unwrap();

        assert_eq!(expected, actual);
    }
}
