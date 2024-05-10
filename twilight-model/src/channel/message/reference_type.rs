use serde::{Deserialize, Serialize};

/// The type of reference for a message.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(from = "u8", into = "u8")]
pub enum MessageReferenceType {
    /// A standard reference used by replies.
    Default,
    /// Reference used to point to a message at a point in time.
    Forward,
    /// An unknown message reference type.
    Unknown(u8),
}

impl From<u8> for MessageReferenceType {
    fn from(value: u8) -> Self {
        match value {
            0 => MessageReferenceType::Default,
            1 => MessageReferenceType::Forward,
            other => MessageReferenceType::Unknown(other),
        }
    }
}

impl From<MessageReferenceType> for u8 {
    fn from(value: MessageReferenceType) -> Self {
        match value {
            MessageReferenceType::Default => 0,
            MessageReferenceType::Forward => 1,
            MessageReferenceType::Unknown(other) => other,
        }
    }
}

impl MessageReferenceType {
    /// Return a string representation of the type.
    pub const fn name(&self) -> &str {
        match self {
            Self::Default => "Default",
            Self::Forward => "Forward",
            Self::Unknown(_) => "Unknown",
        }
    }
}

// The default value is `Default` because the only reason this value would
// not be present is if a message referenced before the API deployed was created.
// The docs state it's fine to assume field to be `Default` if it's not present.
impl Default for MessageReferenceType {
    fn default() -> Self {
        Self::Default
    }
}

#[cfg(test)]
mod tests {
    use super::MessageReferenceType;
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, hash::Hash};

    assert_impl_all!(
        MessageReferenceType: Clone,
        Copy,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Serialize,
        Send,
        Sync,
    );

    #[test]
    fn variants() {
        serde_test::assert_tokens(&MessageReferenceType::Default, &[Token::U8(0)]);
        serde_test::assert_tokens(&MessageReferenceType::Forward, &[Token::U8(1)]);
        serde_test::assert_tokens(&MessageReferenceType::Unknown(99), &[Token::U8(99)]);
    }

    #[test]
    fn names() {
        assert_eq!(MessageReferenceType::Default.name(), "Default");
        assert_eq!(MessageReferenceType::Forward.name(), "Forward");
        assert_eq!(MessageReferenceType::Unknown(99).name(), "Unknown");
    }

    #[test]
    fn default() {
        assert_eq!(
            MessageReferenceType::Default,
            MessageReferenceType::default()
        );
    }
}
