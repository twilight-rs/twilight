use serde::{Deserialize, Serialize};

/// A component allowing a checkbox in a modal.
///
/// checkboxes are only available in modals and must be placed inside a label.
///
/// Fields' default values may be used by setting them to [`None`].
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct Checkbox {
    /// Developer defined identifier.
    ///
    /// Must be between 1-100 characters
    pub custom_id: String,
    /// Whether the checkbox is selected by default.
    ///
    /// Set to false if None is given
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    /// Optional identifier for the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}
