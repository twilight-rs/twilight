use crate::id::{marker::OnboardingPromptMarker, Id};
use serde::{Deserialize, Serialize};

use super::{OnboardingPromptOption, OnboardingPromptType};

/// A prompt in the onboarding flow.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct OnboardingPrompt {
    /// The id of the prompt.
    pub id: Id<OnboardingPromptMarker>,
    /// Whether this prompt is in the onboarding flow.
    pub in_onboarding: bool,
    /// The [`OnboardingPromptType`] of the prompt.
    #[serde(rename = "type")]
    pub kind: OnboardingPromptType,
    /// The array of [`OnboardingPromptOption`]s available to the prompt.
    #[serde(default)]
    pub options: Vec<OnboardingPromptOption>,
    /// Whether this prompt is required in the obboarding flow.
    pub required: bool,
    /// Whether this prompt allows selecting only one option.
    pub single_select: bool,
    /// The title of the prompt.
    pub title: String,
}

#[cfg(test)]
mod tests {
    use serde_test::Token;

    use super::OnboardingPrompt;
    use crate::{guild::onboarding::OnboardingPromptType, id::Id};

    #[test]
    fn onboarding_prompt() {
        let prompt = OnboardingPrompt {
            id: Id::new(123_456_789),
            in_onboarding: true,
            kind: OnboardingPromptType::MultipleChoice,
            options: Vec::new(),
            required: true,
            single_select: true,
            title: String::from("a prompt"),
        };

        serde_test::assert_tokens(
            &prompt,
            &[
                Token::Struct {
                    name: "OnboardingPrompt",
                    len: 7,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("123456789"),
                Token::Str("in_onboarding"),
                Token::Bool(true),
                Token::Str("type"),
                Token::UnitVariant {
                    name: "OnboardingPromptType",
                    variant: "MultipleChoice",
                },
                Token::Str("options"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("required"),
                Token::Bool(true),
                Token::Str("single_select"),
                Token::Bool(true),
                Token::Str("title"),
                Token::Str("a prompt"),
                Token::StructEnd,
            ],
        );
    }
}
