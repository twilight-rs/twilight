use serde::{Deserialize, Serialize};

use crate::{
    id::{
        AnonymizableId, Id,
        marker::{GuildMarker, InteractionMarker, MessageMarker, UserMarker},
    },
    oauth::ApplicationIntegrationMap,
    user::User,
};

use super::InteractionType;

/// Structure containing metadata for interactions.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InteractionMetadata {
    /// IDs for installation context(s) related to an interaction.
    pub authorizing_integration_owners:
        ApplicationIntegrationMap<AnonymizableId<GuildMarker>, Id<UserMarker>>,
    /// ID of the interaction.
    pub id: Id<InteractionMarker>,
    /// ID of the message that contained interactive component, present only on
    /// messages created from component interactions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interacted_message_id: Option<Id<MessageMarker>>,
    /// Type of interaction.
    #[serde(rename = "type")]
    pub kind: InteractionType,
    /// ID of the original response message, present only on follow-up messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_response_message_id: Option<Id<MessageMarker>>,
    /// ID of the message the command was run on, present only on
    /// message command interactions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_message_id: Option<Id<MessageMarker>>,
    /// User the command was run on, present only on user command
    /// interactions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_user: Option<User>,
    /// Metadata for the interaction that was used to open the modal,
    /// present only on modal submit interactions
    // This field cannot be in the nested interaction metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggering_interaction_metadata: Option<Box<InteractionMetadata>>,
    /// User who triggered the interaction.
    pub user: User,
}
