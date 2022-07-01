use serde_repr::{Deserialize_repr, Serialize_repr};

// Keep in sync with `twilight-validate::command`!
#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, Serialize_repr)]
#[non_exhaustive]
#[repr(u8)]
pub enum CommandType {
    /// Slash command.
    ///
    /// Text-based command that appears when a user types `/`.
    ChatInput = 1,
    /// UI-based command.
    ///
    /// Appears when a user right clicks or taps om a user.
    User = 2,
    /// UI-based command.
    ///
    /// Appears when a user right clicks or taps on a message.
    Message = 3,
}

impl CommandType {
    pub const fn kind(self) -> &'static str {
        match self {
            Self::ChatInput => "ChatInput",
            Self::User => "User",
            Self::Message => "Message",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CommandType;
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, hash::Hash};

    assert_impl_all!(
        CommandType: Clone,
        Copy,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Serialize,
        Send,
        Sync
    );

    #[test]
    fn variants() {
        serde_test::assert_tokens(&CommandType::ChatInput, &[Token::U8(1)]);
        serde_test::assert_tokens(&CommandType::User, &[Token::U8(2)]);
        serde_test::assert_tokens(&CommandType::Message, &[Token::U8(3)]);
    }

    #[test]
    fn kinds() {
        assert_eq!("ChatInput", CommandType::ChatInput.kind());
        assert_eq!("User", CommandType::User.kind());
        assert_eq!("Message", CommandType::Message.kind());
    }
}
