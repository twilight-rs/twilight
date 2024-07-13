use crate::{
    config::ResourceType,
    traits::{CacheableCurrentUser, CacheableMessage},
    CacheableModels, InMemoryCache, UpdateCache,
};
use twilight_model::{
    channel::message::{EmojiReactionType, Reaction, ReactionCountDetails},
    gateway::payload::incoming::{
        ReactionAdd, ReactionRemove, ReactionRemoveAll, ReactionRemoveEmoji,
    },
};

impl<CacheModels: CacheableModels> UpdateCache<CacheModels> for ReactionAdd {
    fn update(&self, cache: &InMemoryCache<CacheModels>) {
        if !cache.wants(ResourceType::REACTION) {
            return;
        }

        let key = self.0.message_id;

        let Some(mut message) = cache.messages.get_mut(&key) else {
            return;
        };

        if let Some(reaction) = message
            .reactions_mut()
            .iter_mut()
            .find(|r| reactions_eq(&r.emoji, &self.0.emoji))
        {
            if !reaction.me {
                if let Some(current_user) = cache.current_user() {
                    if current_user.id() == self.0.user_id {
                        reaction.me = true;
                    }
                }
            }

            reaction.count += 1;
        } else {
            let me = cache
                .current_user()
                .map(|user| user.id() == self.0.user_id)
                .unwrap_or_default();

            message.add_reaction(Reaction {
                burst_colors: Vec::new(),
                count: 1,
                count_details: ReactionCountDetails {
                    burst: 0,
                    normal: 1,
                },
                emoji: self.0.emoji.clone(),
                me,
                me_burst: false,
            });
        }
    }
}

impl<CacheModels: CacheableModels> UpdateCache<CacheModels> for ReactionRemove {
    fn update(&self, cache: &InMemoryCache<CacheModels>) {
        if !cache.wants(ResourceType::REACTION) {
            return;
        }

        let Some(mut message) = cache.messages.get_mut(&self.0.message_id) else {
            return;
        };

        if let Some(reaction) = message
            .reactions_mut()
            .iter_mut()
            .find(|r| reactions_eq(&r.emoji, &self.0.emoji))
        {
            if reaction.me {
                if let Some(current_user) = cache.current_user() {
                    if current_user.id() == self.0.user_id {
                        reaction.me = false;
                    }
                }
            }

            if reaction.count > 1 {
                reaction.count -= 1;
            } else {
                message.retain_reactions(|e| !(reactions_eq(&e.emoji, &self.0.emoji)));
            }
        }
    }
}

impl<CacheModels: CacheableModels> UpdateCache<CacheModels> for ReactionRemoveAll {
    fn update(&self, cache: &InMemoryCache<CacheModels>) {
        if !cache.wants(ResourceType::REACTION) {
            return;
        }

        let Some(mut message) = cache.messages.get_mut(&self.message_id) else {
            return;
        };

        message.clear_reactions();
    }
}

impl<CacheModels: CacheableModels> UpdateCache<CacheModels> for ReactionRemoveEmoji {
    fn update(&self, cache: &InMemoryCache<CacheModels>) {
        if !cache.wants(ResourceType::REACTION) {
            return;
        }

        let Some(mut message) = cache.messages.get_mut(&self.message_id) else {
            return;
        };

        let maybe_index = message
            .reactions()
            .iter()
            .position(|r| reactions_eq(&r.emoji, &self.emoji));

        if let Some(index) = maybe_index {
            message.remove_reaction(index);
        }
    }
}

