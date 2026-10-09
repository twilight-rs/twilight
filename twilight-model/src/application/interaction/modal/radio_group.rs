/// User filled in [`RadioGroup`].
///
/// See [Discord Docs/Radio Group Interaction Response Structure]
///
/// [`RadioGroup`]: crate::channel::message::component::RadioGroup
/// [Discord Docs/Radio Group Interaction Response Structure]: https://docs.discord.com/developers/components/reference#radio-group-interaction-response-structure
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModalInteractionRadioGroup {
    /// [Discord Docs/Custom ID]: https://docs.discord.com/developers/components/reference#anatomy-of-a-component-custom-id
    pub custom_id: String,
    /// User defined identifier for the component.
    ///
    /// See [Discord Docs/Custom ID].
    ///
    /// Unique identifier for the component.
    pub id: i32,
    /// Value submitted by the user.
    pub value: Option<String>,
}
