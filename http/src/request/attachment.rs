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
