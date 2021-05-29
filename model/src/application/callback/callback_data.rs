use crate::channel::{
    embed::Embed,
    message::{AllowedMentions, MessageFlags},
};

use crate::component::Component;
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
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<Component>,
}

impl CallbackData {
    #[inline]
    pub fn builder() -> Builder {
        Builder::new()
    }
}

#[derive(Debug)]
pub struct Builder {
    pub allowed_mentions: Option<AllowedMentions>,
    pub content: Option<String>,
    pub embeds: Vec<Embed>,
    pub flags: Option<MessageFlags>,
    pub tts: Option<bool>,
}

impl Builder {
    #[inline]
    pub(crate) fn new() -> Builder {
        Builder::default()
    }

    pub fn with_allowed_mentions(mut self, mentions: AllowedMentions) -> Builder {
        self.allowed_mentions = Some(mentions);
        self
    }
}

impl Default for Builder {
    #[inline]
    fn default() -> Builder {
        Builder {
            allowed_mentions: None,
            content: None,
            embeds: vec![],
            flags: None,
            tts: None,
        }
    }
}
