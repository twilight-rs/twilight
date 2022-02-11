use crate::{
    application::interaction::InteractionType,
    guild::PartialMember,
    id::{marker::InteractionMarker, Id},
    user::User,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MessageInteraction {
    /// ID of the interaction.
    pub id: Id<InteractionMarker>,
    /// Type of the interaction.
    #[serde(rename = "type")]
    pub kind: InteractionType,
    /// If in a guild, the member who invoked the interaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<PartialMember>,
    /// Name of the [`Command`].
    ///
    /// [`Command`]: crate::application::command::Command
    pub name: String,
    /// User who invoked the interaction.
    pub user: User,
}
