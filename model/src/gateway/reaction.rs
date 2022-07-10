use crate::{
    channel::message::ReactionType,
    guild::member::{Member, MemberIntermediary},
    id::{
        marker::{ChannelMarker, GuildMarker, MessageMarker, UserMarker},
        Id,
    },
};
use serde::{
    de::{Deserializer, Error as DeError, IgnoredAny, MapAccess, Visitor},
    Deserialize, Serialize,
};
use std::fmt::{Formatter, Result as FmtResult};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct Reaction {
    pub channel_id: Id<ChannelMarker>,
    pub emoji: ReactionType,
    pub guild_id: Option<Id<GuildMarker>>,
    pub member: Option<Member>,
    pub message_id: Id<MessageMarker>,
    pub user_id: Id<UserMarker>,
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
        let mut guild_id = None;
        let mut member: Option<MemberIntermediary> = None;
        let mut message_id = None;
        let mut user_id = None;

        let span = tracing::trace_span!("deserializing reaction");
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

                    guild_id = map.next_value()?;
                }
                Field::Member => {
                    if member.is_some() {
                        return Err(DeError::duplicate_field("member"));
                    }

                    member = map.next_value()?;
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

        tracing::trace!(?channel_id, ?emoji, ?message_id, ?user_id);

        let member = if let (Some(guild_id), Some(member)) = (guild_id, member) {
            tracing::trace!(%guild_id, ?member, "setting member guild id");

            Some(member.into_member(guild_id))
        } else {
            None
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
    use super::Reaction;
    use crate::{
        channel::message::ReactionType,
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
    fn reaction_with_member() -> Result<(), TimestampParseError> {
        let joined_at = Timestamp::from_str("2020-01-01T00:00:00.000000+00:00")?;

        let value = Reaction {
            channel_id: Id::new(2),
            emoji: ReactionType::Unicode {
                name: "a".to_owned(),
            },
            guild_id: Some(Id::new(1)),
            member: Some(Member {
                avatar: None,
                communication_disabled_until: None,
                deaf: false,
                guild_id: Id::new(1),
                joined_at,
                mute: false,
                nick: Some("typing".to_owned()),
                pending: false,
                premium_since: None,
                roles: vec![Id::new(5)],
                user: User {
                    accent_color: None,
                    avatar: Some(image_hash::AVATAR),
                    banner: None,
                    bot: false,
                    discriminator: 1,
                    email: None,
                    flags: None,
                    id: Id::new(4),
                    locale: None,
                    mfa_enabled: None,
                    name: "test".to_owned(),
                    premium_type: None,
                    public_flags: None,
                    system: None,
                    verified: None,
                },
            }),
            message_id: Id::new(3),
            user_id: Id::new(4),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Reaction",
                    len: 6,
                },
                Token::Str("channel_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("emoji"),
                Token::Struct {
                    name: "ReactionType",
                    len: 1,
                },
                Token::Str("name"),
                Token::Str("a"),
                Token::StructEnd,
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
                Token::Str("5"),
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
                Token::Str("4"),
                Token::Str("username"),
                Token::Str("test"),
                Token::StructEnd,
                Token::StructEnd,
                Token::Str("message_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("3"),
                Token::Str("user_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("4"),
                Token::StructEnd,
            ],
        );

        Ok(())
    }

    #[test]
    fn reaction_without_member() {
        let value = Reaction {
            channel_id: Id::new(2),
            emoji: ReactionType::Unicode {
                name: "a".to_owned(),
            },
            guild_id: None,
            member: None,
            message_id: Id::new(3),
            user_id: Id::new(4),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Reaction",
                    len: 6,
                },
                Token::Str("channel_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("emoji"),
                Token::Struct {
                    name: "ReactionType",
                    len: 1,
                },
                Token::Str("name"),
                Token::Str("a"),
                Token::StructEnd,
                Token::Str("guild_id"),
                Token::None,
                Token::Str("member"),
                Token::None,
                Token::Str("message_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("3"),
                Token::Str("user_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("4"),
                Token::StructEnd,
            ],
        );
    }
}
