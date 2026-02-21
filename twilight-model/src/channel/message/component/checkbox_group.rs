use serde::{Deserialize, Serialize};

/// A component for groups of checkboxes in a modal.
///
/// Checkbox groups are only available in modals and must be placed inside a label.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct CheckboxGroup {
    /// Optional identifier for the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Developer defined identifier.
    pub custom_id: String,
    /// List of checkbox options
    pub options: Vec<CheckboxGroupOption>,
    /// Minimum number of items that can be checked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_values: Option<u8>,
    /// Maximum number of items that can be checked
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_values: Option<u8>,
    /// Whether files have to be uploaded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

/// Checkboxes put into the checkbox group
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct CheckboxGroupOption {
    /// Developer defined identifier
    pub value: String,
    /// User-facing label of the option
    pub label: String,
    /// Optional description for the option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Shows the option as selected by default
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
}
