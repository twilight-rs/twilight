use serde::{Deserialize, Serialize};

/// A component allowing uploading files in a modal.
///
/// File uploads are only available in modals and must be placed inside a label.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct FileUpload {
    /// Optional identifier for the component.
    pub id: Option<i32>,
    /// Developer defined identifier.
    pub custom_id: String,
    /// Maximum number of items that can be uploaded.
    pub max_values: Option<u8>,
    /// Minimum number of items that can be uploaded.
    pub min_values: Option<u8>,
    /// Whether files have to be uploaded.
    pub required: Option<bool>,
}
