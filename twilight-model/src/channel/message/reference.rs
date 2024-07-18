use crate::id::{
    marker::{ChannelMarker, GuildMarker, MessageMarker},
    Id,
};
use serde::{Deserialize, Serialize};

use super::reference_type::MessageReferenceType;

/// Message reference struct.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MessageReference {
    /// Originating message's channel ID.
    ///
    /// Note: optional when creating a reply, but always present when receiving
    /// an event or response containing this model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<Id<ChannelMarker>>,
    /// Originating message's guild ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    /// The type of reference.
    ///
    /// Defaults to [`MessageReferenceType::Default`].
    #[serde(default, rename = "type")]
    pub kind: MessageReferenceType,
    /// Originating message's ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<Id<MessageMarker>>,
    /// Whether to error if the referenced message doesn't exist instead of
    /// sending a normal message.
    ///
    /// Defaults to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_if_not_exists: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::MessageReference;
    use crate::{channel::message::reference_type::MessageReferenceType, id::Id};
    use serde_test::Token;

    #[test]
    fn minimal() {
        let value = MessageReference {
            kind: MessageReferenceType::Default,
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
                    len: 2,
                },
                Token::Str("channel_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("type"),
                Token::U8(0),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn complete() {
        let value = MessageReference {
            channel_id: Some(Id::new(1)),
            guild_id: Some(Id::new(2)),
            kind: MessageReferenceType::Default,
            message_id: Some(Id::new(3)),
            fail_if_not_exists: Some(false),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "MessageReference",
                    len: 5,
                },
                Token::Str("channel_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("type"),
                Token::U8(0),
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
