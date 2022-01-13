use serde::{Deserialize, Serialize};
use twilight_model::id::{marker::AttachmentMarker, Id};

/// Attachment for when creating and updating messages.
#[derive(Clone, Debug)]
pub struct AttachmentFile<'a> {
    pub(crate) description: Option<&'a str>,
    pub(crate) file: &'a [u8],
    pub(crate) filename: &'a str,
}

impl<'a> AttachmentFile<'a> {
    /// Create a attachment from a filename and bytes.
    pub const fn from_bytes(filename: &'a str, file: &'a [u8]) -> Self {
        AttachmentFile {
            description: None,
            file,
            filename,
        }
    }

    /// Set the description of a attachment, this is used for alt-text
    /// on Discords end.
    pub const fn description(mut self, description: &'a str) -> Self {
        self.description = Some(description);

        self
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PartialAttachment<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<&'a str>,
    pub id: u64,
}

impl PartialAttachment<'_> {
    pub const fn from_id(id: Id<AttachmentMarker>) -> Self {
        Self {
            description: None,
            filename: None,
            id: id.get(),
        }
    }
}
