use crate::id::{ChannelId, GuildId, UserId};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct InviteCreate {
    pub channel_id: ChannelId,
    pub code: String,
    pub created_at: String,
    pub guild_id: GuildId,
    pub inviter: Option<PartialUser>,
    pub max_age: u64,
    pub max_uses: u64,
    pub temporary: bool,
    pub uses: u8, // will always be zero
}

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct PartialUser {
    avatar: Option<String>,
    discriminator: String,
    id: UserId,
    username: String,
}
