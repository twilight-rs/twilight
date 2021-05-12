use crate::{
    channel::ChannelType,
    guild::{Permissions, Role},
    id::{ChannelId, GuildId, RoleId, UserId},
    user::User,
    util::is_false,
};
use serde::{de::Deserializer, ser::Serializer, Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
struct CommandInteractionDataResolvedEnvelope {
    channels: HashMap<ChannelId, InteractionChannel>,
    members: HashMap<UserId, InteractionMemberEnvelope>,
    roles: HashMap<RoleId, Role>,
    users: HashMap<UserId, User>,
}

impl From<CommandInteractionDataResolved> for CommandInteractionDataResolvedEnvelope {
    fn from(resolved: CommandInteractionDataResolved) -> Self {
        let mut envelope = CommandInteractionDataResolvedEnvelope {
            channels: HashMap::new(),
            members: HashMap::new(),
            roles: HashMap::new(),
            users: HashMap::new(),
        };

        for channel in resolved.channels {
            envelope.channels.insert(channel.id, channel);
        }

        for member in resolved.members {
            envelope.members.insert(member.id, member.into());
        }

        for role in resolved.roles {
            envelope.roles.insert(role.id, role);
        }

        for user in resolved.users {
            envelope.users.insert(user.id, user);
        }

        envelope
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct InteractionMemberEnvelope {
    guild_id: GuildId,
    #[serde(skip_serializing_if = "Option::is_none")]
    hoisted_role: Option<RoleId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    joined_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nick: Option<String>,
    #[serde(default, skip_serializing_if = "is_false")]
    pending: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    premium_since: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    roles: Vec<RoleId>,
}

impl From<InteractionMember> for InteractionMemberEnvelope {
    fn from(member: InteractionMember) -> Self {
        Self {
            guild_id: member.guild_id,
            hoisted_role: member.hoisted_role,
            joined_at: member.joined_at,
            nick: member.nick,
            pending: member.pending,
            premium_since: member.premium_since,
            roles: member.roles,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandInteractionDataResolved {
    pub channels: Vec<InteractionChannel>,
    pub members: Vec<InteractionMember>,
    pub roles: Vec<Role>,
    pub users: Vec<User>,
}

impl<'de> Deserialize<'de> for CommandInteractionDataResolved {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(CommandInteractionDataResolvedEnvelope::deserialize(deserializer)?.into())
    }
}

impl From<CommandInteractionDataResolvedEnvelope> for CommandInteractionDataResolved {
    fn from(envelope: CommandInteractionDataResolvedEnvelope) -> Self {
        Self {
            channels: envelope.channels.into_iter().map(|(_, v)| v).collect(),
            members: envelope
                .members
                .into_iter()
                .map(|(k, v)| InteractionMember::from((k, v)))
                .collect(),
            roles: envelope.roles.into_iter().map(|(_, v)| v).collect(),
            users: envelope.users.into_iter().map(|(_, v)| v).collect(),
        }
    }
}

impl Serialize for CommandInteractionDataResolved {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let envelope: CommandInteractionDataResolvedEnvelope = self.clone().into();

        envelope.serialize(serializer)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct InteractionChannel {
    pub id: ChannelId,
    #[serde(rename = "type")]
    pub kind: ChannelType,
    pub name: String,
    pub permissions: Permissions,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InteractionMember {
    pub guild_id: GuildId,
    pub hoisted_role: Option<RoleId>,
    pub id: UserId,
    pub joined_at: Option<String>,
    pub nick: Option<String>,
    /// Whether the user has yet to pass the guild's [Membership Screening]
    /// requirements.
    pub pending: bool,
    pub premium_since: Option<String>,
    pub roles: Vec<RoleId>,
}

impl From<(UserId, InteractionMemberEnvelope)> for InteractionMember {
    fn from((id, envelope): (UserId, InteractionMemberEnvelope)) -> Self {
        Self {
            guild_id: envelope.guild_id,
            hoisted_role: envelope.hoisted_role,
            id,
            joined_at: envelope.joined_at,
            nick: envelope.nick,
            pending: envelope.pending,
            premium_since: envelope.premium_since,
            roles: envelope.roles,
        }
    }
}

impl Serialize for InteractionMember {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let envelope: InteractionMemberEnvelope = self.clone().into();

        envelope.serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use super::{CommandInteractionDataResolved, InteractionChannel, InteractionMember};
    use crate::{
        channel::ChannelType,
        guild::{Permissions, Role},
        id::{ChannelId, GuildId, RoleId, UserId},
        user::{PremiumType, User, UserFlags},
    };
    use serde_test::Token;

    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_data_resolved() {
        let value = CommandInteractionDataResolved {
            channels: vec![InteractionChannel {
                id: ChannelId(100),
                kind: ChannelType::GuildText,
                name: "channel name".into(),
                permissions: Permissions::empty(),
            }],
            members: vec![InteractionMember {
                guild_id: GuildId(200),
                hoisted_role: None,
                id: UserId(300),
                joined_at: Some("joined at".into()),
                nick: None,
                pending: false,
                premium_since: None,
                roles: Vec::new(),
            }],
            roles: vec![Role {
                color: 0,
                hoist: true,
                id: RoleId(400),
                managed: false,
                mentionable: true,
                name: "test".to_owned(),
                permissions: Permissions::ADMINISTRATOR,
                position: 12,
                tags: None,
            }],
            users: vec![User {
                avatar: Some("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_owned()),
                bot: false,
                discriminator: "0001".to_owned(),
                email: Some("address@example.com".to_owned()),
                flags: Some(UserFlags::EARLY_SUPPORTER | UserFlags::VERIFIED_BOT_DEVELOPER),
                id: UserId(300),
                locale: Some("en-us".to_owned()),
                mfa_enabled: Some(true),
                name: "test".to_owned(),
                premium_type: Some(PremiumType::Nitro),
                public_flags: Some(UserFlags::EARLY_SUPPORTER | UserFlags::VERIFIED_BOT_DEVELOPER),
                system: None,
                verified: Some(true),
            }],
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "CommandInteractionDataResolvedEnvelope",
                    len: 4,
                },
                Token::Str("channels"),
                Token::Map { len: Some(1) },
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("100"),
                Token::Struct {
                    name: "InteractionChannel",
                    len: 4,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("100"),
                Token::Str("type"),
                Token::U8(0),
                Token::Str("name"),
                Token::Str("channel name"),
                Token::Str("permissions"),
                Token::Str("0"),
                Token::StructEnd,
                Token::MapEnd,
                Token::Str("members"),
                Token::Map { len: Some(1) },
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("300"),
                Token::Struct {
                    name: "InteractionMemberEnvelope",
                    len: 2,
                },
                Token::Str("guild_id"),
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("200"),
                Token::Str("joined_at"),
                Token::Some,
                Token::Str("joined at"),
                Token::StructEnd,
                Token::MapEnd,
                Token::Str("roles"),
                Token::Map { len: Some(1) },
                Token::NewtypeStruct { name: "RoleId" },
                Token::Str("400"),
                Token::Struct {
                    name: "Role",
                    len: 8,
                },
                Token::Str("color"),
                Token::U32(0),
                Token::Str("hoist"),
                Token::Bool(true),
                Token::Str("id"),
                Token::NewtypeStruct { name: "RoleId" },
                Token::Str("400"),
                Token::Str("managed"),
                Token::Bool(false),
                Token::Str("mentionable"),
                Token::Bool(true),
                Token::Str("name"),
                Token::Str("test"),
                Token::Str("permissions"),
                Token::Str("8"),
                Token::Str("position"),
                Token::I64(12),
                Token::StructEnd,
                Token::MapEnd,
                Token::Str("users"),
                Token::Map { len: Some(1) },
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("300"),
                Token::Struct {
                    name: "User",
                    len: 12,
                },
                Token::Str("avatar"),
                Token::Some,
                Token::Str("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                Token::Str("bot"),
                Token::Bool(false),
                Token::Str("discriminator"),
                Token::Str("0001"),
                Token::Str("email"),
                Token::Some,
                Token::Str("address@example.com"),
                Token::Str("flags"),
                Token::Some,
                Token::U64(131_584),
                Token::Str("id"),
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("300"),
                Token::Str("locale"),
                Token::Some,
                Token::Str("en-us"),
                Token::Str("mfa_enabled"),
                Token::Some,
                Token::Bool(true),
                Token::Str("username"),
                Token::Str("test"),
                Token::Str("premium_type"),
                Token::Some,
                Token::U8(2),
                Token::Str("public_flags"),
                Token::Some,
                Token::U64(131_584),
                Token::Str("verified"),
                Token::Some,
                Token::Bool(true),
                Token::StructEnd,
                Token::MapEnd,
                Token::StructEnd,
            ],
        );
    }
}
