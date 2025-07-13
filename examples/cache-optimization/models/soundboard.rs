use twilight_cache_inmemory::traits::CacheableSoundboardSound;
use twilight_model::{
    guild::SoundboardSound,
    id::{marker::SoundboardSoundMarker, Id},
};

#[derive(Clone, Debug, PartialEq)]
pub struct MinimalCachedSoundboardSound {
    pub sound_id: Id<SoundboardSoundMarker>,
}

impl CacheableSoundboardSound for MinimalCachedSoundboardSound {}

impl From<SoundboardSound> for MinimalCachedSoundboardSound {
    fn from(_: SoundboardSound) -> Self {
        Self
    }
}

impl PartialEq<SoundboardSound> for MinimalCachedSoundboardSound {
    fn eq(&self, other: &SoundboardSound) -> bool {
        self.sound_id == other.sound_id
    }
}
