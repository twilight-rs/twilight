use crate::id::AttachmentId;
use std::hash::{Hash, Hasher};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Attachment {
    pub id: AttachmentId,
    pub filename: String,
    pub height: Option<u64>,
    pub proxy_url: String,
    pub size: u64,
    pub url: String,
    pub width: Option<u64>,
}

impl Hash for Attachment {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
