use crate::{
    guild::member::{Member, OptionalMemberDeserializer},
    id::{ChannelId, GuildId, UserId},
};
use serde::{
    de::{Deserializer, Error as DeError, IgnoredAny, MapAccess, Visitor},
    Deserialize, Serialize,
};
use std::fmt::{Formatter, Result as FmtResult};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct TypingStart {
    pub channel_id: ChannelId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<Member>,
    pub timestamp: u64,
    pub user_id: UserId,
}

#[derive(Debug, Deserialize)]
#[serde(field_identifier, rename_all = "snake_case")]
enum Field {
    ChannelId,
    GuildId,
    Member,
    Timestamp,
    UserId,
}

struct TypingStartVisitor;

impl<'de> Visitor<'de> for TypingStartVisitor {
    type Value = TypingStart;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("struct TypingStart")
    }

    fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
        let mut channel_id = None;
        let mut guild_id = None::<Option<_>>;
        let mut member = None::<Member>;
        let mut timestamp = None;
        let mut user_id = None;

        let span = tracing::trace_span!("deserializing typing start");
        let _span_enter = span.enter();

        loop {
            let span_child = tracing::trace_span!("iterating over element");
            let _span_child_enter = span_child.enter();

            let key = match map.next_key() {
                Ok(Some(key)) => {
                    tracing::trace!(?key, "found key");

                    key
                }
                Ok(None) => break,
                Err(why) => {
                    // Encountered when we run into an unknown key.
                    map.next_value::<IgnoredAny>()?;

                    tracing::trace!("ran into an unknown key: {:?}", why);

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

                    let deserializer = OptionalMemberDeserializer::new(GuildId(0));

                    member = map.next_value_seed(deserializer)?;
                }
                Field::Timestamp => {
                    if timestamp.is_some() {
                        return Err(DeError::duplicate_field("timestamp"));
                    }

                    timestamp = Some(map.next_value()?);
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
        let guild_id = guild_id.unwrap_or_default();
        let timestamp = timestamp.ok_or_else(|| DeError::missing_field("timestamp"))?;
        let user_id = user_id.ok_or_else(|| DeError::missing_field("user_id"))?;

        tracing::trace!(
            %channel_id,
            ?guild_id,
            %timestamp,
            %user_id,
        );

        if let (Some(guild_id), Some(member)) = (guild_id, member.as_mut()) {
            tracing::trace!(%guild_id, ?member, "setting member guild id");

            member.guild_id = guild_id;
        }

        Ok(TypingStart {
            channel_id,
            guild_id,
            member,
            timestamp,
            user_id,
        })
    }
}

impl<'de> Deserialize<'de> for TypingStart {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        const FIELDS: &[&str] = &["channel_id", "guild_id", "member", "timestamp", "user_id"];

        deserializer.deserialize_struct("TypingStart", FIELDS, TypingStartVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::super::TypingStart;
    use crate::{
        guild::Member,
        id::{ChannelId, GuildId, RoleId, UserId},
        user::User,
    };
    use serde_test::Token;

    #[allow(clippy::too_many_lines)]
    #[test]
    fn test_typing_start_with_member() {
        let value = TypingStart {
            channel_id: ChannelId(2),
            guild_id: Some(GuildId(1)),
            member: Some(Member {
                deaf: false,
                guild_id: GuildId(1),
                hoisted_role: Some(RoleId(4)),
                joined_at: Some("2020-01-01T00:00:00.000000+00:00".to_owned()),
                mute: false,
                nick: Some("typing".to_owned()),
                pending: false,
                premium_since: None,
                roles: vec![RoleId(4)],
                user: User {
                    id: UserId(3),
                    accent_color: None,
                    avatar: Some("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_owned()),
                    banner: None,
                    bot: false,
                    discriminator: 1,
                    name: "test".to_owned(),
                    mfa_enabled: None,
                    locale: None,
                    verified: None,
                    email: None,
                    flags: None,
                    premium_type: None,
                    system: None,
                    public_flags: None,
                },
            }),
            timestamp: 1_500_000_000,
            user_id: UserId(3),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "TypingStart",
                    len: 5,
                },
                Token::Str("channel_id"),
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("2"),
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
                Token::Str("4"),
                Token::Str("joined_at"),
                Token::Some,
                Token::Str("2020-01-01T00:00:00.000000+00:00"),
                Token::Str("mute"),
                Token::Bool(false),
                Token::Str("nick"),
                Token::Some,
                Token::Str("typing"),
                Token::Str("pending"),
                Token::Bool(false),
                Token::Str("roles"),
                Token::Seq { len: Some(1) },
                Token::NewtypeStruct { name: "RoleId" },
                Token::Str("4"),
                Token::SeqEnd,
                Token::Str("user"),
                Token::Struct {
                    name: "User",
                    len: 7,
                },
                Token::Str("accent_color"),
                Token::None,
                Token::Str("avatar"),
                Token::Some,
                Token::Str("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                Token::Str("banner"),
                Token::None,
                Token::Str("bot"),
                Token::Bool(false),
                Token::Str("discriminator"),
                Token::Str("0001"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("3"),
                Token::Str("username"),
                Token::Str("test"),
                Token::StructEnd,
                Token::StructEnd,
                Token::Str("timestamp"),
                Token::U64(1_500_000_000),
                Token::Str("user_id"),
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("3"),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_typing_start_without_member() {
        let value = TypingStart {
            channel_id: ChannelId(2),
            guild_id: None,
            member: None,
            timestamp: 1_500_000_000,
            user_id: UserId(3),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "TypingStart",
                    len: 3,
                },
                Token::Str("channel_id"),
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("2"),
                Token::Str("timestamp"),
                Token::U64(1_500_000_000),
                Token::Str("user_id"),
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("3"),
                Token::StructEnd,
            ],
        );
    }
}
