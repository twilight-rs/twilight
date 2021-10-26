#[derive(Clone, Debug)]
pub struct AttachmentFile<'a> {
    pub(crate) filename: &'a str,
    pub(crate) description: Option<&'a str>,
    pub(crate) file: &'a [u8],
}

impl<'a> AttachmentFile<'a> {
    pub const fn from_bytes(filename: &'a str, file: &'a [u8]) -> Self {
        AttachmentFile {
            filename,
            description: None,
            file,
        }
    }

    pub const fn description(mut self, description: &'a str) -> Self {
        self.description = Some(description);

        self
    }
}
