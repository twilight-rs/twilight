use crate::id::EmojiId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Hash, Serialize)]
#[serde(untagged)]
pub enum ReactionType {
    Custom {
        animated: bool,
        id: EmojiId,
        name: Option<String>,
    },
    Unicode(String),
}
