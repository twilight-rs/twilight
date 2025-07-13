use twilight_cache_inmemory::traits::CacheableSoundboardSound;

#[derive(Clone, Debug, PartialEq)]
pub struct MinimalCachedSoundboardSound;

impl CacheableSoundboardSound for MinimalCachedSoundboardSound {}
