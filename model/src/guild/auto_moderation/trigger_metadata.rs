use super::AutoModerationKeywordPresetType;
use serde::{Deserialize, Serialize};

/// Additional data used to determine whether a rule should be triggered.
///
/// Different fields are relevant based on the value of [`AutoModerationRule::trigger_type`].
///
/// [`AutoModerationRule::trigger_type`]: super::AutoModerationRule::trigger_type
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AutoModerationTriggerMetadata {
    /// Substrings which will be searched for in content.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub keyword_filter: Vec<String>,
    /// Internally pre-defined wordsets which will be searched for in content.
    ///
    /// A keyword can be a phrase which contains multiple words. Wildcard
    /// symbols can be used to customize how each keyword will be matched. See
    /// [Discord Docs/Keyword Matching Strategies].
    ///
    /// [Discord Docs/Keyword Matching Strategies]: https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-keyword-matching-strategies
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub presets: Vec<AutoModerationKeywordPresetType>,
}
