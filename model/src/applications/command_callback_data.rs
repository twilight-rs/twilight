use crate::channel::embed::Embed;

use serde::{Deserialize, Serialize};

/// Optional extra data sent when responding with an
/// [`Interaction`](crate::applications::Interaction) of type ApplicationCommand
/// when intending to send a message in the response.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct CommandCallbackData {
    pub tts: Option<bool>,
    pub content: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub embeds: Vec<Embed>,
}
