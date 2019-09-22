use crate::channel::ReactionType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MessageReaction {
    pub count: u64,
    pub emoji: ReactionType,
    pub me: bool,
}
