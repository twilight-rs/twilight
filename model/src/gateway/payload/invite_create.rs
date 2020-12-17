use crate::{
    id::{ChannelId, GuildId, UserId},
    invite::TargetUserType,
    user::User,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct InviteCreate {
    pub channel_id: ChannelId,
    pub code: String,
    pub created_at: String,
    pub guild_id: GuildId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inviter: Option<User>,
    pub max_age: u64,
    pub max_uses: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_user_type: Option<TargetUserType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_user: Option<PartialUser>,
    pub temporary: bool,
    pub uses: u8, // will always be zero
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PartialUser {
    avatar: Option<String>,
    discriminator: String,
    id: UserId,
    username: String,
}
