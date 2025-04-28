use twilight_model::channel::message::component::{FileDisplay, UnfurledMediaItem};

#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into an file display"]
pub struct FileDisplayBuilder(FileDisplay);

impl FileDisplayBuilder {
    pub fn new(file: UnfurledMediaItem) -> Self {
        Self(FileDisplay {
            id: None,
            file,
            spoiler: None,
        })
    }

    pub fn id(mut self, id: i32) -> Self {
        self.0.id.replace(id);

        self
    }

    pub fn file(mut self, file: impl Into<UnfurledMediaItem>) -> Self {
        self.0.file = file.into();

        self
    }

    pub fn spoiler(mut self, spoiler: bool) -> Self {
        self.0.spoiler.replace(spoiler);

        self
    }

    pub fn build(self) -> FileDisplay {
        self.0
    }
}

impl From<FileDisplay> for FileDisplayBuilder {
    fn from(file_display: FileDisplay) -> Self {
        Self(file_display)
    }
}
