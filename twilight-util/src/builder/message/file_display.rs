use twilight_model::channel::message::component::{FileDisplay, UnfurledMediaItem};

/// Create a file display with a builder.
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into an file display"]
pub struct FileDisplayBuilder(FileDisplay);

impl FileDisplayBuilder {
    /// Create a new file display builder.
    pub fn new(file: UnfurledMediaItem) -> Self {
        Self(FileDisplay {
            id: None,
            file,
            spoiler: None,
        })
    }

    /// Set the identifier of this file display.
    pub fn id(mut self, id: i32) -> Self {
        self.0.id.replace(id);

        self
    }

    /// Set the file of this file display.
    pub fn file(mut self, file: impl Into<UnfurledMediaItem>) -> Self {
        self.0.file = file.into();

        self
    }

    /// Specify whether this file display is spoilered.
    pub fn spoiler(mut self, spoiler: bool) -> Self {
        self.0.spoiler.replace(spoiler);

        self
    }

    /// Build into a file display.
    pub fn build(self) -> FileDisplay {
        self.0
    }
}

impl From<FileDisplayBuilder> for FileDisplay {
    fn from(builder: FileDisplayBuilder) -> Self {
        builder.build()
    }
}
