use crate::channel::{
    embed::Embed,
    message::{AllowedMentions, MessageFlags},
};

use serde::{Deserialize, Serialize};

/// Optional extra data sent when responding to an [`Interaction`] of type
/// [`ApplicationCommand`].
///
/// This is used when intending to send a message in the response.
///
/// [`Interaction`]: crate::application::interaction::Interaction
/// [`ApplicationCommand`]: crate::application::interaction::Interaction::ApplicationCommand
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct CallbackData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_mentions: Option<AllowedMentions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub embeds: Vec<Embed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<MessageFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<bool>,
}
