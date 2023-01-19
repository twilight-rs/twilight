use crate::{
    guild::member::Member,
    id::{
        marker::{ChannelMarker, GuildMarker, UserMarker},
        Id,
    },
};
use serde::{
    de::{Deserializer, Error as DeError, IgnoredAny, MapAccess, Visitor},
    Deserialize, Serialize,
};
use std::fmt::{Formatter, Result as FmtResult};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct TypingStart {
    pub channel_id: Id<ChannelMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<Member>,
    pub timestamp: u64,
    pub user_id: Id<UserMarker>,
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
        let mut guild_id = None;
        let mut member: Option<Member> = None;
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

                    tracing::trace!("ran into an unknown key: {why:?}");

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

                    member = map.next_value()?;
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
    use super::TypingStart;
    use crate::{
        guild::Member,
        id::Id,
        test::image_hash,
        user::User,
        util::datetime::{Timestamp, TimestampParseError},
    };
    use serde_test::Token;
    use std::str::FromStr;

    #[allow(clippy::too_many_lines)]
    #[test]
    fn typing_start_with_member() -> Result<(), TimestampParseError> {
        let joined_at = Timestamp::from_str("2020-01-01T00:00:00.000000+00:00")?;

        let value = TypingStart {
            channel_id: Id::new(2),
            guild_id: Some(Id::new(1)),
            member: Some(Member {
                avatar: None,
                communication_disabled_until: None,
                deaf: false,
                joined_at,
                mute: false,
                nick: Some("typing".to_owned()),
                pending: false,
                premium_since: None,
                roles: vec![Id::new(4)],
                user: User {
                    id: Id::new(3),
                    accent_color: None,
                    avatar: Some(image_hash::AVATAR),
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
            user_id: Id::new(3),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "TypingStart",
                    len: 5,
                },
                Token::Str("channel_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("member"),
                Token::Some,
                Token::Struct {
                    name: "Member",
                    len: 9,
                },
                Token::Str("communication_disabled_until"),
                Token::None,
                Token::Str("deaf"),
                Token::Bool(false),
                Token::Str("guild_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("joined_at"),
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
                Token::NewtypeStruct { name: "Id" },
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
                Token::Str(image_hash::AVATAR_INPUT),
                Token::Str("banner"),
                Token::None,
                Token::Str("bot"),
                Token::Bool(false),
                Token::Str("discriminator"),
                Token::Str("0001"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("3"),
                Token::Str("username"),
                Token::Str("test"),
                Token::StructEnd,
                Token::StructEnd,
                Token::Str("timestamp"),
                Token::U64(1_500_000_000),
                Token::Str("user_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("3"),
                Token::StructEnd,
            ],
        );

        Ok(())
    }

    #[test]
    fn typing_start_without_member() {
        let value = TypingStart {
            channel_id: Id::new(2),
            guild_id: None,
            member: None,
            timestamp: 1_500_000_000,
            user_id: Id::new(3),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "TypingStart",
                    len: 3,
                },
                Token::Str("channel_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("timestamp"),
                Token::U64(1_500_000_000),
                Token::Str("user_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("3"),
                Token::StructEnd,
            ],
        );
    }
}
