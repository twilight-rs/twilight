use serde::{Deserialize, Serialize};

use super::unfurled_media::UnfurledMediaItem;

/// Component used to organize a set of [`MediaGalleryItem`]s.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct MediaGallery {
    /// Optional identifier for the media gallery.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// 1 to 10 media gallery items.
    pub items: Vec<MediaGalleryItem>,
}

/// A media gallery item.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct MediaGalleryItem {
    /// An unfurled media item containing a url or attachment.
    pub media: UnfurledMediaItem,
    /// Alt text for the media gallery item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether the media should be a spoiler (or blurred out). Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spoiler: Option<bool>,
}
