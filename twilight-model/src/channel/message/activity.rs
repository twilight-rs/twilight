use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Activity associated with a message.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MessageActivity {
    /// [`MessageActivityType`]
    #[serde(rename = "type")]
    pub kind: MessageActivityType,
    /// ID of the player's party, lobby or group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party_id: Option<String>,
}

/// Activity of this message.
#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, Serialize_repr)]
#[repr(u8)]
pub enum MessageActivityType {
    /// Join the the party.
    Join = 1,
    /// Spectate on or with the party.
    Spectate = 2,
    /// Listen to or with the party.
    Listen = 3,
    /// Request to join the party.
    JoinRequest = 5,
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
    }
}
