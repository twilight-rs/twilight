use crate::{
    channel::{permission_overwrite::PermissionOverwrite, ChannelType},
    datetime::Timestamp,
    id::{
        marker::{ChannelMarker, GuildMarker, MessageMarker},
        Id,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct TextChannel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    pub id: Id<ChannelMarker>,
    #[serde(rename = "type")]
    pub kind: ChannelType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_message_id: Option<Id<MessageMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_pin_timestamp: Option<Timestamp>,
    pub name: String,
    #[serde(default)]
    pub nsfw: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<Id<ChannelMarker>>,
    pub permission_overwrites: Vec<PermissionOverwrite>,
    pub position: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_per_user: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::{ChannelType, TextChannel};
    use crate::{
        datetime::{Timestamp, TimestampParseError},
        id::Id,
    };
    use serde_test::Token;
    use std::str::FromStr;

    #[test]
    fn test_text_channel() {
        let value = TextChannel {
            id: Id::new(1),
            guild_id: Some(Id::new(2)),
            kind: ChannelType::GuildText,
            last_message_id: None,
            last_pin_timestamp: None,
            name: "foo".to_owned(),
            nsfw: true,
            permission_overwrites: Vec::new(),
            parent_id: None,
            position: 3,
            rate_limit_per_user: Some(10),
            topic: Some("a topic".to_owned()),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "TextChannel",
                    len: 9,
                },
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("type"),
                Token::U8(0),
                Token::Str("name"),
                Token::Str("foo"),
                Token::Str("nsfw"),
                Token::Bool(true),
                Token::Str("permission_overwrites"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("position"),
                Token::I64(3),
                Token::Str("rate_limit_per_user"),
                Token::Some,
                Token::U64(10),
                Token::Str("topic"),
                Token::Some,
                Token::Str("a topic"),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_text_channel_complete() -> Result<(), TimestampParseError> {
        let last_pin_timestamp = Timestamp::from_str("2021-08-10T12:34:00+00:00")?;

        let value = TextChannel {
            id: Id::new(1),
            guild_id: Some(Id::new(2)),
            kind: ChannelType::GuildText,
            last_message_id: Some(Id::new(3)),
            last_pin_timestamp: Some(last_pin_timestamp),
            name: "foo".to_owned(),
            nsfw: true,
            permission_overwrites: Vec::new(),
            parent_id: Some(Id::new(4)),
            position: 3,
            rate_limit_per_user: Some(10),
            topic: Some("a topic".to_owned()),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "TextChannel",
                    len: 12,
                },
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("type"),
                Token::U8(0),
                Token::Str("last_message_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("3"),
                Token::Str("last_pin_timestamp"),
                Token::Some,
                Token::Str("2021-08-10T12:34:00.000000+00:00"),
                Token::Str("name"),
                Token::Str("foo"),
                Token::Str("nsfw"),
                Token::Bool(true),
                Token::Str("parent_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("4"),
                Token::Str("permission_overwrites"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("position"),
                Token::I64(3),
                Token::Str("rate_limit_per_user"),
                Token::Some,
                Token::U64(10),
                Token::Str("topic"),
                Token::Some,
                Token::Str("a topic"),
                Token::StructEnd,
            ],
        );

        Ok(())
    }
}
