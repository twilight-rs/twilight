use twilight_model::channel::message::component::FileUpload;

/// Create a file upload with a builder.
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into a file upload"]
pub struct FileUploadBuilder(FileUpload);

impl FileUploadBuilder {
    /// Create a new file upload builder.
    pub fn new(custom_id: impl Into<String>) -> Self {
        Self(FileUpload {
            id: None,
            custom_id: custom_id.into(),
            max_values: None,
            min_values: None,
            required: None,
            file_types: None,
        })
    }

    /// Set the identifier of this file upload.
    pub const fn id(mut self, id: i32) -> Self {
        self.0.id.replace(id);

        self
    }

    /// Set the maximum amount of files to upload.
    pub const fn max_values(mut self, max_values: u8) -> Self {
        self.0.max_values.replace(max_values);

        self
    }

    /// Set the minimum amount of files to upload.
    pub const fn min_values(mut self, min_values: u8) -> Self {
        self.0.min_values.replace(min_values);

        self
    }

    /// Set the file types to filter for.
    ///
    /// Can be `image`, `video`, `audio`, or any dot-prefixed extension such as `.pdf`.
    pub fn file_types(mut self, file_types: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.0.file_types = Some(file_types.into_iter().map(Into::into).collect());

        self
    }

    /// Set whether uploading a file is required.
    pub const fn required(mut self, required: bool) -> Self {
        self.0.required.replace(required);

        self
    }

    /// Build into a file upload,
    pub fn build(self) -> FileUpload {
        self.0
    }
}

impl From<FileUploadBuilder> for FileUpload {
    fn from(builder: FileUploadBuilder) -> Self {
        builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder() {
        let expected = FileUpload {
            id: Some(42),
            custom_id: "custom_id".to_string(),
            max_values: Some(5),
            min_values: Some(1),
            file_types: None,
            required: Some(true),
        };

        let actual = FileUploadBuilder::new("custom_id")
            .id(42)
            .max_values(5)
            .min_values(1)
            .required(true)
            .build();

        assert_eq!(actual, expected);
    }
}
