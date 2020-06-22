use crate::{
    guild::Member,
    id::{ChannelId, GuildId, UserId},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct TypingStart {
    pub channel_id: ChannelId,
    pub guild_id: Option<GuildId>,
    pub timestamp: u64,
    pub user_id: UserId,
    pub member: Option<Member>,
}
