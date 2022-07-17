use serde::{Deserialize, Serialize};

/// Internally pre-defined wordsets which will be searched for in content.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(from = "u8", into = "u8")]
pub enum AutoModerationKeywordPresetType {
    /// Words that may be considered forms of swearing or cursing.
    Profanity,
    /// Words that refer to sexually explicit behavior or activity.
    SexualContent,
    /// Personal insults or words that may be considered hate speech.
    Slurs,
    /// Variant value is unknown to the library.
    Unknown(u8),
}

impl From<u8> for AutoModerationKeywordPresetType {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Profanity,
            2 => Self::SexualContent,
            3 => Self::Slurs,
            _ => Self::Unknown(value),
        }
    }
}

impl From<AutoModerationKeywordPresetType> for u8 {
    fn from(value: AutoModerationKeywordPresetType) -> Self {
        match value {
            AutoModerationKeywordPresetType::Profanity => 1,
            AutoModerationKeywordPresetType::SexualContent => 2,
            AutoModerationKeywordPresetType::Slurs => 3,
            AutoModerationKeywordPresetType::Unknown(unknown) => unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AutoModerationKeywordPresetType;
    use serde::{Deserialize, Serialize};
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, hash::Hash};

    assert_impl_all!(
        AutoModerationKeywordPresetType: Clone,
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
        assert_eq!(1, u8::from(AutoModerationKeywordPresetType::Profanity));
        assert_eq!(2, u8::from(AutoModerationKeywordPresetType::SexualContent));
        assert_eq!(3, u8::from(AutoModerationKeywordPresetType::Slurs));
        assert_eq!(250, u8::from(AutoModerationKeywordPresetType::Unknown(250)));
    }
}
