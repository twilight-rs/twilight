use crate::{traits::CacheableModels, InMemoryCache, ResourceType, UpdateCache};
use twilight_model::{gateway::payload::incoming::GuildSoundboardSoundCreate, guild::SoundboardSound};

impl<CacheModels: CacheableModels> UpdateCache<CacheModels> for GuildSoundboardSoundCreate {
    fn update(&self, cache: &InMemoryCache<CacheModels>) {
        if !cache.wants(ResourceType::SOUNDBOARD_SOUNDS) {
            return;
        }

        cache.cache_soundboard_sound(self.0.clone())
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

        self.soundboard_sound.insert(soundboard_sound.sound_id, soundboard_sound);
    }
}
