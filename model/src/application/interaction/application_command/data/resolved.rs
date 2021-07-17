use crate::{
    channel::ChannelType,
    guild::{Permissions, Role},
    id::{ChannelId, RoleId, UserId},
    user::User,
};
use serde::{
    de::{Deserializer, Error as DeError, MapAccess, Visitor},
    ser::{SerializeStruct, Serializer},
    Deserialize, Serialize,
};
use std::{
    collections::hash_map::{HashMap, RandomState},
    fmt::{Formatter, Result as FmtResult},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandInteractionDataResolved {
    pub channels: Vec<InteractionChannel>,
    pub members: Vec<InteractionMember>,
    pub roles: Vec<Role>,
    pub users: Vec<User>,
}

impl<'de> Deserialize<'de> for CommandInteractionDataResolved {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(ResolvedVisitor)
    }
}

impl Serialize for CommandInteractionDataResolved {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let len = vec![
            self.channels.is_empty(),
            self.members.is_empty(),
            self.roles.is_empty(),
            self.users.is_empty(),
        ]
        .into_iter()
        .filter(|b| !b)
        .count();

        let mut state = serializer.serialize_struct("CommandInteractionDataResolved", len)?;

        if !self.channels.is_empty() {
            let map: HashMap<ChannelId, &InteractionChannel, RandomState> = self
                .channels
                .iter()
                .map(|c| c.id)
                .zip(self.channels.iter())
                .collect();

            state.serialize_field("channels", &map)?;
        }

        if !self.members.is_empty() {
            let map: HashMap<UserId, &InteractionMember, RandomState> = self
                .members
                .iter()
                .map(|m| m.id)
                .zip(self.members.iter())
                .collect();

            state.serialize_field("members", &map)?;
        }

        if !self.roles.is_empty() {
            let map: HashMap<RoleId, &Role, RandomState> = self
                .roles
                .iter()
                .map(|r| r.id)
                .zip(self.roles.iter())
                .collect();

            state.serialize_field("roles", &map)?;
        }

        if !self.users.is_empty() {
            let map: HashMap<UserId, &User, RandomState> = self
                .users
                .iter()
                .map(|u| u.id)
                .zip(self.users.iter())
                .collect();

            state.serialize_field("users", &map)?;
        }

        state.end()
    }
}

#[derive(Debug, Deserialize)]
#[serde(field_identifier, rename_all = "snake_case")]
enum ResolvedField {
    Channels,
    Members,
    Roles,
    Users,
}

struct ResolvedVisitor;

impl<'de> Visitor<'de> for ResolvedVisitor {
    type Value = CommandInteractionDataResolved;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("struct CommandInteractionDataResolved")
    }

    fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
        let mut channels: Option<Vec<InteractionChannel>> = None;
        let mut members: Option<Vec<InteractionMember>> = None;
        let mut roles: Option<Vec<Role>> = None;
        let mut users: Option<Vec<User>> = None;

        loop {
            let key = match map.next_key() {
                Ok(Some(key)) => key,
                Ok(None) => break,
                Err(_) => continue,
            };

            match key {
                ResolvedField::Channels => {
                    if channels.is_some() {
                        return Err(DeError::duplicate_field("channels"));
                    }

                    let mapped_channels: HashMap<ChannelId, InteractionChannel> =
                        map.next_value()?;

                    channels = Some(mapped_channels.into_iter().map(|(_, v)| v).collect());
                }
                ResolvedField::Members => {
                    if members.is_some() {
                        return Err(DeError::duplicate_field("members"));
                    }

                    let mapped_members: HashMap<UserId, InteractionMemberEnvelope> =
                        map.next_value()?;

                    members = Some(
                        mapped_members
                            .into_iter()
                            .map(|(k, v)| InteractionMember {
                                hoisted_role: v.hoisted_role,
                                id: k,
                                joined_at: v.joined_at,
                                nick: v.nick,
                                premium_since: v.premium_since,
                                roles: v.roles,
                            })
                            .collect(),
                    );
                }
                ResolvedField::Roles => {
                    if roles.is_some() {
                        return Err(DeError::duplicate_field("roles"));
                    }

                    let map_roles: HashMap<RoleId, Role> = map.next_value()?;

                    roles = Some(map_roles.into_iter().map(|(_, v)| v).collect());
                }
                ResolvedField::Users => {
                    if users.is_some() {
                        return Err(DeError::duplicate_field("users"));
                    }

                    let map_users: HashMap<UserId, User> = map.next_value()?;

                    users = Some(map_users.into_iter().map(|(_, v)| v).collect());
                }
            }
        }

        Ok(CommandInteractionDataResolved {
            channels: channels.unwrap_or_default(),
            members: members.unwrap_or_default(),
            roles: roles.unwrap_or_default(),
            users: users.unwrap_or_default(),
        })
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

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename = "InteractionMemberEnvelope")]
pub struct InteractionMember {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hoisted_role: Option<RoleId>,
    #[serde(skip_serializing)]
    pub id: UserId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub joined_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_since: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<RoleId>,
}

#[derive(Deserialize)]
struct InteractionMemberEnvelope {
    pub hoisted_role: Option<RoleId>,
    pub joined_at: Option<String>,
    pub nick: Option<String>,
    pub premium_since: Option<String>,
    #[serde(default)]
    pub roles: Vec<RoleId>,
}

#[cfg(test)]
mod tests {
    use super::{CommandInteractionDataResolved, InteractionChannel, InteractionMember};
    use crate::{
        channel::ChannelType,
        guild::{Permissions, Role},
        id::{ChannelId, RoleId, UserId},
        user::{PremiumType, User, UserFlags},
    };
    use serde_test::Token;
    use std::num::NonZeroU64;

    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_data_resolved() {
        let value = CommandInteractionDataResolved {
            channels: vec![InteractionChannel {
                id: ChannelId(NonZeroU64::new(100).expect("non zero")),
                kind: ChannelType::GuildText,
                name: "channel name".into(),
                permissions: Permissions::empty(),
            }],
            members: vec![InteractionMember {
                hoisted_role: None,
                id: UserId(NonZeroU64::new(300).expect("non zero")),
                joined_at: Some("joined at".into()),
                nick: None,
                premium_since: None,
                roles: Vec::new(),
            }],
            roles: vec![Role {
                color: 0,
                hoist: true,
                id: RoleId(NonZeroU64::new(400).expect("non zero")),
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
                id: UserId(NonZeroU64::new(300).expect("non zero")),
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
                    name: "CommandInteractionDataResolved",
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
                    len: 1,
                },
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
