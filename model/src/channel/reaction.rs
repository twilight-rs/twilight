use crate::{
    channel::ReactionType,
    guild::member::{Member, MemberDeserializer},
    id::{ChannelId, GuildId, MessageId, UserId},
};
use serde::{
    de::{DeserializeSeed, Deserializer, Error as DeError, MapAccess, Visitor},
    Deserialize, Serialize,
};
use serde_value::Value;
use std::fmt::{Formatter, Result as FmtResult};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct Reaction {
    pub channel_id: ChannelId,
    pub emoji: ReactionType,
    pub guild_id: Option<GuildId>,
    pub member: Option<Member>,
    pub message_id: MessageId,
    pub user_id: UserId,
}

#[derive(Debug, Deserialize)]
#[serde(field_identifier, rename_all = "snake_case")]
enum Field {
    ChannelId,
    Emoji,
    GuildId,
    Member,
    MessageId,
    UserId,
}

struct ReactionVisitor;

impl<'de> Visitor<'de> for ReactionVisitor {
    type Value = Reaction;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("struct Reaction")
    }

    fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
        let mut channel_id = None;
        let mut emoji = None;
        let mut guild_id = None::<Option<_>>;
        let mut member = None::<Option<Value>>;
        let mut message_id = None;
        let mut user_id = None;

        loop {
            let key = match map.next_key() {
                Ok(Some(key)) => key,
                Ok(None) => break,
                Err(_) => {
                    // Encountered when we run into an unknown key.
                    continue;
                }
            };

            match key {
                Field::ChannelId => {
                    if channel_id.is_some() {
                        return Err(DeError::duplicate_field("channel_id"));
                    }

                    channel_id = Some(map.next_value()?);
                }
                Field::Emoji => {
                    if emoji.is_some() {
                        return Err(DeError::duplicate_field("emoji"));
                    }

                    emoji = Some(map.next_value()?);
                }
                Field::GuildId => {
                    if guild_id.is_some() {
                        return Err(DeError::duplicate_field("guild_id"));
                    }

                    guild_id = Some(map.next_value()?);
                }
                Field::Member => {
                    if member.is_some() {
                        return Err(DeError::duplicate_field("member"));
                    }

                    member = Some(map.next_value()?);
                }
                Field::MessageId => {
                    if message_id.is_some() {
                        return Err(DeError::duplicate_field("message_id"));
                    }

                    message_id = Some(map.next_value()?);
                }
                Field::UserId => {
                    if user_id.is_some() {
                        return Err(DeError::duplicate_field("user_id"));
                    }

                    user_id = Some(map.next_value()?);
                }
            }
        }

        let channel_id = channel_id.ok_or_else(|| DeError::missing_field("channel_id"))?;
        let emoji = emoji.ok_or_else(|| DeError::missing_field("emoji"))?;
        let message_id = message_id.ok_or_else(|| DeError::missing_field("message_id"))?;
        let user_id = user_id.ok_or_else(|| DeError::missing_field("user_id"))?;

        let guild_id = guild_id.unwrap_or_default();
        let member = member.unwrap_or_default();

        let member = match (member, guild_id) {
            (Some(value), Some(guild_id)) => {
                let deserializer = MemberDeserializer::new(guild_id);

                Some(deserializer.deserialize(value).map_err(DeError::custom)?)
            }
            _ => None,
        };

        Ok(Reaction {
            channel_id,
            emoji,
            guild_id,
            member,
            message_id,
            user_id,
        })
    }
}

impl<'de> Deserialize<'de> for Reaction {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        const FIELDS: &[&str] = &[
            "channel_id",
            "emoji",
            "guild_id",
            "member",
            "message_id",
            "user_id",
        ];

        deserializer.deserialize_struct("Reaction", FIELDS, ReactionVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Reaction, ReactionType};
    use crate::{
        guild::Member,
        id::{ChannelId, GuildId, MessageId, RoleId, UserId},
        user::User,
    };
    use serde_test::Token;

