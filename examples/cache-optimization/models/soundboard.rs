use twilight_cache_inmemory::traits::CacheableSoundboardSound;
use twilight_model::{
    guild::SoundboardSound,
    id::{
        marker::{GuildMarker, SoundboardSoundMarker},
        Id,
    },
};

#[derive(Clone, Debug, PartialEq)]
pub struct MinimalCachedSoundboardSound {
    pub guild_id: Option<Id<GuildMarker>>,
    pub sound_id: Id<SoundboardSoundMarker>,
}

impl CacheableSoundboardSound for MinimalCachedSoundboardSound {
    fn guild_id(&self) -> Option<Id<GuildMarker>> {
        self.guild_id
    }
}

impl From<SoundboardSound> for MinimalCachedSoundboardSound {
    fn from(sound: SoundboardSound) -> Self {
        Self {
            guild_id: sound.guild_id,
            sound_id: sound.sound_id,
        }
    }
}

impl PartialEq<SoundboardSound> for MinimalCachedSoundboardSound {
    fn eq(&self, other: &SoundboardSound) -> bool {
        self.sound_id == other.sound_id
    }
}
