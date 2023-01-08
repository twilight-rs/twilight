use serde::{Deserialize, Serialize};

/// Indicates in what event context a rule should be checked.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AutoModerationEventType(u8);

impl AutoModerationEventType {
    /// When a member sends or edits a message in a guild.
    pub const MESSAGE_SEND: Self = Self::new(1);

    /// Create a new auto moderation event type from a dynamic value.
    ///
    /// The provided value isn't validated. Known valid values are associated
    /// constants such as [`MESSAGE_SEND`][`Self::MESSAGE_SEND`].
    pub const fn new(auto_moderation_event_type: u8) -> Self {
        Self(auto_moderation_event_type)
    }

    /// Retrieve the value of the auto moderation event type.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_model::guild::auto_moderation::AutoModerationEventType;
    ///
    /// assert_eq!(1, AutoModerationEventType::MESSAGE_SEND.get());
    /// ```
    pub const fn get(&self) -> u8 {
        self.0
    }
}

impl From<u8> for AutoModerationEventType {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<AutoModerationEventType> for u8 {
    fn from(value: AutoModerationEventType) -> Self {
        value.get()
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
        assert_eq!(1, u8::from(AutoModerationEventType::MESSAGE_SEND));
        assert_eq!(250, u8::from(AutoModerationEventType::new(250)));
    }
}