    #[test]
    fn test_reaction_with_member() {
        let expected = Reaction {
            channel_id: ChannelId(2),
            emoji: ReactionType::Unicode {
                name: "🙂".to_owned(),
            },
            guild_id: Some(GuildId(1)),
            member: Some(Member {
                deaf: false,
                guild_id: GuildId(1),
                hoisted_role: Some(RoleId(5)),
                joined_at: Some("2020-01-01T00:00:00.000000+00:00".to_owned()),
                mute: false,
                nick: Some("typing".to_owned()),
                premium_since: None,
                roles: vec![RoleId(5)],
                user: User {
                    avatar: Some("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_owned()),
                    bot: false,
                    discriminator: "0001".to_owned(),
                    email: None,
                    flags: None,
                    id: UserId(4),
                    locale: None,
                    mfa_enabled: None,
                    name: "test".to_owned(),
                    premium_type: None,
                    public_flags: None,
                    system: None,
                    verified: None,
                },
            }),
            message_id: MessageId(3),
            user_id: UserId(4),
        };

        serde_test::assert_tokens(
            &expected,
            &[
                Token::Struct {
                    name: "Reaction",
                    len: 6,
                },
                Token::Str("channel_id"),
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("2"),
                Token::Str("emoji"),
                Token::Struct {
                    name: "ReactionType",
                    len: 1,
                },
                Token::Str("name"),
                Token::Str("🙂"),
                Token::StructEnd,
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("1"),
                Token::Str("member"),
                Token::Some,
                Token::Struct {
                    name: "Member",
                    len: 9,
                },
                Token::Str("deaf"),
                Token::Bool(false),
                Token::Str("guild_id"),
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("1"),
                Token::Str("hoisted_role"),
                Token::Some,
                Token::NewtypeStruct { name: "RoleId" },
                Token::Str("5"),
                Token::Str("joined_at"),
                Token::Some,
                Token::Str("2020-01-01T00:00:00.000000+00:00"),
                Token::Str("mute"),
                Token::Bool(false),
                Token::Str("nick"),
                Token::Some,
                Token::Str("typing"),
                Token::Str("premium_since"),
                Token::None,
                Token::Str("roles"),
                Token::Seq { len: Some(1) },
                Token::NewtypeStruct { name: "RoleId" },
                Token::Str("5"),
                Token::SeqEnd,
                Token::Str("user"),
                Token::Struct {
                    name: "User",
                    len: 13,
                },
                Token::Str("avatar"),
                Token::Some,
                Token::Str("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                Token::Str("bot"),
                Token::Bool(false),
                Token::Str("discriminator"),
                Token::Str("0001"),
                Token::Str("email"),
                Token::None,
                Token::Str("flags"),
                Token::None,
                Token::Str("id"),
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("4"),
                Token::Str("locale"),
                Token::None,
                Token::Str("mfa_enabled"),
                Token::None,
                Token::Str("username"),
                Token::Str("test"),
                Token::Str("premium_type"),
                Token::None,
                Token::Str("public_flags"),
                Token::None,
                Token::Str("system"),
                Token::None,
                Token::Str("verified"),
                Token::None,
                Token::StructEnd,
                Token::StructEnd,
                Token::Str("message_id"),
                Token::NewtypeStruct { name: "MessageId" },
                Token::Str("3"),
                Token::Str("user_id"),
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("4"),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_reaction_without_member() {
        let expected = Reaction {
            channel_id: ChannelId(2),
            emoji: ReactionType::Unicode {
                name: "🙂".to_owned(),
            },
            guild_id: None,
            member: None,
            message_id: MessageId(3),
            user_id: UserId(4),
        };

        serde_test::assert_tokens(
            &expected,
            &[
                Token::Struct {
                    name: "Reaction",
                    len: 6,
                },
                Token::Str("channel_id"),
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("2"),
                Token::Str("emoji"),
                Token::Struct {
                    name: "ReactionType",
                    len: 1,
                },
                Token::Str("name"),
                Token::Str("🙂"),
                Token::StructEnd,
                Token::Str("guild_id"),
                Token::None,
                Token::Str("member"),
                Token::None,
                Token::Str("message_id"),
                Token::NewtypeStruct { name: "MessageId" },
                Token::Str("3"),
                Token::Str("user_id"),
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("4"),
                Token::StructEnd,
            ],
        );
    }
}
