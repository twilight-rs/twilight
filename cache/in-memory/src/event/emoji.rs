use crate::{config::ResourceType, model::CachedEmoji, GuildResource, InMemoryCache, UpdateCache};
use std::borrow::Cow;
use twilight_model::{
    gateway::payload::incoming::GuildEmojisUpdate,
    guild::Emoji,
    id::{EmojiId, GuildId},
};

impl InMemoryCache {
    pub(crate) fn cache_emojis(&self, guild_id: GuildId, emojis: Vec<Emoji>) {
        if let Some(mut guild_emojis) = self.guild_emojis.get_mut(&guild_id) {
            let incoming: Vec<EmojiId> = emojis.iter().map(|e| e.id).collect();

            let removal_filter: Vec<EmojiId> = guild_emojis
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

    pub(crate) fn cache_emoji(&self, guild_id: GuildId, emoji: Emoji) {
        match self.emojis.get(&emoji.id) {
            Some(cached_emoji) if cached_emoji.value == emoji => return,
            Some(_) | None => {}
        }

        let user_id = emoji.user.as_ref().map(|user| user.id);

        if let Some(user) = emoji.user {
            self.cache_user(Cow::Owned(user), Some(guild_id));
        }

        let cached = CachedEmoji {
            id: emoji.id,
            animated: emoji.animated,
            name: emoji.name,
            managed: emoji.managed,
            require_colons: emoji.require_colons,
            roles: emoji.roles,
            user_id,
            available: emoji.available,
        };

        self.emojis.insert(
            cached.id,
            GuildResource {
                guild_id,
                value: cached,
            },
        );

        self.guild_emojis
            .entry(guild_id)
            .or_default()
            .insert(emoji.id);
    }
}

impl UpdateCache for GuildEmojisUpdate {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::EMOJI) {
            return;
        }

        cache.cache_emojis(self.guild_id, self.emojis.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;
    use twilight_model::{id::UserId, user::User};

    #[test]
    fn test_cache_emoji() {
        let cache = InMemoryCache::new();

        // The user to do some of the inserts
        fn user_mod(id: EmojiId) -> Option<User> {
            if id.get() % 2 == 0 {
                // Only use user for half
                Some(test::user(UserId::new(1).expect("non zero")))
            } else {
                None
            }
        }

        // Single inserts
        {
            let guild_1_emoji_ids = (1..=10)
                .map(|n| EmojiId::new(n).expect("non zero"))
                .collect::<Vec<_>>();
            let guild_1_emoji = guild_1_emoji_ids
                .iter()
                .copied()
                .map(|id| test::emoji(id, user_mod(id)))
                .collect::<Vec<_>>();

            for emoji in guild_1_emoji {
                cache.cache_emoji(GuildId::new(1).expect("non zero"), emoji);
            }

            for id in guild_1_emoji_ids.iter().cloned() {
                let global_emoji = cache.emoji(id);
                assert!(global_emoji.is_some());
            }

            // Ensure the emoji has been added to the per-guild lookup map to prevent
            // issues like #551 from returning
            let guild_emojis = cache.guild_emojis(GuildId::new(1).expect("non zero"));
            assert!(guild_emojis.is_some());
            let guild_emojis = guild_emojis.unwrap();

            assert_eq!(guild_1_emoji_ids.len(), guild_emojis.len());
            assert!(guild_1_emoji_ids.iter().all(|id| guild_emojis.contains(id)));
        }

        // Bulk inserts
        {
            let guild_2_emoji_ids = (11..=20)
                .map(|n| EmojiId::new(n).expect("non zero"))
                .collect::<Vec<_>>();
            let guild_2_emojis = guild_2_emoji_ids
                .iter()
                .copied()
                .map(|id| test::emoji(id, user_mod(id)))
                .collect::<Vec<_>>();
            cache.cache_emojis(GuildId::new(2).expect("non zero"), guild_2_emojis);

            for id in guild_2_emoji_ids.iter().cloned() {
                let global_emoji = cache.emoji(id);
                assert!(global_emoji.is_some());
            }

            let guild_emojis = cache.guild_emojis(GuildId::new(2).expect("non zero"));

            assert!(guild_emojis.is_some());
            let guild_emojis = guild_emojis.unwrap();
            assert_eq!(guild_2_emoji_ids.len(), guild_emojis.len());
            assert!(guild_2_emoji_ids.iter().all(|id| guild_emojis.contains(id)));
        }
    }

    #[test]
    fn test_emoji_removal() {
        let cache = InMemoryCache::new();

        let guild_id = GuildId::new(1).expect("non zero");

        let emote = test::emoji(EmojiId::new(1).expect("non zero"), None);
        let emote_2 = test::emoji(EmojiId::new(2).expect("non zero"), None);
        let emote_3 = test::emoji(EmojiId::new(3).expect("non zero"), None);

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

        let emote_4 = test::emoji(EmojiId::new(4).expect("non zero"), None);

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
