use crate::id::Id;
use crate::id::marker::{ChannelMarker, GenericMarker, RoleMarker, UserMarker};
use serde::{Deserialize, Serialize};

pub type ModalInteractionStringSelect = ModalInteractionSelectMenu<String>;
pub type ModalInteractionUserSelect = ModalInteractionSelectMenu<Id<UserMarker>>;
pub type ModalInteractionRoleSelect = ModalInteractionSelectMenu<Id<RoleMarker>>;
pub type ModalInteractionMentionableSelect = ModalInteractionSelectMenu<Id<GenericMarker>>;
pub type ModalInteractionChannelSelect = ModalInteractionSelectMenu<Id<ChannelMarker>>;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ModalInteractionSelectMenu<ValueType> {
    pub id: i32,
    pub custom_id: String,
    pub values: Vec<ValueType>,
}
