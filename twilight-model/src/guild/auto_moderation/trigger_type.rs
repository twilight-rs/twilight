use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter, Result as FmtResult};

/// Characterizes the type of content which can trigger the rule.
#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AutoModerationTriggerType(u8);

impl AutoModerationTriggerType {
    /// Check if content contains words from a user defined list of keywords.
    ///
    /// Maximum of 5 per guild.
    pub const KEYWORD: Self = Self::new(1);

    /// Check if content represents generic spam.
    ///
    /// Currently unreleased. Maximum of 1 per guild.
    pub const SPAM: Self = Self::new(3);

    /// Check if content contains words from internal pre-defined wordsets.
    ///
    /// Maximum of 1 per guild.
    pub const KEYWORD_PRESET: Self = Self::new(4);

    /// Check if content contains more unique mentions than allowed.
    pub const MENTION_SPAM: Self = Self::new(5);

    /// Create a new auto moderation trigger type from a dynamic value.
    ///
    /// The provided value isn't validated. Known valid values are associated
    /// constants such as [`SPAM`][`Self::SPAM`].
    pub const fn new(auto_moderation_trigger_type: u8) -> Self {
        Self(auto_moderation_trigger_type)
    }

    /// Retrieve the value of the auto moderation trigger type.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_model::guild::auto_moderation::AutoModerationTriggerType;
    ///
    /// assert_eq!(5, AutoModerationTriggerType::MENTION_SPAM.get());
    /// ```
    pub const fn get(&self) -> u8 {
        self.0
    }

    /// Name of the associated constant.
    ///
    /// Returns `None` if the value doesn't have a defined constant.
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::KEYWORD => "KEYWORD",
            Self::KEYWORD_PRESET => "KEYWORD_PRESET",
            Self::MENTION_SPAM => "MENTION_SPAM",
            Self::SPAM => "SPAM",
            _ => return None,
        })
    }
}

impl Debug for AutoModerationTriggerType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(name) = self.name() {
            f.debug_struct("AutoModerationTriggerType")
                .field("name", &name)
                .field("value", &self.0)
                .finish()
        } else {
            f.debug_tuple("AutoModerationTriggerType")
                .field(&self.0)
                .finish()
        }
    }
}

impl From<u8> for AutoModerationTriggerType {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<AutoModerationTriggerType> for u8 {
    fn from(value: AutoModerationTriggerType) -> Self {
        value.get()
    }
}

#[cfg(test)]
mod tests {
    use super::AutoModerationTriggerType;
    use serde::{Deserialize, Serialize};
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, hash::Hash};

    assert_impl_all!(
        AutoModerationTriggerType: Clone,
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
        assert_eq!(1, u8::from(AutoModerationTriggerType::KEYWORD));
        assert_eq!(3, u8::from(AutoModerationTriggerType::SPAM));
        assert_eq!(4, u8::from(AutoModerationTriggerType::KEYWORD_PRESET));
        assert_eq!(5, u8::from(AutoModerationTriggerType::MENTION_SPAM));
        assert_eq!(250, u8::from(AutoModerationTriggerType::new(250)));
    }
}
