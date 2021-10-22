use crate::{
    channel::ChannelType,
    datetime::Timestamp,
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
    pub last_pin_timestamp: Option<Timestamp>,
    #[serde(rename = "type")]
    pub kind: ChannelType,
    pub recipients: Vec<User>,
}

#[cfg(test)]
mod tests {
    use super::{ChannelId, ChannelType, MessageId, PrivateChannel};
    use crate::datetime::{Timestamp, TimestampParseError};
    use serde_test::Token;
    use std::str::FromStr;

    #[test]
    fn test_category_channel() -> Result<(), TimestampParseError> {
        let last_pin_timestamp = Timestamp::from_str("2021-08-10T12:34:00+00:00")?;

        let value = PrivateChannel {
            id: ChannelId::new(1).expect("non zero"),
            last_message_id: Some(MessageId::new(2).expect("non zero")),
            last_pin_timestamp: Some(last_pin_timestamp),
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
                Token::Str("2021-08-10T12:34:00.000000+00:00"),
                Token::Str("type"),
                Token::U8(1),
                Token::Str("recipients"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::StructEnd,
            ],
        );

        Ok(())
    }
}
