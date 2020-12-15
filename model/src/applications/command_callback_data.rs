use crate::channel::embed::Embed;
use serde::{Deserialize, Serialize};

//use crate::channel::message::MessageFlags;

/*
 * # InteractionApplicationCommandCallbackData
 *
 * | Name              | Value            |
 * |-------------------|------------------|
 * | tts?              | bool             |
 * | content           | string           |
 * | embeds?           | array of embeds  |
 * | allowed_mentions? | allowed mentions |
 * | flags             | int              |
 */
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct CommandCallbackData {
    pub tts: Option<bool>,
    pub content: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub embeds: Vec<Embed>,
    // TODO: Allowed mentions.
    // TODO: Figure out if flags should be added yet?
    // pub flags: MessageFlags,
}
