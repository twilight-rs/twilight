use crate::id::{
    marker::{ChannelMarker, GuildMarker, MessageMarker},
    Id,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MessageReference {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<Id<ChannelMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<Id<MessageMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_if_not_exists: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::MessageReference;
    use crate::id::Id;
    use serde_test::Token;

    #[test]
    fn minimal() {
        let value = MessageReference {
            channel_id: Some(Id::new(1)),
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
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn complete() {
        let value = MessageReference {
            channel_id: Some(Id::new(1)),
            guild_id: Some(Id::new(2)),
            message_id: Some(Id::new(3)),
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
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("message_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("3"),
                Token::Str("fail_if_not_exists"),
                Token::Some,
                Token::Bool(false),
                Token::StructEnd,
            ],
        );
    }
}
