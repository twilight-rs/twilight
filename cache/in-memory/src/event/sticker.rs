use crate::{
    config::ResourceType, model::CachedSticker, GuildResource, InMemoryCache, UpdateCache,
};
use std::borrow::Cow;
use twilight_model::{
    channel::message::sticker::Sticker,
    gateway::payload::incoming::GuildStickersUpdate,
    id::{marker::GuildMarker, Id},
};

impl InMemoryCache {
    pub(crate) fn cache_stickers(&self, guild_id: Id<GuildMarker>, stickers: Vec<Sticker>) {
        if let Some(mut guild_stickers) = self.guild_stickers.get_mut(&guild_id) {
            let incoming: Vec<_> = stickers.iter().map(|s| s.id).collect();

            let removal_filter: Vec<_> = guild_stickers
                .iter()
                .copied()
                .filter(|s| !incoming.contains(s))
                .collect();

            for to_remove in &removal_filter {
                guild_stickers.remove(to_remove);
            }

            for to_remove in &removal_filter {
                self.stickers.remove(to_remove);
            }
        }

        for sticker in stickers {
            self.cache_sticker(guild_id, sticker);
        }
    }

    pub(crate) fn cache_sticker(&self, guild_id: Id<GuildMarker>, sticker: Sticker) {
        match self.stickers.get(&sticker.id) {
            Some(cached_sticker) if cached_sticker.value == sticker => return,
            Some(_) | None => {}
        }

        if let Some(user) = sticker.user.clone() {
            self.cache_user(Cow::Owned(user), Some(guild_id));
        }

        let sticker_id = sticker.id;
        let cached = CachedSticker::from_model(sticker);

        self.stickers.insert(
            cached.id,
            GuildResource {
                guild_id,
                value: cached,
            },
        );

        self.guild_stickers
            .entry(guild_id)
            .or_default()
            .insert(sticker_id);
    }
}

impl UpdateCache for GuildStickersUpdate {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::STICKER) {
            return;
        }

        cache.cache_stickers(self.guild_id, self.stickers.clone());
    }
}
