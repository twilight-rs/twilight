use serde::{Deserialize, Serialize};

/// A group of selectable checkboxes within a modal.
/// Checkbox groups are only available in modals and must be put inside a label
///
/// Fields' default values may be used by setting them to [`None`].
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct CheckboxGroup {
    /// Developer defined identifier.
    ///
    /// Between 1-100 characters
    pub custom_id: String,
    /// Optional identifier for the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Maximum number of items that can be checked.
    ///
    /// Must be between 1-10.
    ///
    /// Defaults to the number of options given.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_values: Option<u8>,
    /// Minimum number of options that must be selected.
    ///
    /// Must be between 0 and 10, inclusive.
    ///
    /// Defaults to `1`.
    ///
    /// If set to `0`, [`required`] must be `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_values: Option<u8>,
    /// List of checkbox options.
    ///
    /// Must be between 1-10 options.
    pub options: Vec<CheckboxGroupOption>,
    /// Whether at least one option must be selected.
    ///
    /// Defaults to `true`.
    ///
    /// If [`min_values`] is set to `0`, this must be `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

/// Selectable checkbox options put into the checkbox group
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct CheckboxGroupOption {
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
