use serde::{Deserialize, Serialize};

use crate::{
    id::{
        marker::{GuildMarker, InteractionMarker, MessageMarker, UserMarker},
        AnonymizableId, Id,
    },
    oauth::ApplicationIntegrationMap,
};

use super::InteractionType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InteractionMetadata {
    pub id: Id<InteractionMarker>,
    #[serde(rename = "type")]
    pub kind: InteractionType,
    pub user_id: Id<UserMarker>,
    pub authorizing_integration_owners:
        ApplicationIntegrationMap<AnonymizableId<GuildMarker>, Id<UserMarker>>,
    pub original_response_message_id: Option<Id<MessageMarker>>,
    pub interacted_message_id: Option<Id<MessageMarker>>,
    // This field cannot be in the nested interaction metadata.
    pub triggering_interaction_metadata: Box<InteractionMetadata>,
}