fn reactions_eq(a: &EmojiReactionType, b: &EmojiReactionType) -> bool {
    match (a, b) {
        (
            EmojiReactionType::Custom { id: id_a, .. },
            EmojiReactionType::Custom { id: id_b, .. },
        ) => id_a == id_b,
        (
            EmojiReactionType::Unicode { name: name_a },
            EmojiReactionType::Unicode { name: name_b },
        ) => name_a == name_b,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::reactions_eq;
    use crate::{model::CachedMessage, test};
    use twilight_model::{
        channel::message::{EmojiReactionType, Reaction},
        gateway::{
            payload::incoming::{ReactionRemove, ReactionRemoveAll, ReactionRemoveEmoji},
            GatewayReaction,
        },
        id::Id,
    };

    fn find_custom_react(msg: &CachedMessage) -> Option<&Reaction> {
        msg.reactions.iter().find(|&r| {
            reactions_eq(
                &r.emoji,
                &EmojiReactionType::Custom {
                    animated: false,
                    id: Id::new(6),
                    name: None,
                },
            )
        })
    }

    #[test]
    fn reaction_add() {
        let cache = test::cache_with_message_and_reactions();
        let msg = cache.message(Id::new(4)).unwrap();

        assert_eq!(msg.reactions.len(), 3);

        let world_react = msg
            .reactions
            .iter()
            .find(|&r| matches!(&r.emoji, EmojiReactionType::Unicode {name} if name == "🗺️"));
        let smiley_react = msg
            .reactions
            .iter()
            .find(|&r| matches!(&r.emoji, EmojiReactionType::Unicode {name} if name == "😀"));
        let custom_react = find_custom_react(&msg);

        assert!(world_react.is_some());
        assert_eq!(world_react.unwrap().count, 1);
        assert!(smiley_react.is_some());
        assert_eq!(smiley_react.unwrap().count, 2);
        assert!(custom_react.is_some());
        assert_eq!(custom_react.unwrap().count, 1);
    }

    #[test]
    fn reaction_remove() {
        let cache = test::cache_with_message_and_reactions();
        cache.update(&ReactionRemove(GatewayReaction {
            burst: false,
            burst_colors: Vec::new(),
            channel_id: Id::new(2),
            emoji: EmojiReactionType::Unicode {
                name: "😀".to_owned(),
            },
            guild_id: Some(Id::new(1)),
            member: None,
            message_author_id: None,
            message_id: Id::new(4),
            user_id: Id::new(5),
        }));
        cache.update(&ReactionRemove(GatewayReaction {
            burst: false,
            burst_colors: Vec::new(),
            channel_id: Id::new(2),
            emoji: EmojiReactionType::Custom {
                animated: false,
                id: Id::new(6),
                name: None,
            },
            guild_id: Some(Id::new(1)),
            member: None,
            message_author_id: None,
            message_id: Id::new(4),
            user_id: Id::new(5),
        }));

        let msg = cache.message(Id::new(4)).unwrap();

        assert_eq!(msg.reactions.len(), 2);

        let world_react = msg
            .reactions
            .iter()
            .find(|&r| matches!(&r.emoji, EmojiReactionType::Unicode {name} if name == "🗺️"));
        let smiley_react = msg
            .reactions
            .iter()
            .find(|&r| matches!(&r.emoji, EmojiReactionType::Unicode {name} if name == "😀"));
        let custom_react = find_custom_react(&msg);

        assert!(world_react.is_some());
        assert_eq!(world_react.unwrap().count, 1);
        assert!(smiley_react.is_some());
        assert_eq!(smiley_react.unwrap().count, 1);
        assert!(custom_react.is_none());
    }

    #[test]
    fn reaction_remove_all() {
        let cache = test::cache_with_message_and_reactions();
        cache.update(&ReactionRemoveAll {
            channel_id: Id::new(2),
            message_id: Id::new(4),
            guild_id: Some(Id::new(1)),
        });

        let msg = cache.message(Id::new(4)).unwrap();

        assert_eq!(msg.reactions.len(), 0);
    }

    #[test]
    fn reaction_remove_emoji() {
        let cache = test::cache_with_message_and_reactions();
        cache.update(&ReactionRemoveEmoji {
            channel_id: Id::new(2),
            emoji: EmojiReactionType::Unicode {
                name: "😀".to_owned(),
            },
            guild_id: Id::new(1),
            message_id: Id::new(4),
        });
        cache.update(&ReactionRemoveEmoji {
            channel_id: Id::new(2),
            emoji: EmojiReactionType::Custom {
                animated: false,
                id: Id::new(6),
                name: None,
            },
            guild_id: Id::new(1),
            message_id: Id::new(4),
        });

        let msg = cache.message(Id::new(4)).unwrap();

        assert_eq!(msg.reactions.len(), 1);

        let world_react = msg
            .reactions
            .iter()
            .find(|&r| matches!(&r.emoji, EmojiReactionType::Unicode {name} if name == "🗺️"));
        let smiley_react = msg
            .reactions
            .iter()
            .find(|&r| matches!(&r.emoji, EmojiReactionType::Unicode {name} if name == "😀"));
        let custom_react = find_custom_react(&msg);

        assert!(world_react.is_some());
        assert_eq!(world_react.unwrap().count, 1);
        assert!(smiley_react.is_none());
        assert!(custom_react.is_none());
    }
}
