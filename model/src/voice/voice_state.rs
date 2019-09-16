use crate::{
    guild::Member,
    id::{ChannelId, GuildId, UserId},
};
use serde::{Deserialize, Serialize};
use serde_mappable_seq::Key;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VoiceState {
    pub channel_id: Option<ChannelId>,
    pub deaf: bool,
    pub guild_id: Option<GuildId>,
    pub member: Option<Member>,
    pub mute: bool,
    pub self_deaf: bool,
    pub self_mute: bool,
    /// Whether this user is streaming via "Go Live".
    #[serde(default)]
    pub self_stream: bool,
    pub session_id: String,
    pub suppress: bool,
    pub token: Option<String>,
    pub user_id: UserId,
}

impl Key<'_, UserId> for VoiceState {
    fn key(&self) -> UserId {
        self.user_id
    }
}
