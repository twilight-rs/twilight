use crate::{
    config::ResourceType, model::CachedSticker, GuildResource, InMemoryCache, UpdateCache,
};
use std::{borrow::Cow, collections::HashSet};
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

    const GUILD_ID: Id<GuildMarker> = Id::new(1);
    const STICKER_ONE_ID: Id<StickerMarker> = Id::new(2);
    const STICKER_TWO_ID: Id<StickerMarker> = Id::new(3);

    fn cache_with_stickers() -> InMemoryCache {
        let cache = test::cache();
        let one = test::sticker(STICKER_ONE_ID, GUILD_ID);
        let two = test::sticker(STICKER_TWO_ID, GUILD_ID);
        cache.cache_stickers(GUILD_ID, Vec::from([one, two]));

        cache
    }

    /// Test that caching stickers correctly inserts the stickers into the
    /// sticker cache by testing their identity, and that the map of a guild's
    /// sticker associated IDs contains all stickers.
    #[test]
    fn test_cache_stickers() {
        let cache = cache_with_stickers();
        assert_eq!(cache.stickers.len(), 2);
        let one = test::sticker(STICKER_ONE_ID, GUILD_ID);
        let two = test::sticker(STICKER_TWO_ID, GUILD_ID);
        assert!(cache.stickers.get(&STICKER_ONE_ID).map(|r| r.id == STICKER_ONE_ID).unwrap_or_default());
        assert!(cache.stickers.get(&STICKER_TWO_ID).map(|r| r.id == STICKER_TWO_ID).unwrap_or_default());

        let guild_stickers = cache.guild_stickers
            .get(&GUILD_ID)
            .expect("cache has stickers for guild");
        assert_eq!(guild_stickers.len(), 2);
        assert!(guild_stickers.contains(&one.id));
        assert!(guild_stickers.contains(&two.id));
    }

    /// Test that caching an updated list of a guild's stickers removes one of
    /// the existing stickers if not in the updated list, meaning the sticker no
    /// longer exists.
    ///
    /// For example, if two stickers for a guild named "foo" and "bar" are
    /// cached and a new list of stickers with only "foo" is cached, then "bar"
    /// will be removed.
    #[test]
    fn test_cache_stickers_removal() {
        let cache = cache_with_stickers();
        let one = test::sticker(STICKER_ONE_ID, GUILD_ID);
        cache.cache_stickers(GUILD_ID, Vec::from([one]));
        assert_eq!(cache.stickers.len(), 1);
        assert!(cache.stickers.get(&STICKER_ONE_ID).map(|r| r.id == STICKER_ONE_ID).unwrap_or_default());
        let guild_stickers = cache.guild_stickers
            .get(&GUILD_ID)
            .expect("cache has stickers for guild");
        assert_eq!(guild_stickers.len(), 1);
        assert!(guild_stickers.contains(&STICKER_ONE_ID));
    }
}
