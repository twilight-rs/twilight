use crate::{
    config::ResourceType, model::CachedSticker, GuildResource, InMemoryCache, UpdateCache,
};
use std::borrow::Cow;
use std::collections::HashSet;
use twilight_model::{
    channel::message::sticker::Sticker,
    gateway::payload::incoming::GuildStickersUpdate,
    id::{marker::GuildMarker, Id},
};

impl InMemoryCache {
    pub(crate) fn cache_stickers(&self, guild_id: Id<GuildMarker>, stickers: Vec<Sticker>) {
        if let Some(mut guild_stickers) = self.guild_stickers.get_mut(&guild_id) {
            let incoming_sticker_ids = stickers.iter()
                .map(|sticker| sticker.id)
                .collect::<HashSet<_>>();

            // Iterate over the set of a guild's stickers, retaining only the
            // existing stickers that are still present in the updated list of
            // stickers.
            //
            // If one is not, then we remove it both from the guild's set of
            // stickers and the sticker cache.
            guild_stickers.retain(|sticker_id| {
                let retain = incoming_sticker_ids.contains(sticker_id);

                if !retain {
                    self.stickers.remove(sticker_id);
                }

                retain
            });
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

#[cfg(test)]
mod tests {
    use crate::{InMemoryCache, test};
    use twilight_model::id::{marker::{GuildMarker, StickerMarker}, Id};

    #[test]
    fn test_sticker_removal() {
        const GUILD_ID: Id<GuildMarker> = Id::new(1);
        const STICKER_ONE_ID: Id<StickerMarker> = Id::new(2);
        const STICKER_TWO_ID: Id<StickerMarker> = Id::new(3);

        fn guild_sticker_count(cache: &InMemoryCache) -> Option<usize> {
            cache.
                guild_stickers
                .get(&GUILD_ID)
                .map(|guild_stickers| guild_stickers.len())
        }

        let cache = test::cache();
        let one = test::sticker(STICKER_ONE_ID, GUILD_ID);
        let two = test::sticker(STICKER_TWO_ID, GUILD_ID);
        cache.cache_sticker(GUILD_ID, one.clone());
        cache.cache_sticker(GUILD_ID, two);
        assert_eq!(2, cache.stickers.len());
        assert_eq!(Some(2), guild_sticker_count(&cache));

        cache.cache_stickers(GUILD_ID, Vec::from([one]));
        assert_eq!(1, cache.stickers.len());
        assert_eq!(Some(1), guild_sticker_count(&cache));
    }
}
