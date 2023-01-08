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
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]

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

    /// Create a new message activity type from a dynamic value.
    ///
    /// The provided value isn't validated. Known valid values are associated
    /// constants such as [`SPECTATE`][`Self::SPECTATE`].
    pub const fn new(message_activity_type: u8) -> Self {
        Self(message_activity_type)
    }

    /// Retrieve the value of the message activity type.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_model::channel::message::MessageActivityType;
    ///
    /// assert_eq!(1, MessageActivityType::JOIN.get());
    /// ```
    pub const fn get(&self) -> u8 {
        self.0
    }
}

impl From<u8> for MessageActivityType {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<MessageActivityType> for u8 {
    fn from(value: MessageActivityType) -> Self {
        value.get()
    }
}

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
