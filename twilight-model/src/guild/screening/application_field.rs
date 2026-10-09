use serde::{Deserialize, Serialize};

/// Field where applicant selects one of many options.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MultipleChoiceFieldResponse {
    /// Choices applicant can select from.
    pub choices: Vec<String>,
    /// Optional helper text shown below label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Label shown above field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Whether applicant must fill in field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// Index of choice selected by applicant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<i32>,
}

/// A text input field.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct TextFieldResponse {
    /// Optional helper text shown below label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Label shown above field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Placeholder text shown in empty input
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    /// Whether applicant must fill in field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// Applicant's text response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<String>,
}

/// A terms acceptance field.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct TermsFieldResponse {
    /// Terms applicant must acknowledge.
    pub values: Vec<String>,
    /// Optional helper text shown below label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Label shown above field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Placeholder text shown in empty input
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    /// Whether applicant must fill in field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// Whether applicant accepted terms
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<bool>,
}

/// A field within a join application used for member screening.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(tag = "field_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApplicationFieldResponse {
    /// A field that allows the user to select one of multiple options.
    MultipleChoice(MultipleChoiceFieldResponse),
    /// A large text field that allows the user to enter up to 1000 characters.
    Paragraph(TextFieldResponse),
    /// Field requiring applicant to acknowledge list of terms
    Terms(TermsFieldResponse),
    /// A small text field that allows the user to enter up to 150 characters.
    TextInput(TextFieldResponse),
}
