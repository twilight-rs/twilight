use twilight_cache_inmemory::CacheableEmoji;
use twilight_model::{
    guild::Emoji,
    id::{Id, marker::EmojiMarker},
};

#[derive(Clone, Debug, PartialEq)]
pub struct MinimalCachedEmoji {
    pub id: Id<EmojiMarker>,
}

impl From<Emoji> for MinimalCachedEmoji {
    fn from(value: Emoji) -> Self {
        Self { id: value.id }
    }
}

impl PartialEq<Emoji> for MinimalCachedEmoji {
    fn eq(&self, other: &Emoji) -> bool {
        self.id == other.id
    }
}

impl CacheableEmoji for MinimalCachedEmoji {}
