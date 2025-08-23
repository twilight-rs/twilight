use twilight_cache_inmemory::CacheableVoiceState;
use twilight_model::{
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker},
    },
    voice::VoiceState,
};

#[derive(Clone, Debug, PartialEq)]
pub struct MinimalCachedVoiceState {
    pub channel_id: Id<ChannelMarker>,
}

impl From<(Id<ChannelMarker>, Id<GuildMarker>, VoiceState)> for MinimalCachedVoiceState {
    fn from((channel_id, _, _): (Id<ChannelMarker>, Id<GuildMarker>, VoiceState)) -> Self {
        Self { channel_id }
    }
}

impl PartialEq<VoiceState> for MinimalCachedVoiceState {
    fn eq(&self, other: &VoiceState) -> bool {
        other.channel_id == Some(self.channel_id)
    }
}

impl CacheableVoiceState for MinimalCachedVoiceState {
    fn channel_id(&self) -> Id<ChannelMarker> {
        self.channel_id
    }
}
