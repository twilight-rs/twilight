/// User filled in [`Checkbox`].
///
/// See [Discord Docs/File Upload Interaction Response Structure]
///
/// [`Checkbox`]: crate::channel::message::component::Checkbox
/// [Discord Docs/File Upload Interaction Response Structure]: https://discord.com/developers/docs/components/reference#checkbox-checkbox-interaction-response-structure
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModalInteractionCheckbox {
    /// User defined identifier for the component.
    ///
    /// See [Discord Docs/Custom ID].
    ///
    /// Unique identifier for the component.
    pub id: i32,
    /// [Discord Docs/Custom ID]: https://discord.com/developers/docs/components/reference#anatomy-of-a-component-custom-id
    pub custom_id: String,
    /// Value submitted by the user.
    pub value: bool,
}
