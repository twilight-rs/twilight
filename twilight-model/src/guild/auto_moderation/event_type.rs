use serde::{Deserialize, Serialize};

/// Indicates in what event context a rule should be checked.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(from = "u8", into = "u8")]
pub enum AutoModerationEventType {
    /// When a member sends or edits a message in a guild.
    MessageSend,
    /// Variant value is unknown to the library.
    Unknown(u8),
}

impl From<u8> for AutoModerationEventType {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::MessageSend,
            _ => Self::Unknown(value),
        }
    }
}

impl From<AutoModerationEventType> for u8 {
    fn from(value: AutoModerationEventType) -> Self {
        match value {
            AutoModerationEventType::MessageSend => 1,
            AutoModerationEventType::Unknown(unknown) => unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AutoModerationEventType;
    use serde::{Deserialize, Serialize};
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, hash::Hash};

    assert_impl_all!(
        AutoModerationEventType: Clone,
        Copy,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync,
    );

    #[test]
    fn values() {
        assert_eq!(1, u8::from(AutoModerationEventType::MessageSend));
        assert_eq!(250, u8::from(AutoModerationEventType::Unknown(250)));
    }
}
