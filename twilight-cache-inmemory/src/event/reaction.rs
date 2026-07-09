use crate::{
    CacheableModels, InMemoryCache, UpdateCache,
    config::ResourceType,
    traits::{CacheableCurrentUser, CacheableMessage},
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
            if !reaction.me
                && let Some(current_user) = cache.current_user()
                && current_user.id() == self.0.user_id
            {
                reaction.me = true;
            }

            reaction.count += 1;
            if self.0.burst {
                reaction.count_details.burst += 1;
            } else {
                reaction.count_details.normal += 1;
            }
        } else {
            let me = cache
                .current_user()
                .is_some_and(|user| user.id() == self.0.user_id);

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
            if reaction.me
                && let Some(current_user) = cache.current_user()
                && current_user.id() == self.0.user_id
            {
                reaction.me = false;
            }

            if reaction.count > 1 {
                reaction.count -= 1;
                if self.0.burst {
                    reaction.count_details.burst -= 1;
                } else {
                    reaction.count_details.normal -= 1;
                }
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
        channel::message::{EmojiReactionType, Message, MessageType, Reaction},
        gateway::{
            GatewayReaction,
            payload::incoming::{
                MessageCreate, ReactionAdd, ReactionRemove, ReactionRemoveAll, ReactionRemoveEmoji,
            },
        },
        id::Id,
        user::User,
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

    fn make_message() -> Message {
        Message {
            activity: None,
            application: None,
            application_id: None,
            attachments: Vec::new(),
            author: User {
                accent_color: None,
                avatar: None,
                avatar_decoration: None,
                avatar_decoration_data: None,
                banner: None,
                bot: false,
                discriminator: 1,
                email: None,
                flags: None,
                global_name: None,
                id: Id::new(1),
                locale: None,
                mfa_enabled: None,
                name: "test".to_owned(),
                premium_type: None,
                primary_guild: None,
                public_flags: None,
                system: None,
                verified: None,
            },
            call: None,
            channel_id: Id::new(2),
            components: Vec::new(),
            content: "ping".to_owned(),
            edited_timestamp: None,
            embeds: Vec::new(),
            flags: None,
            guild_id: Some(Id::new(1)),
            id: Id::new(3),
            #[allow(deprecated)]
            interaction: None,
            interaction_metadata: None,
            kind: MessageType::Regular,
            member: None,
            mention_channels: Vec::new(),
            mention_everyone: false,
            mention_roles: Vec::new(),
            mentions: Vec::new(),
            message_snapshots: Vec::new(),
            pinned: false,
            poll: None,
            reactions: Vec::new(),
            reference: None,
            referenced_message: None,
            role_subscription_data: None,
            sticker_items: Vec::new(),
            timestamp: twilight_model::util::Timestamp::from_secs(1_632_072_645).expect("non zero"),
            thread: None,
            tts: false,
            webhook_id: None,
        }
    }

    fn cache_with_message() -> crate::DefaultInMemoryCache {
        let cache = crate::DefaultInMemoryCache::new();
        cache.update(&MessageCreate(make_message()));
        cache
    }

    #[test]
    fn reaction_add_burst_count_details() {
        let cache = cache_with_message();
        // Seed with a normal reaction so the reaction already exists.
        cache.update(&ReactionAdd(GatewayReaction {
            burst: false,
            burst_colors: Vec::new(),
            channel_id: Id::new(2),
            emoji: EmojiReactionType::Unicode {
                name: "😀".to_owned(),
            },
            guild_id: Some(Id::new(1)),
            member: None,
            message_author_id: None,
            message_id: Id::new(3),
            user_id: Id::new(9),
        }));
        // Add a burst reaction on the same emoji.
        cache.update(&ReactionAdd(GatewayReaction {
            burst: true,
            burst_colors: Vec::new(),
            channel_id: Id::new(2),
            emoji: EmojiReactionType::Unicode {
                name: "😀".to_owned(),
            },
            guild_id: Some(Id::new(1)),
            member: None,
            message_author_id: None,
            message_id: Id::new(3),
            user_id: Id::new(10),
        }));

        let msg = cache.message(Id::new(3)).unwrap();
        let reaction = &msg.reactions[0];
        assert_eq!(reaction.count, 2);
        assert_eq!(reaction.count_details.burst, 1);
        assert_eq!(reaction.count_details.normal, 1);
    }

    #[test]
    fn reaction_add_normal_count_details() {
        let cache = cache_with_message();
        cache.update(&ReactionAdd(GatewayReaction {
            burst: false,
            burst_colors: Vec::new(),
            channel_id: Id::new(2),
            emoji: EmojiReactionType::Unicode {
                name: "😀".to_owned(),
            },
            guild_id: Some(Id::new(1)),
            member: None,
            message_author_id: None,
            message_id: Id::new(3),
            user_id: Id::new(10),
        }));

        let msg = cache.message(Id::new(3)).unwrap();
        let reaction = &msg.reactions[0];
        assert_eq!(reaction.count, 1);
        assert_eq!(reaction.count_details.burst, 0);
        assert_eq!(reaction.count_details.normal, 1);
    }

    #[test]
    fn reaction_remove_burst_count_details() {
        let cache = cache_with_message();
        // Seed with a normal reaction so the reaction already exists.
        cache.update(&ReactionAdd(GatewayReaction {
            burst: false,
            burst_colors: Vec::new(),
            channel_id: Id::new(2),
            emoji: EmojiReactionType::Unicode {
                name: "😀".to_owned(),
            },
            guild_id: Some(Id::new(1)),
            member: None,
            message_author_id: None,
            message_id: Id::new(3),
            user_id: Id::new(9),
        }));
        // Add a burst reaction on top.
        cache.update(&ReactionAdd(GatewayReaction {
            burst: true,
            burst_colors: Vec::new(),
            channel_id: Id::new(2),
            emoji: EmojiReactionType::Unicode {
                name: "😀".to_owned(),
            },
            guild_id: Some(Id::new(1)),
            member: None,
            message_author_id: None,
            message_id: Id::new(3),
            user_id: Id::new(10),
        }));
        // Remove the burst reaction.
        cache.update(&ReactionRemove(GatewayReaction {
            burst: true,
            burst_colors: Vec::new(),
            channel_id: Id::new(2),
            emoji: EmojiReactionType::Unicode {
                name: "😀".to_owned(),
            },
            guild_id: Some(Id::new(1)),
            member: None,
            message_author_id: None,
            message_id: Id::new(3),
            user_id: Id::new(10),
        }));

        let msg = cache.message(Id::new(3)).unwrap();
        let reaction = &msg.reactions[0];
        assert_eq!(reaction.count, 1);
        assert_eq!(reaction.count_details.burst, 0);
        assert_eq!(reaction.count_details.normal, 1);
    }

    #[test]
    fn reaction_remove_normal_count_details() {
        let cache = cache_with_message();
        cache.update(&ReactionAdd(GatewayReaction {
            burst: false,
            burst_colors: Vec::new(),
            channel_id: Id::new(2),
            emoji: EmojiReactionType::Unicode {
                name: "😀".to_owned(),
            },
            guild_id: Some(Id::new(1)),
            member: None,
            message_author_id: None,
            message_id: Id::new(3),
            user_id: Id::new(10),
        }));
        cache.update(&ReactionAdd(GatewayReaction {
            burst: false,
            burst_colors: Vec::new(),
            channel_id: Id::new(2),
            emoji: EmojiReactionType::Unicode {
                name: "😀".to_owned(),
            },
            guild_id: Some(Id::new(1)),
            member: None,
            message_author_id: None,
            message_id: Id::new(3),
            user_id: Id::new(11),
        }));
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
            message_id: Id::new(3),
            user_id: Id::new(10),
        }));

        let msg = cache.message(Id::new(3)).unwrap();
        let reaction = &msg.reactions[0];
        assert_eq!(reaction.count, 1);
        assert_eq!(reaction.count_details.burst, 0);
        assert_eq!(reaction.count_details.normal, 1);
    }

    #[test]
    fn reaction_add_preserves_count_details_on_second_add() {
        let cache = cache_with_message();
        cache.update(&ReactionAdd(GatewayReaction {
            burst: false,
            burst_colors: Vec::new(),
            channel_id: Id::new(2),
            emoji: EmojiReactionType::Unicode {
                name: "😀".to_owned(),
            },
            guild_id: Some(Id::new(1)),
            member: None,
            message_author_id: None,
            message_id: Id::new(3),
            user_id: Id::new(10),
        }));

        {
            let msg = cache.message(Id::new(3)).unwrap();
            let reaction = &msg.reactions[0];
            assert_eq!(reaction.count, 1);
            assert_eq!(reaction.count_details.burst, 0);
            assert_eq!(reaction.count_details.normal, 1);
        }

        // Second normal add on same emoji.
        cache.update(&ReactionAdd(GatewayReaction {
            burst: false,
            burst_colors: Vec::new(),
            channel_id: Id::new(2),
            emoji: EmojiReactionType::Unicode {
                name: "😀".to_owned(),
            },
            guild_id: Some(Id::new(1)),
            member: None,
            message_author_id: None,
            message_id: Id::new(3),
            user_id: Id::new(11),
        }));

        let msg = cache.message(Id::new(3)).unwrap();
        let reaction = &msg.reactions[0];
        assert_eq!(reaction.count, 2);
        assert_eq!(reaction.count_details.normal, 2);
        assert_eq!(reaction.count_details.burst, 0);
    }
}
