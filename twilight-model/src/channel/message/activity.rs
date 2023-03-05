use serde::{Deserialize, Serialize};

/// Activity associated with a message.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct MessageActivity {
    /// [`MessageActivityType`]
    #[serde(rename = "type")]
    pub kind: MessageActivityType,
    /// ID of the player's party, lobby or group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party_id: Option<String>,
}

/// Activity of this message.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(from = "u8", into = "u8")]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub enum MessageActivityType {
    /// Join the the party.
    Join,
    /// Spectate on or with the party.
    Spectate,
    /// Listen to or with the party.
    Listen,
    /// Request to join the party.
    JoinRequest,
    /// Variant value is unknown to the library.
    Unknown(u8),
}

impl From<u8> for MessageActivityType {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Join,
            2 => Self::Spectate,
            3 => Self::Listen,
            5 => Self::JoinRequest,
            unknown => Self::Unknown(unknown),
        }
    }
}

impl From<MessageActivityType> for u8 {
    fn from(value: MessageActivityType) -> Self {
        match value {
            MessageActivityType::Join => 1,
            MessageActivityType::Spectate => 2,
            MessageActivityType::Listen => 3,
            MessageActivityType::JoinRequest => 5,
            MessageActivityType::Unknown(unknown) => unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{MessageActivity, MessageActivityType};
    use serde_test::Token;

    #[test]
    fn message_activity() {
        let value = MessageActivity {
            kind: MessageActivityType::Join,
            party_id: None,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "MessageActivity",
                    len: 1,
                },
                Token::Str("type"),
                Token::U8(1),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn message_activity_complete() {
        let value = MessageActivity {
            kind: MessageActivityType::Join,
            party_id: Some("test".to_owned()),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "MessageActivity",
                    len: 2,
                },
                Token::Str("type"),
                Token::U8(1),
                Token::Str("party_id"),
                Token::Some,
                Token::Str("test"),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn variants() {
        serde_test::assert_tokens(&MessageActivityType::Join, &[Token::U8(1)]);
        serde_test::assert_tokens(&MessageActivityType::Spectate, &[Token::U8(2)]);
        serde_test::assert_tokens(&MessageActivityType::Listen, &[Token::U8(3)]);
        serde_test::assert_tokens(&MessageActivityType::JoinRequest, &[Token::U8(5)]);
        serde_test::assert_tokens(&MessageActivityType::Unknown(99), &[Token::U8(99)]);
    }
}
