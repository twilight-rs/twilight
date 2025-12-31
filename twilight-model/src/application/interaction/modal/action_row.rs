use crate::application::interaction::modal::ModalInteractionComponent;

/// User filled in [`ActionRow`].
///
/// [`ActionRow`]: crate::channel::message::component::ActionRow
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModalInteractionActionRow {
    /// List of components.
    pub components: Vec<ModalInteractionComponent>,
    /// Unique identifier for the component.
    pub id: i32,
}
