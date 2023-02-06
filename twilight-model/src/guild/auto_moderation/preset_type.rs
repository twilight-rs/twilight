use serde::{Deserialize, Serialize};

/// Internally pre-defined wordsets which will be searched for in content.
#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AutoModerationKeywordPresetType(u8);

impl AutoModerationKeywordPresetType {
    /// Words that may be considered forms of swearing or cursing.
    pub const PROFANITY: Self = Self::new(1);

    /// Words that refer to sexually explicit behavior or activity.
    pub const SEXUAL_CONTENT: Self = Self::new(2);

    /// Personal insults or words that may be considered hate speech.
    pub const SLURS: Self = Self::new(3);

    /// Name of the associated constant.
    ///
    /// Returns `None` if the value doesn't have a defined constant.
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::PROFANITY => "PROFANITY",
            Self::SEXUAL_CONTENT => "SEXUAL_CONTENT",
            Self::SLURS => "SLURS",
            _ => return None,
        })
    }
}

impl_typed!(AutoModerationKeywordPresetType, u8);

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
        assert_eq!(1, u8::from(AutoModerationKeywordPresetType::PROFANITY));
        assert_eq!(2, u8::from(AutoModerationKeywordPresetType::SEXUAL_CONTENT));
        assert_eq!(3, u8::from(AutoModerationKeywordPresetType::SLURS));
        assert_eq!(250, u8::from(AutoModerationKeywordPresetType::new(250)));
    }
}
