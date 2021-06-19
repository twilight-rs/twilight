use crate::{config::ResourceType, model::CachedMessage, InMemoryCache, UpdateCache};
use std::borrow::Cow;
use twilight_model::{
    gateway::payload::{MessageCreate, MessageDelete, MessageDeleteBulk, MessageUpdate},
    id::{ChannelId, MessageId},
};

impl InMemoryCache {
    /// Gets a message by channel ID and message ID.
    ///
    /// This is an O(n) operation. This requires one or both of the
    /// [`GUILD_MESSAGES`] or [`DIRECT_MESSAGES`] intents.
    ///
    /// [`GUILD_MESSAGES`]: ::twilight_model::gateway::Intents::GUILD_MESSAGES
    /// [`DIRECT_MESSAGES`]: ::twilight_model::gateway::Intents::DIRECT_MESSAGES
    pub fn message(&self, channel_id: ChannelId, message_id: MessageId) -> Option<CachedMessage> {
        let channel = self.0.messages.get(&channel_id)?;

        channel.iter().find(|msg| msg.id == message_id).cloned()
    }
}

impl UpdateCache for MessageCreate {
    fn update(&self, cache: &InMemoryCache) {
        if cache.wants(ResourceType::USER) {
            cache.cache_user(Cow::Borrowed(&self.author), self.guild_id);
        }

        if let (Some(member), Some(guild_id), true) = (
            &self.member,
            self.guild_id,
            cache.wants(ResourceType::MEMBER),
        ) {
            cache.cache_borrowed_partial_member(guild_id, member, self.author.id)
        }

        if !cache.wants(ResourceType::MESSAGE) {
            return;
        }

        let mut channel = cache.0.messages.entry(self.0.channel_id).or_default();

        if channel.len() > cache.0.config.message_cache_size() {
            channel.pop_back();
        }

        channel.push_front(CachedMessage::from(self.0.clone()));
    }
}

impl UpdateCache for MessageDelete {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::MESSAGE) {
            return;
        }

        let mut channel = cache.0.messages.entry(self.channel_id).or_default();

        if let Some(idx) = channel.iter().position(|msg| msg.id == self.id) {
            channel.remove(idx);
        }
    }
}

impl UpdateCache for MessageDeleteBulk {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::MESSAGE) {
            return;
        }

        let mut channel = cache.0.messages.entry(self.channel_id).or_default();

        for id in &self.ids {
            if let Some(idx) = channel.iter().position(|msg| &msg.id == id) {
                channel.remove(idx);
            }
        }
    }
}

