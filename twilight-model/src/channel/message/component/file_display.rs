use super::unfurled_media::UnfurledMediaItem;

/// A component displaying an uploaded file as an attachment.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FileDisplay {
    /// Optional identifier for the component.
    pub id: Option<i32>,
    /// The file to be displayed in the component.
    pub file: UnfurledMediaItem,
    /// Whether the file is spoilered (blurred).
    pub spoiler: Option<bool>,
}
