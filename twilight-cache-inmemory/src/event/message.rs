use crate::{CacheableModels, InMemoryCache, UpdateCache, config::ResourceType};
use std::borrow::Cow;
use twilight_model::gateway::payload::incoming::{
    MessageCreate, MessageDelete, MessageDeleteBulk, MessageUpdate,
};

impl<CacheModels: CacheableModels> UpdateCache<CacheModels> for MessageCreate {
    fn update(&self, cache: &InMemoryCache<CacheModels>) {
        if cache.wants(ResourceType::USER) {
            cache.cache_user(Cow::Borrowed(&self.author), self.guild_id);
        }

        if let (Some(member), Some(guild_id), true) = (
            &self.member,
            self.guild_id,
            cache.wants(ResourceType::MEMBER),
        ) {
            cache.cache_borrowed_partial_member(guild_id, member, self.author.id);
        }

        if !cache.wants(ResourceType::MESSAGE) {
            return;
        }

        let mut channel_messages = cache.channel_messages.entry(self.0.channel_id).or_default();

        // If the channel has more messages than the cache size the user has
        // requested then we pop a message ID out. Once we have the popped ID we
        // can remove it from the message cache. This prevents the cache from
        // filling up with old messages that aren't in any channel cache.
        if channel_messages.len() >= cache.config.message_cache_size() {
            if let Some(popped_id) = channel_messages.pop_back() {
                cache.messages.remove(&popped_id);
            }
        }

        channel_messages.push_front(self.0.id);
        cache
            .messages
            .insert(self.0.id, CacheModels::Message::from(self.0.clone()));
    }
}

impl<CacheModels: CacheableModels> UpdateCache<CacheModels> for MessageDelete {
    fn update(&self, cache: &InMemoryCache<CacheModels>) {
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

impl<CacheModels: CacheableModels> UpdateCache<CacheModels> for MessageDeleteBulk {
    fn update(&self, cache: &InMemoryCache<CacheModels>) {
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

impl<CacheModels: CacheableModels> UpdateCache<CacheModels> for MessageUpdate {
    fn update(&self, cache: &InMemoryCache<CacheModels>) {
        if cache.wants(ResourceType::USER) {
            cache.cache_user(Cow::Borrowed(&self.author), self.guild_id);
        }

        if let (Some(member), Some(guild_id), true) = (
            &self.member,
            self.guild_id,
            cache.wants(ResourceType::MEMBER),
        ) {
            cache.cache_borrowed_partial_member(guild_id, member, self.author.id);
        }

        if !cache.wants(ResourceType::MESSAGE) {
            return;
        }

        // In special cases, this message was popped out due to the limitation
        // of the message cache capacity, or its Event::MessageCreate was missed.
        // If that is the case, we do not only add it to the message cache but
        // also add its ID to the channel messages cache.
        if cache
            .messages
            .insert(self.id, CacheModels::Message::from(self.0.clone()))
            .is_some()
        {
            return;
        }

        let mut channel_messages = cache.channel_messages.entry(self.0.channel_id).or_default();

        // If this channel cache is full, we pop an message ID out of
        // the channel cache and also remove it from the message cache.
        if channel_messages.len() >= cache.config.message_cache_size() {
            if let Some(popped_id) = channel_messages.pop_back() {
                cache.messages.remove(&popped_id);
            }
        }

        channel_messages.push_front(self.0.id);
    }
}

#[cfg(test)]
mod tests {
    use crate::{DefaultInMemoryCache, ResourceType};
    use twilight_model::{
        channel::message::{Message, MessageFlags, MessageType},
        gateway::payload::incoming::MessageCreate,
        guild::{MemberFlags, PartialMember},
        id::Id,
        user::User,
        util::{ImageHash, Timestamp, image_hash::ImageHashParseError},
    };

    #[allow(deprecated)]
    #[test]
    fn message_create() -> Result<(), ImageHashParseError> {
        let joined_at = Some(Timestamp::from_secs(1_632_072_645).expect("non zero"));
        let cache = DefaultInMemoryCache::builder()
            .resource_types(ResourceType::MESSAGE | ResourceType::MEMBER | ResourceType::USER)
            .message_cache_size(2)
            .build();

        let avatar = ImageHash::parse(b"e91c75bc7656063cc745f4e79d0b7664")?;
        let flags = MemberFlags::BYPASSES_VERIFICATION | MemberFlags::DID_REJOIN;
        let mut msg = Message {
            activity: None,
            application: None,
            application_id: None,
            attachments: Vec::new(),
            author: User {
                accent_color: None,
                avatar: Some(avatar),
                avatar_decoration: None,
                avatar_decoration_data: None,
                banner: None,
                bot: false,
                discriminator: 1,
                email: None,
                flags: None,
                global_name: Some("test".to_owned()),
                id: Id::new(3),
                locale: None,
                mfa_enabled: None,
                name: "test".to_owned(),
                premium_type: None,
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
            flags: Some(MessageFlags::empty()),
            guild_id: Some(Id::new(1)),
            id: Id::new(4),
            interaction: None,
            interaction_metadata: None,
            kind: MessageType::Regular,
            member: Some(PartialMember {
                avatar: None,
                communication_disabled_until: None,
                deaf: false,
                flags,
                joined_at,
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
            message_snapshots: Vec::new(),
            pinned: false,
            poll: None,
            reactions: Vec::new(),
            reference: None,
            referenced_message: None,
            role_subscription_data: None,
            sticker_items: Vec::new(),
            timestamp: Timestamp::from_secs(1_632_072_645).expect("non zero"),
            thread: None,
            tts: false,
            webhook_id: None,
        };

        cache.update(&MessageCreate(msg.clone()));
        msg.id = Id::new(5);
        cache.update(&MessageCreate(msg));

        {
            let entry = cache.user_guilds(Id::new(3)).unwrap();
            assert_eq!(entry.value().len(), 1);
        }
        assert_eq!(
            cache.member(Id::new(1), Id::new(3)).unwrap().user_id,
            Id::new(3),
        );
        {
            let entry = cache.channel_messages.get(&Id::new(2)).unwrap();
            assert_eq!(entry.value().len(), 2);
        }

        let messages = cache
            .channel_messages(Id::new(2))
            .expect("channel is in cache");

        let mut iter = messages.iter();
        // messages are iterated over in descending order from insertion
        assert_eq!(Some(&Id::new(5)), iter.next());
        assert_eq!(Some(&Id::new(4)), iter.next());
        assert!(iter.next().is_none());

        Ok(())
    }
}
