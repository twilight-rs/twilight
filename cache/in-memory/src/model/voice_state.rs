use twilight_model::{
    id::{ChannelId, GuildId, UserId},
    voice::VoiceState,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CachedVoiceState {
    pub channel_id: Option<ChannelId>,
    pub deaf: bool,
    pub guild_id: Option<GuildId>,
    pub mute: bool,
    pub self_deaf: bool,
    pub self_mute: bool,
    pub self_stream: bool,
    pub session_id: String,
    pub suppress: bool,
    pub token: Option<String>,
    pub user_id: UserId,
}

impl PartialEq<VoiceState> for CachedVoiceState {
    fn eq(&self, other: &VoiceState) -> bool {
        self.channel_id == other.channel_id
            && self.deaf == other.deaf
            && self.guild_id == other.guild_id
            && self.mute == other.mute
            && self.self_deaf == other.self_deaf
            && self.self_mute == other.self_mute
            && self.self_stream == other.self_stream
            && self.session_id == other.session_id
            && self.suppress == other.suppress
            && self.token == other.token
            && self.user_id == other.user_id
    }
}
