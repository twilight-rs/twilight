use twilight_cache_inmemory::CacheableSticker;
use twilight_model::{
    channel::message::Sticker,
    id::{marker::StickerMarker, Id},
};

#[derive(Clone, Debug, PartialEq)]
pub struct MinimalCachedSticker {
    pub id: Id<StickerMarker>,
}

impl From<Sticker> for MinimalCachedSticker {
    fn from(sticker: Sticker) -> Self {
        Self { id: sticker.id }
    }
}

impl PartialEq<Sticker> for MinimalCachedSticker {
    fn eq(&self, other: &Sticker) -> bool {
        self.id == other.id
    }
}

impl CacheableSticker for MinimalCachedSticker {
    fn id(&self) -> Id<StickerMarker> {
        self.id
    }
}
