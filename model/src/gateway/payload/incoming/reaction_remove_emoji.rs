use crate::{
    channel::ReactionType,
    id::{marker, Id},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ReactionRemoveEmoji {
    pub channel_id: Id<marker::Channel>,
    pub emoji: ReactionType,
    pub guild_id: Id<marker::Guild>,
    pub message_id: Id<marker::Message>,
}
