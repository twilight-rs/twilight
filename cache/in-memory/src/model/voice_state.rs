use serde::Serialize;
use twilight_model::{
    id::{ChannelId, GuildId, UserId},
    voice::VoiceState,
};

/// Represents a cached [`VoiceState`].
///
/// [`VoiceState`]: twilight_model::voice::VoiceState
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CachedVoiceState {
    /// ID of the channel that this user is connected to.
    pub channel_id: Option<ChannelId>,
    /// Whether the user is deafened.
    pub deaf: bool,
    /// ID of the guild that this user is connected in, if there is one.
    pub guild_id: Option<GuildId>,
    /// Whether the user is muted.
    pub mute: bool,
    /// Whether the user has deafened themself.
    pub self_deaf: bool,
    /// Whether the user has muted themself.
    pub self_mute: bool,
    /// Whether the user is streaming via "Go Live".
    pub self_stream: bool,
    /// Session ID.
    pub session_id: String,
    /// Whether this user is muted by the current user.
    pub suppress: bool,
    /// Voice connection token.
    pub token: Option<String>,
    /// ID of the user.
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
