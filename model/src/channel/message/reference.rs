use crate::id::{ChannelId, GuildId, MessageId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MessageReference {
    pub channel_id: Option<ChannelId>,
    pub guild_id: Option<GuildId>,
    pub message_id: Option<MessageId>,
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
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "MessageReference",
                    len: 3,
                },
                Token::Str("channel_id"),
                Token::Some,
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("1"),
                Token::Str("guild_id"),
                Token::None,
                Token::Str("message_id"),
                Token::None,
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
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "MessageReference",
                    len: 3,
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
                Token::StructEnd,
            ],
        );
    }
}
