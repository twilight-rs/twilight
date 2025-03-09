use serde::{Deserialize, Serialize};

use super::unfurled_media::UnfurledMediaItem;

/// Component used to organize a set of [`MediaGalleryItem`]s.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MediaGallery {
    /// Unique identifier of the media gallery.
    pub id: Option<i32>,
    /// List of media gallery items.
    pub items: Vec<MediaGalleryItem>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct MediaGalleryItem {
    pub media: UnfurledMediaItem,
    pub description: Option<String>,
    pub spoiler: Option<bool>,
}
