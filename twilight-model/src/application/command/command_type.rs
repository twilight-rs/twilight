use serde::{Deserialize, Serialize};

// Keep in sync with `twilight-validate::command`!
#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct CommandType(u8);

impl CommandType {
    /// Slash command.
    ///
    /// Text-based command that appears when a user types `/`.
    pub const CHAT_INPUT: Self = Self::new(1);

    /// UI-based command.
    ///
    /// Appears when a user right clicks or taps on a user.
    pub const USER: Self = Self::new(2);

    /// UI-based command.
    ///
    /// Appears when a user right clicks or taps on a message.
    pub const MESSAGE: Self = Self::new(3);

    /// Name of the associated constant.
    ///
    /// Returns `None` if the value doesn't have a defined constant.
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::CHAT_INPUT => "CHAT_INPUT",
            Self::MESSAGE => "MESSAGE",
            Self::USER => "USER",
            _ => return None,
        })
    }
}

impl_typed!(CommandType, u8);

#[cfg(test)]
mod tests {
    use super::CommandType;
    use serde_test::Token;

    const MAP: &[(CommandType, u8)] = &[
        (CommandType::CHAT_INPUT, 1),
        (CommandType::USER, 2),
        (CommandType::MESSAGE, 3),
    ];

    #[test]
    fn variants() {
        for (kind, num) in MAP {
            serde_test::assert_tokens(
                kind,
                &[
                    Token::NewtypeStruct {
                        name: "CommandType",
                    },
                    Token::U8(*num),
                ],
            );
            assert_eq!(*kind, CommandType::from(*num));
            assert_eq!(*num, kind.get());
        }
    }
}
