use crate::{
    channel::ReactionType,
    guild::member::Member,
    id::{ChannelId, GuildId, MessageId, UserId},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Reaction {
    pub channel_id: ChannelId,
    pub emoji: ReactionType,
    pub guild_id: Option<GuildId>,
    pub member: Option<Member>,
    pub message_id: MessageId,
    pub user_id: UserId,
}