impl UpdateCache for MessageUpdate {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::MESSAGE) {
            return;
        }

        let mut channel = cache.0.messages.entry(self.channel_id).or_default();

        if let Some(mut message) = channel.iter_mut().find(|msg| msg.id == self.id) {
            if let Some(attachments) = &self.attachments {
                message.attachments = attachments.clone();
            }

            if let Some(content) = &self.content {
                message.content = content.clone();
            }

            if let Some(edited_timestamp) = &self.edited_timestamp {
                message.edited_timestamp.replace(edited_timestamp.clone());
            }

            if let Some(embeds) = &self.embeds {
                message.embeds = embeds.clone();
            }

            if let Some(mention_everyone) = self.mention_everyone {
                message.mention_everyone = mention_everyone;
            }

            if let Some(mention_roles) = &self.mention_roles {
                message.mention_roles = mention_roles.clone();
            }

            if let Some(mentions) = &self.mentions {
                message.mentions = mentions.iter().map(|x| x.id).collect::<Vec<_>>();
            }

            if let Some(pinned) = self.pinned {
                message.pinned = pinned;
            }

            if let Some(timestamp) = &self.timestamp {
                message.timestamp = timestamp.clone();
            }

            if let Some(tts) = self.tts {
                message.tts = tts;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;
    use twilight_model::{
        channel::message::Mention,
        guild::{PartialMember, Permissions},
        id::{ChannelId, GuildId, UserId},
        user::{User, UserFlags},
    };

    #[test]
    fn test_message_lifecycle() {
        let cache = InMemoryCache::new();

        let event = MessageCreate(test::message(MessageId(2), "content".to_string()));
        cache.update(&event);

        {
            let message = cache.message(ChannelId(2), MessageId(2)).unwrap();
            assert_eq!("content".to_string(), message.content);
        }

        let event = MessageUpdate {
            attachments: None,
            author: None,
            channel_id: ChannelId(2),
            content: Some("content <@123>".into()),
            edited_timestamp: Some("timestamp".into()),
            embeds: None,
            guild_id: Some(GuildId(1)),
            id: MessageId(2),
            kind: None,
            mention_everyone: None,
            mention_roles: None,
            mentions: Some(Vec::from([Mention {
                avatar: None,
                bot: false,
                discriminator: "1234".into(),
                id: UserId(123),
                member: Some(PartialMember {
                    deaf: false,
                    joined_at: None,
                    mute: false,
                    nick: None,
                    permissions: Some(Permissions::empty()),
                    premium_since: None,
                    roles: Vec::new(),
                    user: Some(User {
                        avatar: None,
                        bot: false,
                        discriminator: "1234".into(),
                        email: None,
                        flags: None,
                        id: UserId(123),
                        locale: None,
                        mfa_enabled: None,
                        name: "username".into(),
                        premium_type: None,
                        public_flags: None,
                        system: None,
                        verified: None,
                    }),
                }),
                name: "username".into(),
                public_flags: UserFlags::empty(),
            }])),
            pinned: None,
            timestamp: None,
            tts: None,
        };
        cache.update(&event);

        {
            let message = cache.message(ChannelId(2), MessageId(2)).unwrap();

            assert_eq!("content <@123>".to_string(), message.content);
            assert_eq!(1, message.mentions.len());
        }

        (3..=10)
            .map(|id| MessageId(id))
            .map(|id| test::message(id, "content".to_string()))
            .map(|message| MessageCreate(message))
            .map(|event| cache.update(&event))
            .for_each(drop);

        {
            let channel_messages = cache.0.messages.get(&ChannelId(2)).unwrap();

            assert_eq!(9, channel_messages.len());
            assert!(cache.message(ChannelId(2), MessageId(10)).is_some());
        }

        let event = MessageDeleteBulk {
            channel_id: ChannelId(2),
            guild_id: Some(GuildId(2)),
            ids: (3..=10).map(|id| MessageId(id)).collect(),
        };
        cache.update(&event);

        {
            let channel_messages = cache.0.messages.get(&ChannelId(2)).unwrap();

            assert_eq!(1, channel_messages.len());
            assert!(cache.message(ChannelId(2), MessageId(10)).is_none());
        }

        let event = MessageDelete {
            channel_id: ChannelId(2),
            guild_id: Some(GuildId(2)),
            id: MessageId(2),
        };
        cache.update(&event);

        {
            assert!(cache.message(ChannelId(2), MessageId(2)).is_none());
        }
    }

    #[test]
    fn test_message_create() {
        let cache = InMemoryCache::builder()
            .resource_types(ResourceType::MESSAGE | ResourceType::MEMBER | ResourceType::USER)
            .message_cache_size(1)
            .build();

        let event = MessageCreate(test::message(MessageId(4), "content".to_string()));
        cache.update(&event);

        {
            let entry = cache.0.users.get(&UserId(3)).unwrap();
            assert_eq!(entry.value().1.len(), 1);
        }

        assert_eq!(
            cache.member(GuildId(1), UserId(3)).unwrap().user_id,
            UserId(3),
        );

        {
            let entry = cache.0.messages.get(&ChannelId(2)).unwrap();
            assert_eq!(entry.value().len(), 1);
        }
    }
}
