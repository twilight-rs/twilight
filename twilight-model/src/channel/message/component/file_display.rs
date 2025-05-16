use serde::{Deserialize, Serialize};

use super::unfurled_media::UnfurledMediaItem;

/// A component displaying an uploaded file as an attachment.
///
/// Files are only available in messages.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct FileDisplay {
    /// Optional identifier for the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// This unfurled media item is unique in that it only supports attachment
    /// references using the `attachment://<filename>` syntax.
    pub file: UnfurledMediaItem,
    /// Whether the media should be a spoiler (or blurred out). Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spoiler: Option<bool>,
}
