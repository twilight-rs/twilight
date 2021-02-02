use crate::channel::embed::Embed;

use serde::{Deserialize, Serialize};

/// Optional extra data sent when responding to an [`Interaction`] of type
/// [`ApplicationCommand`].
///
/// This is used when intending to send a message in the response.
///
/// [`Interaction`]: crate::applications::interaction::Interaction
/// [`ApplicationCommand`]: crate::applications::interaction::Interaction::ApplicationCommand
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct CommandCallbackData {
    pub tts: Option<bool>,
    pub content: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub embeds: Vec<Embed>,
}
