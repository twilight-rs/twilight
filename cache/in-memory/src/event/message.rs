use crate::{config::ResourceType, model::CachedMessage, InMemoryCache, UpdateCache};
use std::borrow::Cow;
use twilight_model::gateway::payload::incoming::{
    MessageCreate, MessageDelete, MessageDeleteBulk, MessageUpdate,
};

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

        let mut channel_messages = cache.channel_messages.entry(self.0.channel_id).or_default();

        // If the channel has more messages than the cache size the user has
        // requested then we pop a message ID out. Once we have the popped ID we
        // can remove it from the message cache. This prevents the cache from
        // filling up with old messages that aren't in any channel cache.
        if channel_messages.len() > cache.config.message_cache_size() {
            if let Some(popped_id) = channel_messages.pop_back() {
                cache.messages.remove(&popped_id);
            }
        }

        channel_messages.push_front(self.0.id);
        cache
            .messages
            .insert(self.0.id, CachedMessage::from(self.0.clone()));
    }
}

impl UpdateCache for MessageDelete {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::MESSAGE) {
            return;
        }

        cache.messages.remove(&self.id);

        let mut channel_messages = cache.channel_messages.entry(self.channel_id).or_default();

        if let Some(idx) = channel_messages.iter().position(|id| *id == self.id) {
            channel_messages.remove(idx);
        }
    }
}

impl UpdateCache for MessageDeleteBulk {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::MESSAGE) {
            return;
        }

        let mut channel_messages = cache.channel_messages.entry(self.channel_id).or_default();

        for id in &self.ids {
            cache.messages.remove(id);

            if let Some(idx) = channel_messages
                .iter()
                .position(|message_id| message_id == id)
            {
                channel_messages.remove(idx);
            }
        }
    }
}

impl UpdateCache for MessageUpdate {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::MESSAGE) {
            return;
        }

        if let Some(mut message) = cache.messages.get_mut(&self.id) {
            if let Some(attachments) = &self.attachments {
                message.attachments = attachments.clone();
            }

            if let Some(content) = &self.content {
                message.content = content.clone();
            }

            if let Some(edited_timestamp) = self.edited_timestamp {
                message.edited_timestamp.replace(edited_timestamp);
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

            if let Some(timestamp) = self.timestamp {
                message.timestamp = timestamp;
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
    use twilight_model::{
        channel::message::{Message, MessageFlags, MessageType},
        datetime::Timestamp,
        guild::PartialMember,
        id::{ChannelId, GuildId, MessageId, UserId},
        user::User,
    };

    #[test]
    fn test_message_create() {
        let cache = InMemoryCache::builder()
            .resource_types(ResourceType::MESSAGE | ResourceType::MEMBER | ResourceType::USER)
            .message_cache_size(1)
            .build();
        let msg = Message {
            activity: None,
            application: None,
            application_id: None,
            attachments: Vec::new(),
            author: User {
                accent_color: None,
                avatar: Some("".to_owned()),
                banner: None,
                bot: false,
                discriminator: 1,
                email: None,
                flags: None,
                id: UserId::new(3).expect("non zero"),
                locale: None,
                mfa_enabled: None,
                name: "test".to_owned(),
                premium_type: None,
                public_flags: None,
                system: None,
                verified: None,
            },
            channel_id: ChannelId::new(2).expect("non zero"),
            components: Vec::new(),
            content: "ping".to_owned(),
            edited_timestamp: None,
            embeds: Vec::new(),
            flags: Some(MessageFlags::empty()),
            guild_id: Some(GuildId::new(1).expect("non zero")),
            id: MessageId::new(4).expect("non zero"),
            interaction: None,
            kind: MessageType::Regular,
            member: Some(PartialMember {
                deaf: false,
                joined_at: None,
                mute: false,
                nick: Some("member nick".to_owned()),
                permissions: None,
                premium_since: None,
                roles: Vec::new(),
                user: None,
            }),
            mention_channels: Vec::new(),
            mention_everyone: false,
            mention_roles: Vec::new(),
            mentions: Vec::new(),
            pinned: false,
            reactions: Vec::new(),
            reference: None,
            sticker_items: Vec::new(),
            thread: None,
            referenced_message: None,
            timestamp: Timestamp::from_secs(1_632_072_645).expect("non zero"),
            tts: false,
            webhook_id: None,
        };

        cache.update(&MessageCreate(msg));

        {
            let entry = cache
                .user_guilds
                .get(&UserId::new(3).expect("non zero"))
                .unwrap();
            assert_eq!(entry.value().len(), 1);
        }
        assert_eq!(
            cache
                .member(
                    GuildId::new(1).expect("non zero"),
                    UserId::new(3).expect("non zero")
                )
                .unwrap()
                .user_id,
            UserId::new(3).expect("non zero"),
        );
        {
            let entry = cache
                .channel_messages
                .get(&ChannelId::new(2).expect("non zero"))
                .unwrap();
            assert_eq!(entry.value().len(), 1);
        }
    }
}
