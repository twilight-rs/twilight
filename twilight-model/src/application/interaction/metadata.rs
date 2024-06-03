use serde::{Deserialize, Serialize};

use crate::{
    id::{
        marker::{GuildMarker, InteractionMarker, MessageMarker, UserMarker},
        AnonymizableId, Id,
    },
    oauth::ApplicationIntegrationMap,
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
    /// Metadata for the interaction that was used to open the modal,
    /// present only on modal submit interactions
    // This field cannot be in the nested interaction metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggering_interaction_metadata: Option<Box<InteractionMetadata>>,
    /// ID of the user who triggered the interaction.
    pub user_id: Id<UserMarker>,
}
