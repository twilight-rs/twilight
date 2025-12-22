/// [`TextDisplay`] component contained in a [`ModalSubmit`] interaction.
///
/// See [Discord Docs/Text Display Interaction Response Structure].
///
/// [`TextDisplay`]: crate::channel::message::component::TextDisplay
/// [`ModalSubmit`]: crate::application::interaction::InteractionType::ModalSubmit
/// [Discord Docs/Text Display Interaction Response Structure]: https://discord.com/developers/docs/components/reference#text-display-text-display-interaction-response-structure
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModalInteractionTextDisplay {
    /// Unique identifier for the component.
    pub id: i32,
}
