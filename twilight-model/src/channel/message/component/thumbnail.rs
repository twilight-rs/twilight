use serde::{Deserialize, Serialize};

use super::UnfurledMediaItem;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct Thumbnail {
    pub id: Option<i32>,
    pub media: UnfurledMediaItem,
    pub description: Option<String>,
    pub spoiler: Option<bool>,
}
