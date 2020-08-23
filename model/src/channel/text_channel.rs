use crate::{
    channel::{permission_overwrite::PermissionOverwrite, ChannelType},
    id::{ChannelId, GuildId, MessageId},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct TextChannel {
    pub guild_id: Option<GuildId>,
    pub id: ChannelId,
    #[serde(rename = "type")]
    pub kind: ChannelType,
    pub last_message_id: Option<MessageId>,
    pub last_pin_timestamp: Option<String>,
    pub name: String,
    #[serde(default)]
    pub nsfw: bool,
    pub permission_overwrites: Vec<PermissionOverwrite>,
    pub parent_id: Option<ChannelId>,
    pub position: i64,
    pub rate_limit_per_user: Option<u64>,
    pub topic: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::{ChannelId, ChannelType, GuildId, TextChannel};
    use serde_test::Token;

    #[test]
    fn test_text_channel() {
        let value = TextChannel {
            id: ChannelId(1),
            guild_id: Some(GuildId(2)),
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
                Token::None,
                Token::Str("last_pin_timestamp"),
                Token::None,
                Token::Str("name"),
                Token::Str("foo"),
                Token::Str("nsfw"),
                Token::Bool(true),
                Token::Str("permission_overwrites"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("parent_id"),
                Token::None,
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
