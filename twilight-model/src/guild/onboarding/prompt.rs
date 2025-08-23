use crate::id::{Id, marker::OnboardingPromptMarker};
use serde::{Deserialize, Serialize};

use super::{OnboardingPromptOption, OnboardingPromptType};

/// A prompt in the onboarding flow.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct OnboardingPrompt {
    /// ID of the prompt.
    pub id: Id<OnboardingPromptMarker>,
    /// Whether this prompt is in the onboarding flow.
    pub in_onboarding: bool,
    /// [`OnboardingPromptType`] of the prompt.
    #[serde(rename = "type")]
    pub kind: OnboardingPromptType,
    /// Array of [`OnboardingPromptOption`]s available to the prompt.
    pub options: Vec<OnboardingPromptOption>,
    /// Whether this prompt is required in the onboarding flow.
    pub required: bool,
    /// Whether this prompt allows selecting only one option.
    pub single_select: bool,
    /// Title of the prompt.
    pub title: String,
}

#[cfg(test)]
mod tests {
    use super::OnboardingPrompt;
    use crate::{guild::onboarding::OnboardingPromptType, id::Id};
    use serde_test::Token;

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
                Token::U8(0),
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
