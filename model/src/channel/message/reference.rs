use crate::id::{ChannelId, GuildId, MessageId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MessageReference {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<ChannelId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_if_not_exists: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::{ChannelId, GuildId, MessageId, MessageReference};
    use serde_test::Token;

    #[test]
    fn test_minimal() {
        let value = MessageReference {
            channel_id: Some(ChannelId(1)),
            guild_id: None,
            message_id: None,
            fail_if_not_exists: None,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "MessageReference",
                    len: 1,
                },
                Token::Str("channel_id"),
                Token::Some,
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("1"),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_complete() {
        let value = MessageReference {
            channel_id: Some(ChannelId(1)),
            guild_id: Some(GuildId(2)),
            message_id: Some(MessageId(3)),
            fail_if_not_exists: Some(false),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "MessageReference",
                    len: 4,
                },
                Token::Str("channel_id"),
                Token::Some,
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("1"),
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("2"),
                Token::Str("message_id"),
                Token::Some,
                Token::NewtypeStruct { name: "MessageId" },
                Token::Str("3"),
                Token::Str("fail_if_not_exists"),
                Token::Some,
                Token::Bool(false),
                Token::StructEnd,
            ],
        );
    }
}
