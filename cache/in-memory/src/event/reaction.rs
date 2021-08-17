use crate::{config::ResourceType, InMemoryCache, UpdateCache};
use twilight_model::{
    channel::message::MessageReaction,
    gateway::payload::{ReactionAdd, ReactionRemove, ReactionRemoveAll, ReactionRemoveEmoji},
};

impl UpdateCache for ReactionAdd {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::REACTION) {
            return;
        }

        let mut channel = cache.0.messages.entry(self.0.channel_id).or_default();

        let message = match channel.iter_mut().find(|msg| msg.id() == self.0.message_id) {
            Some(message) => message,
            None => return,
        };

        if let Some(reaction) = message
            .reactions
            .iter_mut()
            .find(|r| r.emoji == self.0.emoji)
        {
            if !reaction.me {
                if let Some(current_user) = cache.current_user() {
                    if current_user.id == self.0.user_id {
                        reaction.me = true;
                    }
                }
            }

            reaction.count += 1;
        } else {
            let me = cache
                .current_user()
                .map(|user| user.id == self.0.user_id)
                .unwrap_or_default();

            message.reactions.push(MessageReaction {
                count: 1,
                emoji: self.0.emoji.clone(),
                me,
            });
        }
    }
}

impl UpdateCache for ReactionRemove {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::REACTION) {
            return;
        }

        let mut channel = cache.0.messages.entry(self.0.channel_id).or_default();

        let message = match channel.iter_mut().find(|msg| msg.id() == self.0.message_id) {
            Some(message) => message,
            None => return,
        };

        if let Some(reaction) = message
            .reactions
            .iter_mut()
            .find(|r| r.emoji == self.0.emoji)
        {
            if reaction.me {
                if let Some(current_user) = cache.current_user() {
                    if current_user.id == self.0.user_id {
                        reaction.me = false;
                    }
                }
            }

            if reaction.count > 1 {
                reaction.count -= 1;
            } else {
                message.reactions.retain(|e| !(e.emoji == self.0.emoji));
            }
        }
    }
}

impl UpdateCache for ReactionRemoveAll {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::REACTION) {
            return;
        }

        let mut channel = cache.0.messages.entry(self.channel_id).or_default();

        let message = match channel.iter_mut().find(|msg| msg.id() == self.message_id) {
            Some(message) => message,
            None => return,
        };

        message.reactions.clear();
    }
}

impl UpdateCache for ReactionRemoveEmoji {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::REACTION) {
            return;
        }

        let mut channel = cache.0.messages.entry(self.channel_id).or_default();

        let message = match channel.iter_mut().find(|msg| msg.id() == self.message_id) {
            Some(message) => message,
            None => return,
        };

        let maybe_index = message.reactions.iter().position(|r| r.emoji == self.emoji);

        if let Some(index) = maybe_index {
            message.reactions.remove(index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;
    use twilight_model::{
        channel::{Reaction, ReactionType},
        id::{ChannelId, GuildId, MessageId, UserId},
    };

    #[test]
    fn test_reaction_add() {
        let cache = test::cache_with_message_and_reactions();
        let msg = cache.message(ChannelId(2), MessageId(4)).unwrap();

        assert_eq!(msg.reactions.len(), 2);

        let world_react = msg
            .reactions
            .iter()
            .find(|&r| matches!(&r.emoji, ReactionType::Unicode {name} if name == "🗺️"));
        let smiley_react = msg
            .reactions
            .iter()
            .find(|&r| matches!(&r.emoji, ReactionType::Unicode {name} if name == "😀"));

        assert!(world_react.is_some());
        assert_eq!(world_react.unwrap().count, 1);
        assert!(smiley_react.is_some());
        assert_eq!(smiley_react.unwrap().count, 2);
    }

    #[test]
    fn test_reaction_remove() {
        let cache = test::cache_with_message_and_reactions();
        cache.update(&ReactionRemove(Reaction {
            channel_id: ChannelId(2),
            emoji: ReactionType::Unicode {
                name: "😀".to_owned(),
            },
            guild_id: Some(GuildId(1)),
            member: None,
            message_id: MessageId(4),
            user_id: UserId(5),
        }));

        let msg = cache.message(ChannelId(2), MessageId(4)).unwrap();

        assert_eq!(msg.reactions.len(), 2);

        let world_react = msg
            .reactions
            .iter()
            .find(|&r| matches!(&r.emoji, ReactionType::Unicode {name} if name == "🗺️"));
        let smiley_react = msg
            .reactions
            .iter()
            .find(|&r| matches!(&r.emoji, ReactionType::Unicode {name} if name == "😀"));

        assert!(world_react.is_some());
        assert_eq!(world_react.unwrap().count, 1);
        assert!(smiley_react.is_some());
        assert_eq!(smiley_react.unwrap().count, 1);
    }

    #[test]
    fn test_reaction_remove_all() {
        let cache = test::cache_with_message_and_reactions();
        cache.update(&ReactionRemoveAll {
            channel_id: ChannelId(2),
            message_id: MessageId(4),
            guild_id: Some(GuildId(1)),
        });

        let msg = cache.message(ChannelId(2), MessageId(4)).unwrap();

        assert_eq!(msg.reactions.len(), 0);
    }

    #[test]
    fn test_reaction_remove_emoji() {
        let cache = test::cache_with_message_and_reactions();
        cache.update(&ReactionRemoveEmoji {
            channel_id: ChannelId(2),
            emoji: ReactionType::Unicode {
                name: "😀".to_owned(),
            },
            guild_id: GuildId(1),
            message_id: MessageId(4),
        });

        let msg = cache.message(ChannelId(2), MessageId(4)).unwrap();

        assert_eq!(msg.reactions.len(), 1);

        let world_react = msg
            .reactions
            .iter()
            .find(|&r| matches!(&r.emoji, ReactionType::Unicode {name} if name == "🗺️"));
        let smiley_react = msg
            .reactions
            .iter()
            .find(|&r| matches!(&r.emoji, ReactionType::Unicode {name} if name == "😀"));

        assert!(world_react.is_some());
        assert_eq!(world_react.unwrap().count, 1);
        assert!(smiley_react.is_none());
    }
}
