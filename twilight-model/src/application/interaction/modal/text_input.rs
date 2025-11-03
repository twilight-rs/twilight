/// User filled in [`TextInput`].
///
/// See [Discord Docs/Text Input Interaction Response Structure].
///
/// [`TextInput`]: crate::channel::message::component::TextInput
/// [Discord Docs/Text Input Interaction Response Structure]: https://discord.com/developers/docs/components/reference#text-input-text-input-interaction-response-structure
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModalInteractionTextInput {
    /// User defined identifier for the component.
    ///
    /// See [Discord Docs/Custom ID].
    ///
    /// [Discord Docs/Custom ID]: https://discord.com/developers/docs/components/reference#anatomy-of-a-component-custom-id
    pub custom_id: String,
    /// Unique identifier for the component.
    pub id: i32,
    /// Value submitted by the user.
    pub value: String,
}
