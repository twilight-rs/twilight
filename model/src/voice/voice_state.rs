use crate::{
    guild::Member,
    id::{ChannelId, GuildId, UserId},
};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct VoiceState {
    pub channel_id: Option<ChannelId>,
    pub deaf: bool,
    pub guild_id: Option<GuildId>,
    pub member: Option<Member>,
    pub mute: bool,
    pub self_deaf: bool,
    pub self_mute: bool,
    /// Whether this user is streaming via "Go Live".
    #[cfg_attr(feature = "serde-support", serde(default))]
    pub self_stream: bool,
    pub session_id: String,
    pub suppress: bool,
    pub token: Option<String>,
    pub user_id: UserId,
}

#[cfg(feature = "serde-support")]
mod serde_support {
    use super::VoiceState;
    use crate::id::UserId;
    use serde_mappable_seq::Key;

    impl Key<'_, UserId> for VoiceState {
        fn key(&self) -> UserId {
            self.user_id
        }
    }
}
