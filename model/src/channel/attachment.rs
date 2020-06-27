use crate::id::AttachmentId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Attachment {
    pub filename: String,
    pub height: Option<u64>,
    pub id: AttachmentId,
    pub proxy_url: String,
    pub size: u64,
    pub url: String,
    pub width: Option<u64>,
}
