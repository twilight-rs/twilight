pub mod activity_instance_resource;
pub mod interaction_callback_type;
pub mod resource;
pub mod response;

use serde::Deserialize;

use crate::id::{
    Id,
    marker::{InteractionMarker, MessageMarker},
};

use super::InteractionType;

/// Interaction object associated with the interaction response.
/// 
/// See [Discord Docs/Interaction Callback Object]
/// 
/// [Discord Docs/Interaction Callback Object]: https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-callback-interaction-callback-object
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct InteractionCallback {
    /// Instance ID of the Activity if one was launched or joined
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_instance_id: Option<String>,
    /// ID of the interaction.
    pub id: Id<InteractionMarker>,
    /// Interaction type.
    #[serde(rename = "type")]
    pub kind: InteractionType,
    /// Whether the response message is ephemeral
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_message_ephemeral: Option<bool>,
    /// ID of the message that was created by the interaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_message_id: Option<Id<MessageMarker>>,
    /// Whether the message is in a loading state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_message_loading: Option<bool>,
}
