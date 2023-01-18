use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter, Result as FmtResult};

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

    /// Create a new auto moderation keyword preset type from a dynamic value.
    ///
    /// The provided value isn't validated. Known valid values are associated
    /// constants such as [`PROFANITY`][`Self::PROFANITY`].
    pub const fn new(auto_moderation_keyword_preset_type: u8) -> Self {
        Self(auto_moderation_keyword_preset_type)
    }

    /// Retrieve the value of the auto moderation keyword preset type.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_model::guild::auto_moderation::AutoModerationKeywordPresetType;
    ///
    /// assert_eq!(2, AutoModerationKeywordPresetType::SEXUAL_CONTENT.get());
    /// ```
    pub const fn get(&self) -> u8 {
        self.0
    }

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

impl Debug for AutoModerationKeywordPresetType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(name) = self.name() {
            f.debug_struct("AutoModerationKeywordPresetType")
                .field("name", &name)
                .field("value", &self.0)
                .finish()
        } else {
            f.debug_tuple("AutoModerationKeywordPresetType")
                .field(&self.0)
                .finish()
        }
    }
}

impl From<u8> for AutoModerationKeywordPresetType {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<AutoModerationKeywordPresetType> for u8 {
    fn from(value: AutoModerationKeywordPresetType) -> Self {
        value.get()
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
        assert_eq!(1, u8::from(AutoModerationKeywordPresetType::PROFANITY));
        assert_eq!(2, u8::from(AutoModerationKeywordPresetType::SEXUAL_CONTENT));
        assert_eq!(3, u8::from(AutoModerationKeywordPresetType::SLURS));
        assert_eq!(250, u8::from(AutoModerationKeywordPresetType::new(250)));
    }
}
