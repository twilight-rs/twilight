use crate::{
    channel::{permission_overwrite::PermissionOverwrite, ChannelType},
    id::{ChannelId, GuildId, MessageId},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct TextChannel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,
    pub id: ChannelId,
    #[serde(rename = "type")]
    pub kind: ChannelType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_message_id: Option<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_pin_timestamp: Option<String>,
    pub name: String,
    #[serde(default)]
    pub nsfw: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<ChannelId>,
    pub permission_overwrites: Vec<PermissionOverwrite>,
    pub position: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_per_user: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::{ChannelId, ChannelType, GuildId, MessageId, TextChannel};
    use serde_test::Token;

    #[test]
    fn test_text_channel() {
        let value = TextChannel {
            id: ChannelId::new(1).expect("non zero"),
            guild_id: Some(GuildId::new(2).expect("non zero")),
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
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("2"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "ChannelId" },
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
    fn test_text_channel_complete() {
        let value = TextChannel {
            id: ChannelId::new(1).expect("non zero"),
            guild_id: Some(GuildId::new(2).expect("non zero")),
            kind: ChannelType::GuildText,
            last_message_id: Some(MessageId::new(3).expect("non zero")),
            last_pin_timestamp: Some("123".to_owned()),
            name: "foo".to_owned(),
            nsfw: true,
            permission_overwrites: Vec::new(),
            parent_id: Some(ChannelId::new(4).expect("non zero")),
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
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("2"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("1"),
                Token::Str("type"),
                Token::U8(0),
                Token::Str("last_message_id"),
                Token::Some,
                Token::NewtypeStruct { name: "MessageId" },
                Token::Str("3"),
                Token::Str("last_pin_timestamp"),
                Token::Some,
                Token::Str("123"),
                Token::Str("name"),
                Token::Str("foo"),
                Token::Str("nsfw"),
                Token::Bool(true),
                Token::Str("parent_id"),
                Token::Some,
                Token::NewtypeStruct { name: "ChannelId" },
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
    }
}
