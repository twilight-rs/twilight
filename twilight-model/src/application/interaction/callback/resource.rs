use serde::Deserialize;

use crate::channel::Message;

use super::{
    activity_instance_resource::ActivityInstanceResource,
    interaction_callback_type::InteractionCallbackType,
};

/// Resource that was created by the interaction response.
/// 
/// See [Discord Docs/Interaction Callback Resource Object].
///
/// [Discord Docs/Interaction Callback Resource Object]: https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-callback-interaction-callback-resource-object
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct InteractionCallbackResource {
    /// Represents the Activity launched by this interaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_instance: Option<ActivityInstanceResource>,
    /// Interaction callback type
    #[serde(rename = "type")]
    pub kind: InteractionCallbackType,
    /// Message created by the interaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Message>,
}
