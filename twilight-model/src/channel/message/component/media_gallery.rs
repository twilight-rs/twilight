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

/// A media gallery item.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct MediaGalleryItem {
    /// The attachment for this media gallery item.
    pub media: UnfurledMediaItem,
    /// The description of this media gallery item.
    pub description: Option<String>,
    /// Whether this media gallery item is spoilered (blurred).
    pub spoiler: Option<bool>,
}
