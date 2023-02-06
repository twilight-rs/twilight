use serde::{Deserialize, Serialize};

/// Indicates in what event context a rule should be checked.
#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AutoModerationEventType(u8);

impl AutoModerationEventType {
    /// When a member sends or edits a message in a guild.
    pub const MESSAGE_SEND: Self = Self::new(1);

    /// Name of the associated constant.
    ///
    /// Returns `None` if the value doesn't have a defined constant.
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::MESSAGE_SEND => "MESSAGE_SEND",
            _ => return None,
        })
    }
}

impl_typed!(AutoModerationEventType, u8);

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
        assert_eq!(1, u8::from(AutoModerationEventType::MESSAGE_SEND));
        assert_eq!(250, u8::from(AutoModerationEventType::new(250)));
    }
}
