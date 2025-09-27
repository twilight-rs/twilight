use serde::{Deserialize, Serialize};

use crate::{
    guild::Emoji,
    id::{
        marker::{AnimationMarker, ChannelMarker, GuildMarker, SoundboardSoundMarker, UserMarker},
        Id,
    },
};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct VoiceChannelEffectSend {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation_id: Option<Id<AnimationMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation_type: Option<VoiceChannelEffectAnimationType>,
    pub channel_id: Id<ChannelMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<Emoji>,
    pub guild_id: Id<GuildMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound_id: Option<Id<SoundboardSoundMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound_volume: Option<f64>,
    pub user_id: Id<UserMarker>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum VoiceChannelEffectAnimationType {
    Premium,
    Basic,
}
