pub mod option;
pub mod prompt;
pub mod prompt_type;

use serde::{Deserialize, Serialize};

use crate::id::marker::GuildMarker;
use crate::id::{marker::ChannelMarker, Id};

pub use self::option::OnboardingPromptOption;
pub use self::prompt::OnboardingPrompt;
pub use self::prompt_type::OnboardingPromptType;

/// The onboarding data for a guild.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Onboarding {
    /// Channel ids that new members get opted into automatically
    default_channel_ids: Vec<Id<ChannelMarker>>,
    /// Whether the guild has enabled onboarding
    enabled: bool,
    /// The id of the guild this onboarding is a part of.
    guild_id: Id<GuildMarker>,
    /// The array of [`OnboardingPrompt`]s for the guild onboarding flow.
    prompts: Vec<OnboardingPrompt>,
}

#[cfg(test)]
mod tests {
    use crate::id::{marker::GuildMarker, Id};

    use super::Onboarding;
    use serde_test::Token;

    #[test]
    fn onboarding() {
        let onboarding = Onboarding {
            default_channel_ids: Vec::new(),
            enabled: true,
            guild_id: Id::<GuildMarker>::new(123_456_789),
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
