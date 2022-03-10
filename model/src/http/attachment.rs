//! Models used when sending attachments to Discord.

use serde::{Deserialize, Serialize};

/// Attachment for when creating and updating messages.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Attachment {
    pub description: Option<String>,
    pub file: Vec<u8>,
    pub filename: String,
}

impl Attachment {
    /// Create a attachment from a filename and bytes.
    pub const fn from_bytes(filename: String, file: Vec<u8>) -> Self {
        Self {
            description: None,
            file,
            filename,
        }
    }

    /// Set the description of a attachment, this is used for alt-text
    /// on Discords end.
    pub fn description(&mut self, description: String) {
        self.description = Some(description);
    }
}
