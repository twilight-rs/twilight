//! Types for guild onboarding.

mod option;
mod prompt;
mod prompt_type;

use crate::id::{
    marker::{ChannelMarker, GuildMarker},
    Id,
};
use serde::{Deserialize, Serialize};

pub use self::{
    option::OnboardingPromptOption, prompt::OnboardingPrompt, prompt_type::OnboardingPromptType,
};

/// The onboarding data for a guild.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Onboarding {
    /// Channel IDs that new members get opted into automatically
    pub default_channel_ids: Vec<Id<ChannelMarker>>,
    /// Whether the guild has enabled onboarding.
    pub enabled: bool,
    /// ID of the guild this onboarding is a part of.
    pub guild_id: Id<GuildMarker>,
    /// Array of [`OnboardingPrompt`]s for the guild onboarding flow.
    pub prompts: Vec<OnboardingPrompt>,
}

#[cfg(test)]
mod tests {
    use super::Onboarding;
    use crate::id::Id;
    use serde_test::Token;

    #[test]
    fn onboarding() {
        let onboarding = Onboarding {
            default_channel_ids: Vec::new(),
            enabled: true,
            guild_id: Id::new(123_456_789),
            prompts: Vec::new(),
        };

        serde_test::assert_tokens(
            &onboarding,
            &[
                Token::Struct {
                    name: "Onboarding",
                    len: 4,
                },
                Token::Str("default_channel_ids"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("enabled"),
                Token::Bool(true),
                Token::Str("guild_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("123456789"),
                Token::Str("prompts"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::StructEnd,
            ],
        )
    }
}
