use serde::{Deserialize, Serialize};

use super::unfurled_media::UnfurledMediaItem;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MediaGallery {
    pub id: Option<i32>,
    pub items: Vec<MediaGalleryItems>,

}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct MediaGalleryItems {
    pub media: UnfurledMediaItem,
    pub description: Option<String>,
    pub spoiler: Option<bool>,
}
