use serde::{Deserialize, Serialize};

/// Characterizes the type of content which can trigger the rule.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(from = "u8", into = "u8")]
pub enum AutoModerationTriggerType {
    /// Check if content contains words from a user defined list of keywords.
    ///
    /// Maximum of 3 per guild.
    Keyword,
    /// Check if content contains any harmful links.
    ///
    /// Currently unreleased. Maximum of 1 per guild.
    HarmfulLink,
    /// Check if content represents generic spam.
    ///
    /// Currently unreleased. Maximum of 1 per guild.
    Spam,
    /// Check if content contains words from internal pre-defined wordsets.
    ///
    /// Maximum of 1 per guild.
    KeywordPreset,
    /// Variant value is unknown to the library.
    Unknown(u8),
}

impl From<u8> for AutoModerationTriggerType {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Keyword,
            2 => Self::HarmfulLink,
            3 => Self::Spam,
            4 => Self::KeywordPreset,
            _ => Self::Unknown(value),
        }
    }
}

impl From<AutoModerationTriggerType> for u8 {
    fn from(value: AutoModerationTriggerType) -> Self {
        match value {
            AutoModerationTriggerType::Keyword => 1,
            AutoModerationTriggerType::HarmfulLink => 2,
            AutoModerationTriggerType::Spam => 3,
            AutoModerationTriggerType::KeywordPreset => 4,
            AutoModerationTriggerType::Unknown(unknown) => unknown,
        }
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
        assert_eq!(1, u8::from(AutoModerationTriggerType::Keyword));
        assert_eq!(2, u8::from(AutoModerationTriggerType::HarmfulLink));
        assert_eq!(3, u8::from(AutoModerationTriggerType::Spam));
        assert_eq!(4, u8::from(AutoModerationTriggerType::KeywordPreset));
        assert_eq!(250, u8::from(AutoModerationTriggerType::Unknown(250)));
    }
}
