use serde::{Deserialize, Serialize};

/// The type of prompt.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum OnboardingPromptType {
    /// A prompt that allows the user to select multiple options.
    MultipleChoice,
    /// A prompt that allows the user to use a dropdown to select an option.
    DropDown,
}
