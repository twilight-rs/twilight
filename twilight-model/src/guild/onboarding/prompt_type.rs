use serde::{Deserialize, Serialize};

/// The type of an onboarding prompt.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(from = "u8", into = "u8")]
pub enum OnboardingPromptType {
    /// A prompt that allows the user to select multiple options.
    MultipleChoice,
    /// A prompt that allows the user to use a dropdown to select an option.
    DropDown,
    /// An unknown prompt type.
    Unknown(u8),
}

impl OnboardingPromptType {
    pub const fn name(self) -> &'static str {
        match self {
            Self::MultipleChoice => "MultipleChoice",
            Self::DropDown => "DropDown",
            Self::Unknown(_) => "Unknown",
        }
    }
}

impl From<u8> for OnboardingPromptType {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::MultipleChoice,
            2 => Self::DropDown,
            unknown => Self::Unknown(unknown),
        }
    }
}

impl From<OnboardingPromptType> for u8 {
    fn from(value: OnboardingPromptType) -> Self {
        match value {
            OnboardingPromptType::MultipleChoice => 1,
            OnboardingPromptType::DropDown => 2,
            OnboardingPromptType::Unknown(unknown) => unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::OnboardingPromptType;
    use serde_test::{assert_tokens, Token};

    #[test]
    fn onboarding_prompt_type() {
        const MAP: &[(OnboardingPromptType, u8, &str)] = &[
            (OnboardingPromptType::MultipleChoice, 1, "MultipleChoice"),
            (OnboardingPromptType::DropDown, 2, "DropDown"),
            (OnboardingPromptType::Unknown(3), 3, "Unknown"),
        ];

        for (prompt_type, number, name) in MAP {
            assert_eq!(prompt_type.name(), *name);
            assert_eq!(u8::from(*prompt_type), *number);
            assert_eq!(OnboardingPromptType::from(*number), *prompt_type);
            assert_tokens(number, &[Token::U8(*number)])
        }
    }
}
