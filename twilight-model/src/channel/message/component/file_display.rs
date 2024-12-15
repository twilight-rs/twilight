use super::unfurled_media::UnfurledMediaItem;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FileDisplay {
    pub id: Option<i32>,
    pub file: UnfurledMediaItem,
    pub spoiler: Option<bool>,
}
