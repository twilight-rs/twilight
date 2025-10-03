use crate::application::interaction::modal::ModalInteractionComponent;
use crate::channel::message::component::ComponentType;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};

/// User filled in [`ActionRow`].
///
/// See [Discord Docs/Modal Submit Data Structure].
///
/// [`ActionRow`]: crate::channel::message::component::ActionRow
/// [Discord Docs/Modal Submit Data Structure]: https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-modal-submit-data-structure
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct ModalInteractionActionRow {
    pub id: i32,
    /// List of components.
    pub components: Vec<ModalInteractionComponent>,
}

impl Serialize for ModalInteractionActionRow {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("ModalInteractionDataActionRow", 2)?;

        state.serialize_field("type", &ComponentType::ActionRow)?;
        state.serialize_field("components", &self.components)?;

        state.end()
    }
}
