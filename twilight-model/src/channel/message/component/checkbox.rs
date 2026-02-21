use serde::{Deserialize, Serialize};

/// A component allowing a checkbox in a modal.
///
/// checkboxes are only available in modals and must be placed inside a label.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct Checkbox {
    /// Optional identifier for the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Developer defined identifier.
    pub custom_id: String,
    /// Whether the checkbox is selected by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
}
