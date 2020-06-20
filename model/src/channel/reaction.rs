use crate::{
    channel::ReactionType,
    guild::member::Member,
    id::{ChannelId, GuildId, MessageId, UserId},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Reaction {
    pub channel_id: ChannelId,
    pub emoji: ReactionType,
    pub guild_id: Option<GuildId>,
    pub member: Option<Member>,
    pub message_id: MessageId,
    pub user_id: UserId,
}

#[cfg(feature = "serde-support")]
mod if_serde_support {
    use super::Reaction;
    use crate::guild::member::if_serde_support::MemberDeserializer;
    use serde::{
        de::{DeserializeSeed, Deserializer, Error as DeError, MapAccess, Visitor},
        Deserialize,
    };
    use serde_value::Value;
    use std::fmt::{Formatter, Result as FmtResult};

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
            let guild_id = guild_id.unwrap_or_default();
            let member = member.unwrap_or_default();
            let message_id = message_id.ok_or_else(|| DeError::missing_field("message_id"))?;
            let user_id = user_id.ok_or_else(|| DeError::missing_field("user_id"))?;

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
        use serde_json::json;

        #[test]
        fn test_typing_start_with_member_deser() {
            let input = json!({
                "channel_id": "2",
                "emoji": {
                    "id": null,
                    "name": "🙂",
                },
                "guild_id": "1",
                "member": {
                    "deaf": false,
                    "hoisted_role": "5",
                    "joined_at": "2020-01-01T00:00:00.000000+00:00",
                    "mute": false,
                    "nick": "typing",
                    "roles": ["5"],
                    "user": {
                        "username": "test",
                        "id": "4",
                        "discriminator": "0001",
                        "avatar": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                    },
                },
                "message_id": "3",
                "user_id": "4",
            });

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
                        id: UserId(4),
                        avatar: Some("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_owned()),
                        bot: false,
                        discriminator: "0001".to_owned(),
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
                message_id: MessageId(3),
                user_id: UserId(4),
            };

            assert_eq!(expected, serde_json::from_value(input).unwrap());
        }

        #[test]
        fn test_typing_start_without_member_deser() {
            let input = json!({
                "channel_id": "2",
                "emoji": {
                    "id": null,
                    "name": "🙂",
                },
                "guild_id": null,
                "member": null,
                "message_id": "3",
                "user_id": "4",
            });

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

            assert_eq!(expected, serde_json::from_value(input).unwrap());
        }
    }
}
