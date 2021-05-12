mod data;
mod kind;

pub use self::{
    data::{CommandData, CommandDataOption, InteractionData},
    kind::InteractionType,
};

use crate::{
    guild::PartialMember,
    id::{ApplicationId, ChannelId, GuildId, InteractionId},
    user::User,
};
use serde::{
    de::{Deserializer, Error as DeError},
    Deserialize, Serialize,
};
use std::{
    convert::{TryFrom, TryInto},
    fmt::{Display, Formatter, Result as FmtResult},
};

/// Payload received when a user executes an interaction.
///
/// Each variant corresponds to `InteractionType` in the discord docs. Refer to
/// [the discord docs] for more information.
///
/// [the discord docs]: https://discord.com/developers/docs/interactions/slash-commands#interaction
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum Interaction {
    /// Ping variant.
    Ping(InteractionPing),
    /// Application command variant.
    ApplicationCommand(InteractionApplicationCommand),
}

impl Interaction {
    pub fn guild_id(&self) -> Option<GuildId> {
        match self {
            Interaction::Ping(_) => None,
            Interaction::ApplicationCommand(inner) => inner.guild_id,
        }
    }
}

impl<'de> Deserialize<'de> for Interaction {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let envelope = InteractionEnvelope::deserialize(deserializer)?;
        envelope.try_into().map_err(DeError::custom)
    }
}

/// Data present in an [`Interaction`] of type [`Ping`].
///
/// [`Ping`]: Interaction::Ping
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct InteractionPing {
    /// ID of the associated application.
    pub application_id: ApplicationId,
    /// The id of the interaction
    pub id: InteractionId,
    #[serde(rename = "type")]
    /// The kind of the interaction
    pub kind: InteractionType,
    /// The token of the interaction
    pub token: String,
}

/// Data present in an [`Interaction`] of type [`ApplicationCommand`].
///
/// [`ApplicationCommand`]: Interaction::ApplicationCommand
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct InteractionApplicationCommand {
    /// ID of the associated application.
    pub application_id: ApplicationId,
    /// The channel the interaction was triggered from.
    pub channel_id: ChannelId,
    /// The data corresponding to the InteractionType.
    pub command_data: CommandData,
    /// The guild the interaction was triggered from.
    pub guild_id: Option<GuildId>,
    /// The id of the interaction
    pub id: InteractionId,
    #[serde(rename = "type")]
    /// The kind of the interaction
    pub kind: InteractionType,
    /// The member that triggered the interaction.
    pub member: Option<PartialMember>,
    /// The token of the interaction
    pub token: String,
    /// The user that triggered the interaction.
    pub user: Option<User>,
}

impl<'a> TryFrom<InteractionEnvelope> for Interaction {
    type Error = InteractionEnvelopeParseError;

    fn try_from(envelope: InteractionEnvelope) -> Result<Self, Self::Error> {
        match envelope.kind {
            InteractionType::Ping => Ok(Interaction::Ping(InteractionPing {
                application_id: envelope.application_id,
                id: envelope.id,
                kind: envelope.kind,
                token: envelope.token,
            })),
            InteractionType::ApplicationCommand => {
                let channel_id = match envelope.channel_id {
                    Some(id) => id,
                    None => return Err(Self::Error::MissingField("channel_id")),
                };

                let command_data = match envelope.data {
                    Some(InteractionData::ApplicationCommand(cmd)) => cmd,
                    Some(_) => {
                        return Err(Self::Error::DataMismatch {
                            wanted: "command_data",
                            got: "other kind of data",
                        });
                    }
                    None => return Err(Self::Error::MissingField("data")),
                };

                Ok(Interaction::ApplicationCommand(
                    InteractionApplicationCommand {
                        application_id: envelope.application_id,
                        channel_id,
                        command_data,
                        guild_id: envelope.guild_id,
                        id: envelope.id,
                        kind: envelope.kind,
                        member: envelope.member,
                        token: envelope.token,
                        user: envelope.user,
                    },
                ))
            }
        }
    }
}

/// Raw interaction payload received from Discord.
///
/// It is checked and parsed into an [`Interaction`].  Only used internally.
#[derive(Debug, Deserialize, Serialize)]
struct InteractionEnvelope {
    id: InteractionId,
    application_id: ApplicationId,
    #[serde(rename = "type")]
    kind: InteractionType,
    data: Option<InteractionData>,
    guild_id: Option<GuildId>,
    channel_id: Option<ChannelId>,
    member: Option<PartialMember>,
    user: Option<User>,
    token: String,
}

#[derive(Debug)]
enum InteractionEnvelopeParseError {
    DataMismatch {
        wanted: &'static str,
        got: &'static str,
    },
    MissingField(&'static str),
}

impl Display for InteractionEnvelopeParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::DataMismatch { wanted, got } => {
                write!(f, "invalid data: wanted {} got {}", wanted, got)
            }
            Self::MissingField(s) => write!(f, "The field {} was missing", s),
        }
    }
}

impl std::error::Error for InteractionEnvelopeParseError {}

#[cfg(test)]
mod test {
    use super::InteractionApplicationCommand;
    use crate::{
        applications::interaction::{
            data::{CommandData, CommandDataOption},
            Interaction, InteractionType,
        },
        guild::{PartialMember, Permissions},
        id::UserId,
        user::{User, UserFlags},
    };

    #[test]
    fn test_interaction() {
        let json = r#"{
    "type": 2,
    "token": "A_UNIQUE_TOKEN",
    "member": {
        "user": {
            "id": "100",
            "username": "Mason",
            "avatar": "avatar string",
            "discriminator": "1337",
            "public_flags": 131141
        },
        "roles": ["400"],
        "premium_since": null,
        "permissions": "2147483647",
        "pending": false,
        "nick": null,
        "mute": false,
        "joined_at": "2017-03-13T10:10:10.040000+00:00",
        "is_pending": false,
        "deaf": false
    },
    "id": "200",
    "application_id": "700",
    "guild_id": "300",
    "data": {
        "options": [{
            "name": "cardname",
            "value": "The Gitrog Monster"
        }],
        "name": "cardsearch",
        "id": "500"
    },
    "channel_id": "600"
}"#;

        let expected = Interaction::ApplicationCommand(InteractionApplicationCommand {
            application_id: 700.into(),
            channel_id: 600.into(),
            command_data: CommandData {
                options: vec![CommandDataOption::String {
                    name: "cardname".to_string(),
                    value: "The Gitrog Monster".to_string(),
                }],
                name: "cardsearch".to_string(),
                id: 500.into(),
                resolved: None,
            },
            guild_id: Some(300.into()),
            id: 200.into(),
            kind: InteractionType::ApplicationCommand,
            member: Some(PartialMember {
                user: Some(User {
                    id: UserId(100),
                    name: "Mason".to_string(),
                    avatar: Some("avatar string".to_string()),
                    discriminator: 1337.to_string(),
                    public_flags: UserFlags::from_bits(131_141),
                    bot: false,
                    email: None,
                    flags: None,
                    locale: None,
                    mfa_enabled: None,
                    premium_type: None,
                    system: None,
                    verified: None,
                }),
                roles: vec![400.into()],
                permissions: Permissions::from_bits(2_147_483_647),
                premium_since: None,
                nick: None,
                mute: false,
                joined_at: Some("2017-03-13T10:10:10.040000+00:00".to_string()),
                deaf: false,
            }),
            token: "A_UNIQUE_TOKEN".to_string(),
            user: None,
        });

        let actual = serde_json::from_str::<Interaction>(&json).unwrap();

        assert_eq!(expected, actual);
    }
}
