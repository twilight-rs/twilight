use crate::{
    id::{ChannelId, GuildId},
    user::User,
};

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
    pub inviter: User, // TODO: This is a partial user, need feedback on whether to make a new struct for it here, or use something else
    pub max_age: u64,
    pub max_uses: u64,
    pub temporary: bool,
    pub uses: u8, // will always be zero
}
