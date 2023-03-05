use crate::id::{
    marker::{ChannelMarker, GuildMarker, MessageMarker},
    Id,
};
use serde::{Deserialize, Serialize};

/// Message reference struct.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct MessageReference {
    /// Originating message's channel ID.
    ///
    /// Note: optional when creating a reply, but always present when receiving
    /// an event or response containing this model.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "rkyv", with(crate::id::IdNiche))]
    pub channel_id: Option<Id<ChannelMarker>>,
    /// Originating message's guild ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "rkyv", with(crate::id::IdNiche))]
    pub guild_id: Option<Id<GuildMarker>>,
    /// Originating message's ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "rkyv", with(crate::id::IdNiche))]
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
