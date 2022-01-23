use serde::Serialize;
use twilight_model::{
    id::{
        marker::{ChannelMarker, GuildMarker, UserMarker},
        Id,
    },
    voice::VoiceState,
};

/// Represents a cached [`VoiceState`].
///
/// [`VoiceState`]: twilight_model::voice::VoiceState
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CachedVoiceState {
    channel_id: Option<Id<ChannelMarker>>,
    deaf: bool,
    guild_id: Option<Id<GuildMarker>>,
    mute: bool,
    self_deaf: bool,
    self_mute: bool,
    self_stream: bool,
    session_id: String,
    suppress: bool,
    token: Option<String>,
    user_id: Id<UserMarker>,
}

impl CachedVoiceState {
    /// ID of the channel that this user is connected to.
    pub const fn channel_id(&self) -> Option<Id<ChannelMarker>> {
        self.channel_id
    }

    /// Whether the user is deafened.
    pub const fn deaf(&self) -> bool {
        self.deaf
    }

    /// ID of the guild that this user is connected in, if there is one.
    pub const fn guild_id(&self) -> Option<Id<GuildMarker>> {
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
    pub const fn user_id(&self) -> Id<UserMarker> {
        self.user_id
    }
}

impl From<VoiceState> for CachedVoiceState {
    fn from(voice_state: VoiceState) -> Self {
        let VoiceState {
            channel_id,
            deaf,
            guild_id,
            member: _,
            mute,
            self_deaf,
            self_mute,
            self_stream,
            self_video: _,
            session_id,
            suppress,
            token,
            user_id,
            request_to_speak_timestamp: _,
        } = voice_state;

        Self {
            channel_id,
            deaf,
            guild_id,
            mute,
            self_deaf,
            self_mute,
            self_stream,
            session_id,
            suppress,
            token,
            user_id,
        }
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

#[cfg(test)]
mod tests {
    use super::CachedVoiceState;
    use twilight_model::{id::Id, voice::VoiceState};

    fn voice_state() -> VoiceState {
        VoiceState {
            channel_id: Some(Id::new(1)),
            deaf: false,
            guild_id: Some(Id::new(2)),
            member: None,
            mute: true,
            self_deaf: false,
            self_mute: true,
            self_stream: false,
            self_video: true,
            session_id: "ba8bd70ac7239ffc710e2fc8db52f240".to_owned(),
            suppress: false,
            token: None,
            user_id: Id::new(3),
            request_to_speak_timestamp: None,
        }
    }

    #[test]
    fn test_eq() {
        let voice_state = voice_state();
        let cached = CachedVoiceState::from(voice_state.clone());

        assert_eq!(cached, voice_state);
    }

    #[test]
    fn test_getters() {
        let voice_state = voice_state();
        let cached = CachedVoiceState::from(voice_state.clone());

        assert_eq!(cached.channel_id(), voice_state.channel_id);
        assert_eq!(cached.deaf(), voice_state.deaf);
        assert_eq!(cached.guild_id(), voice_state.guild_id);
        assert_eq!(cached.mute(), voice_state.mute);
        assert_eq!(cached.self_deaf(), voice_state.self_deaf);
        assert_eq!(cached.self_mute(), voice_state.self_mute);
        assert_eq!(cached.self_stream(), voice_state.self_stream);
        assert_eq!(cached.session_id(), voice_state.session_id);
        assert_eq!(cached.suppress(), voice_state.suppress);
        assert_eq!(cached.token(), voice_state.token.as_deref());
        assert_eq!(cached.user_id(), voice_state.user_id);
    }
}
