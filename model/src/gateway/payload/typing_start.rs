use crate::{
    guild::Member,
    id::{ChannelId, GuildId, UserId},
};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TypingStart {
    pub channel_id: ChannelId,
    pub guild_id: Option<GuildId>,
    pub timestamp: u64,
    pub user_id: UserId,
    pub member: Option<Member>,
}
