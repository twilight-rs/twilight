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
    channel_id: Option<ChannelId>,
    deaf: bool,
    guild_id: Option<GuildId>,
    mute: bool,
    self_deaf: bool,
    self_mute: bool,
    self_stream: bool,
    session_id: String,
    suppress: bool,
    token: Option<String>,
    user_id: UserId,
}

impl CachedVoiceState {
    /// ID of the channel that this user is connected to.
    pub const fn channel_id(&self) -> Option<ChannelId> {
        self.channel_id
    }

    /// Whether the user is deafened.
    pub const fn deaf(&self) -> bool {
        self.deaf
    }

    /// ID of the guild that this user is connected in, if there is one.
    pub const fn guild_id(&self) -> Option<GuildId> {
        self.guild_id
    }

    /// Whether the user is muted.
    pub const fn mute(&self) -> bool {
        self.mute
    }

    /// Whether the user has deafened themself.
    pub const fn self_deaf(&self) -> bool {
        self.self_deaf
    }

    /// Whether the user has muted themself.
    pub const fn self_mute(&self) -> bool {
        self.self_mute
    }

    /// Whether the user is streaming via "Go Live".
    pub const fn self_stream(&self) -> bool {
        self.self_stream
    }

    /// Session ID.
    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    /// Whether this user is muted by the current user.
    pub const fn suppress(&self) -> bool {
        self.suppress
    }

    /// Voice connection token.
    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    /// ID of the user.
    pub const fn user_id(&self) -> UserId {
        self.user_id
    }
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
