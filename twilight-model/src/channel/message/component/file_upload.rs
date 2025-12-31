use serde::{Deserialize, Serialize};

/// A component allowing uploading files in a modal.
///
/// File uploads are only available in modals and must be placed inside a label.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct FileUpload {
    /// Developer defined identifier.
    pub custom_id: String,
    /// Optional identifier for the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Maximum number of items that can be uploaded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_values: Option<u8>,
    /// Minimum number of items that can be uploaded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_values: Option<u8>,
    /// Whether files have to be uploaded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}
