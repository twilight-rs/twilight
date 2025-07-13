use crate::{traits::CacheableModels, InMemoryCache, UpdateCache};
use twilight_model::gateway::payload::incoming::GuildSoundboardSoundCreate;

impl<CacheModels: CacheableModels> UpdateCache<CacheModels> for GuildSoundboardSoundCreate {}

impl<CacheModels: CacheableModels> InMemoryCache<CacheModels> {}
