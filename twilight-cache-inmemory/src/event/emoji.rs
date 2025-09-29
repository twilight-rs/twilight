use crate::{CacheableModels, GuildResource, InMemoryCache, UpdateCache, config::ResourceType};
use std::borrow::Cow;
use twilight_model::{
    gateway::payload::incoming::GuildEmojisUpdate,
    guild::Emoji,
    id::{Id, marker::GuildMarker},
};

impl<CacheModels: CacheableModels> InMemoryCache<CacheModels> {
    pub(crate) fn cache_emojis(&self, guild_id: Id<GuildMarker>, emojis: Vec<Emoji>) {
        if let Some(mut guild_emojis) = self.guild_emojis.get_mut(&guild_id) {
            let incoming: Vec<_> = emojis.iter().map(|e| e.id).collect();

            let removal_filter: Vec<_> = guild_emojis
                .iter()
                .copied()
                .filter(|e| !incoming.contains(e))
                .collect();

            for to_remove in &removal_filter {
                guild_emojis.remove(to_remove);
            }

            for to_remove in &removal_filter {
                self.emojis.remove(to_remove);
            }
        }

        for emoji in emojis {
            self.cache_emoji(guild_id, emoji);
        }
    }

    pub(crate) fn cache_emoji(&self, guild_id: Id<GuildMarker>, emoji: Emoji) {
        if let Some(cached_emoji) = self.emojis.get(&emoji.id)
            && cached_emoji.value == emoji
        {
            return;
        }

        if let Some(user) = emoji.user.as_ref() {
            self.cache_user(Cow::Borrowed(user), Some(guild_id));
        }

        let emoji_id = emoji.id;
        let cached = CacheModels::Emoji::from(emoji);

        self.emojis.insert(
            emoji_id,
            GuildResource {
                guild_id,
                value: cached,
            },
        );

        self.guild_emojis
            .entry(guild_id)
            .or_default()
            .insert(emoji_id);
    }
}

impl<CacheModels: CacheableModels> UpdateCache<CacheModels> for GuildEmojisUpdate {
    fn update(&self, cache: &InMemoryCache<CacheModels>) {
        if !cache.wants(ResourceType::EMOJI) {
            return;
        }

        cache.cache_emojis(self.guild_id, self.emojis.clone());
    }
}

#[cfg(test)]
mod tests {
    use crate::{DefaultInMemoryCache, test};
    use twilight_model::{
        gateway::payload::incoming::GuildEmojisUpdate,
        id::{Id, marker::EmojiMarker},
        user::User,
    };

    #[test]
    fn cache_emoji() {
        // The user to do some of the inserts
        fn user_mod(id: Id<EmojiMarker>) -> Option<User> {
            if id.get().is_multiple_of(2) {
                // Only use user for half
                Some(test::user(Id::new(1)))
            } else {
                None
            }
        }

        let cache = DefaultInMemoryCache::new();

        // Single inserts
        {
            let guild_1_emoji_ids = (1..=10).map(Id::new).collect::<Vec<_>>();
            let guild_1_emoji = guild_1_emoji_ids
                .iter()
                .copied()
                .map(|id| test::emoji(id, user_mod(id)))
                .collect::<Vec<_>>();

            for emoji in guild_1_emoji {
                cache.cache_emoji(Id::new(1), emoji);
            }

            for id in guild_1_emoji_ids.iter().copied() {
                let global_emoji = cache.emoji(id);
                assert!(global_emoji.is_some());
            }

            // Ensure the emoji has been added to the per-guild lookup map to prevent
            // issues like #551 from returning
            let guild_emojis = cache.guild_emojis(Id::new(1));
            assert!(guild_emojis.is_some());
            let guild_emojis = guild_emojis.unwrap();

            assert_eq!(guild_1_emoji_ids.len(), guild_emojis.len());
            assert!(guild_1_emoji_ids.iter().all(|id| guild_emojis.contains(id)));
        }

        // Bulk inserts
        {
            let guild_2_emoji_ids = (11..=20).map(Id::new).collect::<Vec<_>>();
            let guild_2_emojis = guild_2_emoji_ids
                .iter()
                .copied()
                .map(|id| test::emoji(id, user_mod(id)))
                .collect::<Vec<_>>();
            cache.cache_emojis(Id::new(2), guild_2_emojis);

            for id in guild_2_emoji_ids.iter().copied() {
                let global_emoji = cache.emoji(id);
                assert!(global_emoji.is_some());
            }

            let guild_emojis = cache.guild_emojis(Id::new(2));

            assert!(guild_emojis.is_some());
            let guild_emojis = guild_emojis.unwrap();
            assert_eq!(guild_2_emoji_ids.len(), guild_emojis.len());
            assert!(guild_2_emoji_ids.iter().all(|id| guild_emojis.contains(id)));
        }
    }

    #[test]
    fn emoji_removal() {
        let cache = DefaultInMemoryCache::new();

        let guild_id = Id::new(1);

        let emote = test::emoji(Id::new(1), None);
        let emote_2 = test::emoji(Id::new(2), None);
        let emote_3 = test::emoji(Id::new(3), None);

        cache.cache_emoji(guild_id, emote.clone());
        cache.cache_emoji(guild_id, emote_2.clone());
        cache.cache_emoji(guild_id, emote_3.clone());

        cache.update(&GuildEmojisUpdate {
            emojis: vec![emote.clone(), emote_3.clone()],
            guild_id,
        });

        assert_eq!(cache.emojis.len(), 2);
        assert_eq!(cache.guild_emojis.get(&guild_id).unwrap().len(), 2);
        assert!(cache.emoji(emote.id).is_some());
        assert!(cache.emoji(emote_2.id).is_none());
        assert!(cache.emoji(emote_3.id).is_some());

        cache.update(&GuildEmojisUpdate {
            emojis: vec![emote.clone()],
            guild_id,
        });

        assert_eq!(cache.emojis.len(), 1);
        assert_eq!(cache.guild_emojis.get(&guild_id).unwrap().len(), 1);
        assert!(cache.emoji(emote.id).is_some());
        assert!(cache.emoji(emote_2.id).is_none());

        let emote_4 = test::emoji(Id::new(4), None);

        cache.update(&GuildEmojisUpdate {
            emojis: vec![emote_4.clone()],
            guild_id,
        });

        assert_eq!(cache.emojis.len(), 1);
        assert_eq!(cache.guild_emojis.get(&guild_id).unwrap().len(), 1);
        assert!(cache.emoji(emote_4.id).is_some());
        assert!(cache.emoji(emote.id).is_none());

        cache.update(&GuildEmojisUpdate {
            emojis: vec![],
            guild_id,
        });

        assert!(cache.emojis.is_empty());
        assert!(cache.guild_emojis.get(&guild_id).unwrap().is_empty());
    }
}
