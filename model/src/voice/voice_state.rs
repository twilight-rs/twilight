use crate::{
    guild::Member,
    id::{ChannelId, GuildId, UserId},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VoiceState {
    pub channel_id: Option<ChannelId>,
    pub deaf: bool,
    pub guild_id: Option<GuildId>,
    pub member: Option<Member>,
    pub mute: bool,
    pub self_deaf: bool,
    pub self_mute: bool,
    pub session_id: String,
    pub suppress: bool,
    pub token: Option<String>,
    pub user_id: UserId,
}
