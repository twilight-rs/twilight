use serde::{Deserialize, Serialize};

use super::UnfurledMediaItem;

/// Thumbnail containing a small image. Only usable as a accessory in a [section].
///
/// [section]: super::Section
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct Thumbnail {
    /// Optional identifier for component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// A unfurled media item containing either a url or attachment.
    pub media: UnfurledMediaItem,
    /// Alt text for the media.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether the thumbnail should be a spoiler (or blurred
    /// out). Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spoiler: Option<bool>,
}
