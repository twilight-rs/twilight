use serde::Serialize;
use twilight_model::{
    datetime::Timestamp,
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
    channel_id: Id<ChannelMarker>,
    deaf: bool,
    guild_id: Id<GuildMarker>,
    mute: bool,
    request_to_speak_timestamp: Option<Timestamp>,
    self_deaf: bool,
    self_mute: bool,
    self_stream: bool,
    self_video: bool,
    session_id: String,
    suppress: bool,
    token: Option<String>,
    user_id: Id<UserMarker>,
}

impl CachedVoiceState {
    /// ID of the channel that this user is connected to.
    pub const fn channel_id(&self) -> Id<ChannelMarker> {
        self.channel_id
    }

    /// Whether the user is deafened.
    pub const fn deaf(&self) -> bool {
        self.deaf
    }

    /// ID of the guild that this user is connected in, if there is one.
    pub const fn guild_id(&self) -> Id<GuildMarker> {
        self.guild_id
    }

    /// Whether the user is muted.
    pub const fn mute(&self) -> bool {
        self.mute
    }

    /// Timestamp of when the user requested to speak.
    pub const fn request_to_speak_timestamp(&self) -> Option<Timestamp> {
        self.request_to_speak_timestamp
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

    /// Whether the user's camera is enabled.
    pub const fn self_video(&self) -> bool {
        self.self_video
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

    /// Construct a cached voice state from its [`twilight_model`] form.
    #[allow(clippy::missing_const_for_fn)]
    pub(crate) fn from_model(
        channel_id: Id<ChannelMarker>,
        guild_id: Id<GuildMarker>,
        voice_state: VoiceState,
    ) -> Self {
        // Reasons for dropping fields:
        //
        // - `channel_id`: provided as a function parameter
        // - `guild_id`: provided as a function parameter
        // - `member`: we have the user's ID from the `user_id` field
        let VoiceState {
            channel_id: _,
            deaf,
            guild_id: _,
            member: _,
            mute,
            self_deaf,
            self_mute,
            self_stream,
            self_video,
            session_id,
            suppress,
            token,
            user_id,
            request_to_speak_timestamp,
        } = voice_state;

        Self {
            channel_id,
            deaf,
            guild_id,
            mute,
            request_to_speak_timestamp,
            self_deaf,
            self_mute,
            self_stream,
            self_video,
            session_id,
            suppress,
            token,
            user_id,
        }
    }
}

impl PartialEq<VoiceState> for CachedVoiceState {
    fn eq(&self, other: &VoiceState) -> bool {
        Some(self.channel_id) == other.channel_id
            && self.deaf == other.deaf
            && Some(self.guild_id) == other.guild_id
            && self.mute == other.mute
            && self.request_to_speak_timestamp == other.request_to_speak_timestamp
            && self.self_deaf == other.self_deaf
            && self.self_mute == other.self_mute
            && self.self_stream == other.self_stream
            && self.self_video == other.self_video
            && self.session_id == other.session_id
            && self.suppress == other.suppress
            && self.token == other.token
            && self.user_id == other.user_id
    }
}

#[cfg(test)]
mod tests {
    use super::CachedVoiceState;
    use crate::test;
    use serde::Serialize;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::fmt::Debug;
    use twilight_model::{
        id::{
            marker::{ChannelMarker, GuildMarker, UserMarker},
            Id,
        },
        voice::VoiceState,
    };

    assert_fields!(
        CachedVoiceState: channel_id,
        deaf,
        guild_id,
        mute,
        request_to_speak_timestamp,
        self_deaf,
        self_mute,
        self_stream,
        self_video,
        session_id,
        suppress,
        token,
        user_id
    );
    assert_impl_all!(
        CachedVoiceState: Clone,
        Debug,
        Eq,
        PartialEq,
        PartialEq<VoiceState>,
        Serialize,
    );

    const CHANNEL_ID: Id<ChannelMarker> = Id::new(1);
    const GUILD_ID: Id<GuildMarker> = Id::new(2);
    const USER_ID: Id<UserMarker> = Id::new(3);

    #[test]
    fn test_eq() {
        let voice_state = test::voice_state(GUILD_ID, Some(CHANNEL_ID), USER_ID);
        let cached = CachedVoiceState::from_model(CHANNEL_ID, GUILD_ID, voice_state.clone());

        assert_eq!(cached, voice_state);
    }

    #[test]
    fn test_getters() {
        let voice_state = test::voice_state(GUILD_ID, Some(CHANNEL_ID), USER_ID);
        let cached = CachedVoiceState::from_model(CHANNEL_ID, GUILD_ID, voice_state.clone());

        assert_eq!(Some(cached.channel_id()), voice_state.channel_id);
        assert_eq!(cached.deaf(), voice_state.deaf);
        assert_eq!(Some(cached.guild_id()), voice_state.guild_id);
        assert_eq!(cached.mute(), voice_state.mute);
        assert_eq!(
            cached.request_to_speak_timestamp(),
            voice_state.request_to_speak_timestamp
        );
        assert_eq!(cached.self_deaf(), voice_state.self_deaf);
        assert_eq!(cached.self_mute(), voice_state.self_mute);
        assert_eq!(cached.self_stream(), voice_state.self_stream);
        assert_eq!(cached.self_video(), voice_state.self_video);
        assert_eq!(cached.session_id(), voice_state.session_id);
        assert_eq!(cached.suppress(), voice_state.suppress);
        assert_eq!(cached.token(), voice_state.token.as_deref());
        assert_eq!(cached.user_id(), voice_state.user_id);
    }
}
