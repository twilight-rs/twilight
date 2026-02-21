use crate::id::Id;
use crate::id::marker::AttachmentMarker;

/// User filled in [`FileUpload`].
///
/// See [Discord Docs/File Upload Interaction Response Structure]
///
/// [`FileUpload`]: crate::channel::message::component::FileUpload
/// [Discord Docs/File Upload Interaction Response Structure]: https://discord.com/developers/docs/components/reference#file-upload-file-upload-interaction-response-structure
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModalInteractionFileUpload {
    /// User defined identifier for the component.
    ///
    /// See [Discord Docs/Custom ID].
    ///
    /// [Discord Docs/Custom ID]: https://discord.com/developers/docs/components/reference#anatomy-of-a-component-custom-id
    pub custom_id: String,
    /// Unique identifier for the component.
    pub id: i32,
    /// IDs of the uploaded files found in [`ModalInteractionData::resolved`].
    ///
    /// [`ModalInteractionData::resolved`]: crate::application::interaction::modal::ModalInteractionData::resolved
    pub values: Vec<Id<AttachmentMarker>>,
}
