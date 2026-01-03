use serde::{Deserialize, Serialize};

use super::Component;

/// Top-level layout [`Component`].
///
/// Labels wrap modal components with text as a label and an optional description.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct Label {
    /// The component within the label.
    pub component: Box<Component>,
    /// An optional description text for the label; max 100 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional identifier for the label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// The label text; max 45 characters.
    #[allow(clippy::struct_field_names)]
    pub label: String,
}
