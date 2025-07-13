use std::ops::Deref;
use crate::traits::CacheableSoundboardSound;
use crate::{traits::CacheableModels, InMemoryCache, ResourceType, UpdateCache};
use twilight_model::{
    gateway::payload::incoming::{GuildSoundboardSoundCreate, GuildSoundboardSoundDelete},
    guild::SoundboardSound,
    id::{marker::SoundboardSoundMarker, Id},
};

impl<CacheModels: CacheableModels> UpdateCache<CacheModels> for GuildSoundboardSoundCreate {
    fn update(&self, cache: &InMemoryCache<CacheModels>) {
        if !cache.wants(ResourceType::SOUNDBOARD_SOUNDS) {
            return;
        }

        cache.cache_soundboard_sound(self.0.clone());
    }
}

impl<CacheModels: CacheableModels> UpdateCache<CacheModels> for GuildSoundboardSoundDelete {
    fn update(&self, cache: &InMemoryCache<CacheModels>) {
        cache.delete_soundboard_sound(self.sound_id);
    }
}

impl<CacheModels: CacheableModels> InMemoryCache<CacheModels> {
    pub(crate) fn cache_soundboard_sound(&self, soundboard_sound: SoundboardSound) {
        if let Some(guild_id) = soundboard_sound.guild_id {
            self.guild_soundboard_sounds
                .entry(guild_id)
                .or_default()
                .insert(soundboard_sound.sound_id);
        }

        self.soundboard_sound.insert(
            soundboard_sound.sound_id,
            CacheModels::SoundboardSound::from(soundboard_sound),
        );
    }

    pub(crate) fn delete_soundboard_sound(&self, sound_id: Id<SoundboardSoundMarker>) {
        let Some((_, sound)) = self.soundboard_sound.remove(&sound_id) else {
            return;
        };
        let Some(guild_id) = sound.guild_id() else {
            return;
        };
        let Some(mut sounds) = self.guild_soundboard_sounds.get_mut(&guild_id) else {
            return;
        };
        sounds.remove(&sound_id);
    }
}
