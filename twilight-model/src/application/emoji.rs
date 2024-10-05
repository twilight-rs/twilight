use serde::{Deserialize, Serialize};

use crate::guild::Emoji;

/// List of application emojis
#[derive(Clone, Debug, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct EmojiList {
    /// List of application emojis
    pub items: Vec<Emoji>,
}
