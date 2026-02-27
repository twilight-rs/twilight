/// User filled in [`CheckboxGroup`].
///
/// See [Discord Docs/File Upload Interaction Response Structure]
///
/// [`CheckboxGroup`]: crate::channel::message::component::CheckboxGroup
/// [Discord Docs/File Upload Interaction Response Structure]: https://discord.com/developers/docs/components/reference#checkbox-group-checkbox-group-interaction-response-structure
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModalInteractionCheckboxGroup {
    /// [Discord Docs/Custom ID]: https://discord.com/developers/docs/components/reference#anatomy-of-a-component-custom-id
    pub custom_id: String,
    /// User defined identifier for the component.
    ///
    /// See [Discord Docs/Custom ID].
    ///
    /// Unique identifier for the component.
    pub id: i32,
    /// Value submitted by the user.
    pub values: Vec<String>,
}
