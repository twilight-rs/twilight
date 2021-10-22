use crate::{
    config::ResourceType, model::CachedSticker, GuildResource, InMemoryCache, UpdateCache,
};
use std::borrow::Cow;
use twilight_model::{
    channel::message::sticker::{Sticker, StickerId},
    gateway::payload::incoming::GuildStickersUpdate,
    id::GuildId,
};

impl InMemoryCache {
    pub(crate) fn cache_stickers(&self, guild_id: GuildId, stickers: Vec<Sticker>) {
        if let Some(mut guild_stickers) = self.guild_stickers.get_mut(&guild_id) {
            let incoming: Vec<StickerId> = stickers.iter().map(|s| s.id).collect();

            let removal_filter: Vec<StickerId> = guild_stickers
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

    pub(crate) fn cache_sticker(&self, guild_id: GuildId, sticker: Sticker) {
        match self.stickers.get(&sticker.id) {
            Some(cached_sticker) if cached_sticker.value == sticker => return,
            Some(_) | None => {}
        }

        let user_id = sticker.user.as_ref().map(|user| user.id);

        if let Some(user) = sticker.user {
            self.cache_user(Cow::Owned(user), Some(guild_id));
        }

        let cached = CachedSticker {
            available: sticker.available,
            description: sticker.description.unwrap_or_default(),
            format_type: sticker.format_type,
            guild_id: sticker.guild_id,
            id: sticker.id,
            kind: sticker.kind,
            name: sticker.name,
            pack_id: sticker.pack_id,
            sort_value: sticker.sort_value,
            tags: sticker.tags,
            user_id,
        };

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
            .insert(sticker.id);
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
