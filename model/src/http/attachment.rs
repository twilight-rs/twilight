//! Models used when sending attachments to Discord.

use serde::{Deserialize, Serialize};

/// Attachment for when creating and updating messages.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Attachment {
    pub description: Option<String>,
    #[serde(skip)]
    pub file: Vec<u8>,
    pub filename: String,
    pub id: u64,
}

impl Attachment {
    /// Create a attachment from a filename and bytes.
    pub const fn from_bytes(filename: String, file: Vec<u8>, id: u64) -> Self {
        Self {
            description: None,
            file,
            filename,
            id,
        }
    }

    /// Set the description of a attachment, this is used for alt-text
    /// on Discords end.
    pub fn description(&mut self, description: String) {
        self.description = Some(description);
    }
}
