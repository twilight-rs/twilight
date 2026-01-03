use crate::application::interaction::modal::ModalInteractionComponent;

/// User filled in [`Label`].
///
/// See [Discord Docs/Label Interaction Response Structure].
///
/// [`Label`]: crate::channel::message::component::Label
/// [Discord Docs/Label Interaction Response Structure]: https://discord.com/developers/docs/components/reference#label-label-interaction-response-structure
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModalInteractionLabel {
    /// Child component within the label.
    pub component: Box<ModalInteractionComponent>,
    /// Unique identifier for the component.
    pub id: i32,
}
