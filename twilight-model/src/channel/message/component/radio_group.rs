use serde::{Deserialize, Serialize};

/// A single-selection group of radio options within a modal.
/// Radio groups are only available in modals and must be put inside a label
///
/// Fields' default values may be used by setting them to [`None`].
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct RadioGroup {
    /// Developer defined identifier.
    ///
    /// Between 1-100 characters
    pub custom_id: String,
    /// Optional identifier for the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// List of radio options.
    ///
    /// Must be between 2-10 options.
    pub options: Vec<RadioGroupOption>,
    /// Whether a selection is required.
    ///
    /// Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

/// Selectable radio options put into the radio group
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct RadioGroupOption {
    /// If the option is selected by default.
    ///
    /// Set to false if None is given
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    /// Optional description for the option.
    ///
    /// Up to 100 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// User-facing label of the option.
    ///
    /// Must be between 1-100 characters
    pub label: String,
    /// Developer defined identifier.
    ///
    /// Must be between 1-100 characters
    pub value: String,
}
