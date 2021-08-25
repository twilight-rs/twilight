use crate::{
    channel::ChannelType,
    id::{ChannelId, MessageId},
    user::User,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PrivateChannel {
    pub id: ChannelId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_message_id: Option<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_pin_timestamp: Option<String>,
    #[serde(rename = "type")]
    pub kind: ChannelType,
    pub recipients: Vec<User>,
}

#[cfg(test)]
mod tests {
    use super::{ChannelId, ChannelType, MessageId, PrivateChannel};
    use serde_test::Token;

    #[test]
    fn test_category_channel() {
        let value = PrivateChannel {
            id: ChannelId::new(1).expect("non zero"),
            last_message_id: Some(MessageId::new(2).expect("non zero")),
            last_pin_timestamp: Some("timestamp".to_owned()),
            kind: ChannelType::Private,
            recipients: Vec::new(),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "PrivateChannel",
                    len: 5,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("1"),
                Token::Str("last_message_id"),
                Token::Some,
                Token::NewtypeStruct { name: "MessageId" },
                Token::Str("2"),
                Token::Str("last_pin_timestamp"),
                Token::Some,
                Token::Str("timestamp"),
                Token::Str("type"),
                Token::U8(1),
                Token::Str("recipients"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::StructEnd,
            ],
        );
    }
}
