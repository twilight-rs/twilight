use serde::{Deserialize, Serialize};

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
#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]

pub struct MessageActivityType(u8);

impl MessageActivityType {
    /// Join the the party.
    pub const JOIN: Self = Self::new(1);

    /// Spectate on or with the party.
    pub const SPECTATE: Self = Self::new(2);

    /// Listen to or with the party.
    pub const LISTEN: Self = Self::new(3);

    /// Request to join the party.
    pub const JOIN_REQUEST: Self = Self::new(5);

    /// Name of the associated constant.
    ///
    /// Returns `None` if the value doesn't have a defined constant.
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::JOIN => "JOIN",
            Self::SPECTATE => "SPECTATE",
            Self::LISTEN => "LISTEN",
            Self::JOIN_REQUEST => "JOIN_REQUEST",
            _ => return None,
        })
    }
}

impl_typed!(MessageActivityType, u8);

#[cfg(test)]
mod tests {
    use super::{MessageActivity, MessageActivityType};
    use serde_test::Token;

    #[test]
    fn message_activity() {
        let value = MessageActivity {
            kind: MessageActivityType::JOIN,
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
                Token::NewtypeStruct {
                    name: "MessageActivityType",
                },
                Token::U8(1),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn message_activity_complete() {
        let value = MessageActivity {
            kind: MessageActivityType::JOIN,
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
                Token::NewtypeStruct {
                    name: "MessageActivityType",
                },
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
        serde_test::assert_tokens(
            &MessageActivityType::JOIN,
            &[
                Token::NewtypeStruct {
                    name: "MessageActivityType",
                },
                Token::U8(1),
            ],
        );
        serde_test::assert_tokens(
            &MessageActivityType::SPECTATE,
            &[
                Token::NewtypeStruct {
                    name: "MessageActivityType",
                },
                Token::U8(2),
            ],
        );
        serde_test::assert_tokens(
            &MessageActivityType::LISTEN,
            &[
                Token::NewtypeStruct {
                    name: "MessageActivityType",
                },
                Token::U8(3),
            ],
        );
        serde_test::assert_tokens(
            &MessageActivityType::JOIN_REQUEST,
            &[
                Token::NewtypeStruct {
                    name: "MessageActivityType",
                },
                Token::U8(5),
            ],
        );
        serde_test::assert_tokens(
            &MessageActivityType::new(99),
            &[
                Token::NewtypeStruct {
                    name: "MessageActivityType",
                },
                Token::U8(99),
            ],
        );
    }
}
