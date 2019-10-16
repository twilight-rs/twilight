use super::{config::EventType, InMemoryCache, InMemoryCacheError};
use async_trait::async_trait;
use dawn_cache_trait::{Cache, UpdateCache};
use dawn_model::{
    channel::{message::MessageReaction, Channel},
    gateway::payload::*,
    guild::GuildStatus,
    id::GuildId,
};
use futures_util::lock::Mutex;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    sync::Arc,
};

fn guard(this: &InMemoryCache, event_type: EventType) -> bool {
    this.0.config.event_types().contains(event_type)
}

#[async_trait]
impl UpdateCache<BanAdd> for InMemoryCache {
    async fn update(&self, _: &BanAdd) -> Result<(), InMemoryCacheError> {
        Ok(())
    }
}

#[async_trait]
impl UpdateCache<BanRemove> for InMemoryCache {
    async fn update(&self, _: &BanRemove) -> Result<(), InMemoryCacheError> {
        Ok(())
    }
}

#[async_trait]
impl UpdateCache<ChannelCreate> for InMemoryCache {
    async fn update(&self, event: &ChannelCreate) -> Result<(), InMemoryCacheError> {
        if !guard(self, EventType::CHANNEL_CREATE) {
            return Ok(());
        }

        match event.0.clone() {
            Channel::Group(c) => {
                super::upsert_item(&self.0.groups, c.id, c).await;
            },
            Channel::Guild(c) => {
                if let Some(gid) = super::guild_channel_guild_id(&c) {
                    self.cache_guild_channel(*gid, c.clone()).await;
                }
            },
            Channel::Private(c) => {
                self.cache_private_channel(c.clone()).await;
            },
        }

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<ChannelDelete> for InMemoryCache {
    async fn update(&self, event: &ChannelDelete) -> Result<(), InMemoryCacheError> {
        if !guard(self, EventType::CHANNEL_DELETE) {
            return Ok(());
        }

        match event.0 {
            Channel::Group(ref c) => {
                self.delete_group(c.id).await;
            },
            Channel::Guild(ref c) => {
                let id = *super::guild_channel_id(&c);

                self.delete_guild_channel(id).await;
            },
            Channel::Private(ref c) => {
                self.0.channels_private.lock().await.remove(&c.id);
            },
        }

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<ChannelPinsUpdate> for InMemoryCache {
    async fn update(&self, _: &ChannelPinsUpdate) -> Result<(), InMemoryCacheError> {
        Ok(())
    }
}

#[async_trait]
impl UpdateCache<ChannelUpdate> for InMemoryCache {
    async fn update(&self, event: &ChannelUpdate) -> Result<(), InMemoryCacheError> {
        if !guard(self, EventType::CHANNEL_UPDATE) {
            return Ok(());
        }

        match event.0.clone() {
            Channel::Group(c) => {
                self.cache_group(c.clone()).await;
            },
            Channel::Guild(c) => {
                if let Some(gid) = super::guild_channel_guild_id(&c) {
                    self.cache_guild_channel(*gid, c.clone()).await;
                }
            },
            Channel::Private(c) => {
                self.cache_private_channel(c.clone()).await;
            },
        }

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<GuildCreate> for InMemoryCache {
    async fn update(&self, event: &GuildCreate) -> Result<(), InMemoryCacheError> {
        if !guard(self, EventType::GUILD_CREATE) {
            return Ok(());
        }

        self.cache_guild(event.0.clone()).await;

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<GuildDelete> for InMemoryCache {
    async fn update(&self, event: &GuildDelete) -> Result<(), InMemoryCacheError> {
        async fn remove_ids<T: Eq + Hash, U>(
            guild_map: &Mutex<HashMap<GuildId, HashSet<T>>>,
            container: &Mutex<HashMap<T, U>>,
            guild_id: GuildId,
        ) {
            if let Some(ids) = guild_map.lock().await.remove(&guild_id) {
                let mut items = container.lock().await;

                for id in ids {
                    items.remove(&id);
                }
            }
        }

        if !guard(self, EventType::GUILD_DELETE) {
            return Ok(());
        }

        let id = event.guild.id;

        self.0.guilds.lock().await.remove(&id);

        remove_ids(&self.0.guild_channels, &self.0.channels_guild, id).await;
        remove_ids(&self.0.guild_emojis, &self.0.emojis, id).await;
        remove_ids(&self.0.guild_roles, &self.0.roles, id).await;
        remove_ids(&self.0.guild_voice_states, &self.0.voice_states, id).await;

        if let Some(ids) = self.0.guild_members.lock().await.remove(&id) {
            let mut members = self.0.members.lock().await;

            for user_id in ids {
                members.remove(&(id, user_id));
            }
        }

        if let Some(ids) = self.0.guild_presences.lock().await.remove(&id) {
            let mut presences = self.0.presences.lock().await;

            for user_id in ids {
                presences.remove(&(Some(id), user_id));
            }
        }

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<GuildEmojisUpdate> for InMemoryCache {
    async fn update(&self, event: &GuildEmojisUpdate) -> Result<(), InMemoryCacheError> {
        if !guard(self, EventType::GUILD_EMOJIS_UPDATE) {
            return Ok(());
        }

        self.cache_emojis(event.guild_id, event.emojis.values().cloned())
            .await;

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<GuildIntegrationsUpdate> for InMemoryCache {
    async fn update(&self, _: &GuildIntegrationsUpdate) -> Result<(), InMemoryCacheError> {
        Ok(())
    }
}

#[async_trait]
impl UpdateCache<GuildUpdate> for InMemoryCache {
    async fn update(&self, event: &GuildUpdate) -> Result<(), InMemoryCacheError> {
        if !guard(self, EventType::GUILD_UPDATE) {
            return Ok(());
        }

        let mut guilds = self.0.guilds.lock().await;

        let mut guild = match guilds.get_mut(&event.0.id).cloned() {
            Some(guild) => guild,
            None => return Ok(()),
        };

        let g = &event.0;

        let mut guild = Arc::make_mut(&mut guild);
        guild.afk_channel_id = g.afk_channel_id;
        guild.afk_timeout = g.afk_timeout;
        guild.banner = g.banner.clone();
        guild.default_message_notifications = g.default_message_notifications;
        guild.description = g.description.clone();
        guild.embed_channel_id = g.embed_channel_id;
        guild.embed_enabled.replace(g.embed_enabled);
        guild.features = g.features.clone();
        guild.icon = g.icon.clone();
        guild.max_members = g.max_members;
        guild.max_presences = g.max_presences;
        guild.mfa_level = g.mfa_level;
        guild.name = g.name.clone();
        guild.owner = g.owner;
        guild.owner_id = g.owner_id;
        guild.permissions = g.permissions;
        guild.preferred_locale = g.preferred_locale.clone();
        guild.premium_tier = g.premium_tier;
        guild
            .premium_subscription_count
            .replace(g.premium_subscription_count);
        guild.region = g.region.clone();
        guild.splash = g.splash.clone();
        guild.system_channel_id = g.system_channel_id;
        guild.verification_level = g.verification_level;
        guild.vanity_url_code = g.vanity_url_code.clone();
        guild.widget_channel_id = g.widget_channel_id;
        guild.widget_enabled = g.widget_enabled;

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<MemberAdd> for InMemoryCache {
    async fn update(&self, event: &MemberAdd) -> Result<(), InMemoryCacheError> {
        if !guard(self, EventType::MEMBER_ADD) {
            return Ok(());
        }

        // This will always be present on members from the gateway.
        let guild_id = match event.guild_id {
            Some(guild_id) => guild_id,
            None => return Ok(()),
        };

        self.cache_member(guild_id, event.0.clone()).await;

        let mut guild = self.0.guild_members.lock().await;
        guild.entry(guild_id).or_default().insert(event.0.user.id);

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<MemberChunk> for InMemoryCache {
    async fn update(&self, event: &MemberChunk) -> Result<(), InMemoryCacheError> {
        if !guard(self, EventType::MEMBER_CHUNK) {
            return Ok(());
        }

        if event.members.is_empty() {
            return Ok(());
        }

        self.cache_members(event.guild_id, event.members.values().cloned())
            .await;
        let user_ids = event.members.keys();
        let mut members = self.0.guild_members.lock().await;

        let guild = members.entry(event.guild_id).or_default();

        for id in user_ids {
            guild.insert(*id);
        }

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<MemberRemove> for InMemoryCache {
    async fn update(&self, event: &MemberRemove) -> Result<(), InMemoryCacheError> {
        if !guard(self, EventType::MEMBER_REMOVE) {
            return Ok(());
        }

        self.0
            .members
            .lock()
            .await
            .remove(&(event.guild_id, event.user.id));

        let mut guild_members = self.0.guild_members.lock().await;

        if let Some(members) = guild_members.get_mut(&event.guild_id) {
            members.remove(&event.user.id);
        }

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<MemberUpdate> for InMemoryCache {
    async fn update(&self, event: &MemberUpdate) -> Result<(), InMemoryCacheError> {
        if !guard(self, EventType::MEMBER_UPDATE) {
            return Ok(());
        }

        let mut members = self.0.members.lock().await;

        let mut member = match members.get_mut(&(event.guild_id, event.user.id)) {
            Some(member) => member,
            None => return Ok(()),
        };
        let mut member = Arc::make_mut(&mut member);

        member.nick = event.nick.clone();
        member.roles = event.roles.clone();

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<MessageCreate> for InMemoryCache {
    async fn update(&self, event: &MessageCreate) -> Result<(), InMemoryCacheError> {
        if !guard(self, EventType::MESSAGE_CREATE) {
            return Ok(());
        }

        let mut channels = self.0.messages.lock().await;
        let channel = channels.entry(event.0.channel_id).or_default();

        if channel.len() > self.0.config.message_cache_size() {
            if let Some(k) = channel.iter().next_back().map(|x| *x.0) {
                channel.remove(&k);
            }
        }

        channel.insert(event.0.id, Arc::new(From::from(event.0.clone())));

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<MessageDelete> for InMemoryCache {
    async fn update(&self, event: &MessageDelete) -> Result<(), InMemoryCacheError> {
        if !guard(self, EventType::MESSAGE_DELETE) {
            return Ok(());
        }

        let mut channels = self.0.messages.lock().await;
        let channel = channels.entry(event.channel_id).or_default();
        channel.remove(&event.id);

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<MessageDeleteBulk> for InMemoryCache {
    async fn update(&self, event: &MessageDeleteBulk) -> Result<(), InMemoryCacheError> {
        if !guard(self, EventType::MESSAGE_DELETE_BULK) {
            return Ok(());
        }

        let mut channels = self.0.messages.lock().await;
        let channel = channels.entry(event.channel_id).or_default();

        for id in &event.ids {
            channel.remove(id);
        }

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<MessageUpdate> for InMemoryCache {
    async fn update(&self, event: &MessageUpdate) -> Result<(), InMemoryCacheError> {
        if !guard(self, EventType::MESSAGE_UPDATE) {
            return Ok(());
        }

        let mut channels = self.0.messages.lock().await;
        let channel = channels.entry(event.channel_id).or_default();

        if let Some(mut message) = channel.get_mut(&event.id) {
            let mut msg = Arc::make_mut(&mut message);

            if let Some(attachments) = &event.attachments {
                msg.attachments = attachments.clone();
            }

            if let Some(content) = &event.content {
                msg.content = content.clone();
            }

            if let Some(edited_timestamp) = &event.edited_timestamp {
                msg.edited_timestamp.replace(edited_timestamp.clone());
            }

            if let Some(embeds) = &event.embeds {
                msg.embeds = embeds.clone();
            }

            if let Some(mention_everyone) = event.mention_everyone {
                msg.mention_everyone = mention_everyone;
            }

            if let Some(mention_roles) = &event.mention_roles {
                msg.mention_roles = mention_roles.clone();
            }

            if let Some(mentions) = &event.mentions {
                msg.mentions = mentions.iter().map(|x| x.id).collect::<Vec<_>>();
            }

            if let Some(pinned) = event.pinned {
                msg.pinned = pinned;
            }

            if let Some(timestamp) = &event.timestamp {
                msg.timestamp = timestamp.clone();
            }

            if let Some(tts) = event.tts {
                msg.tts = tts;
            }
        }

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<PresenceUpdate> for InMemoryCache {
    async fn update(&self, _: &PresenceUpdate) -> Result<(), InMemoryCacheError> {
        if !guard(self, EventType::PRESENCE_UPDATE) {
            return Ok(());
        }

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<ReactionAdd> for InMemoryCache {
    async fn update(&self, event: &ReactionAdd) -> Result<(), InMemoryCacheError> {
        if !guard(self, EventType::REACTION_ADD) {
            return Ok(());
        }

        let mut channels = self.0.messages.lock().await;
        let channel = channels.entry(event.0.channel_id).or_default();

        let mut message = match channel.get_mut(&event.0.message_id) {
            Some(message) => message,
            None => return Ok(()),
        };

        let msg = Arc::make_mut(&mut message);

        if let Some(reaction) = msg.reactions.iter_mut().find(|r| r.emoji == event.0.emoji) {
            if !reaction.me {
                if let Some(current_user) = self.current_user().await? {
                    if current_user.id == event.0.user_id {
                        reaction.me = true;
                    }
                }
            }

            reaction.count += 1;
        } else {
            let mut me = false;

            if let Some(current_user) = self.current_user().await? {
                if current_user.id == event.0.user_id {
                    me = true;
                }
            }

            msg.reactions.push(MessageReaction {
                count: 1,
                emoji: event.0.emoji.clone(),
                me,
            });
        }

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<ReactionRemove> for InMemoryCache {
    async fn update(&self, _: &ReactionRemove) -> Result<(), InMemoryCacheError> {
        if !guard(self, EventType::REACTION_REMOVE) {
            return Ok(());
        }

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<ReactionRemoveAll> for InMemoryCache {
    async fn update(&self, _: &ReactionRemoveAll) -> Result<(), InMemoryCacheError> {
        if !guard(self, EventType::REACTION_REMOVE_ALL) {
            return Ok(());
        }

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<Ready> for InMemoryCache {
    async fn update(&self, event: &Ready) -> Result<(), InMemoryCacheError> {
        if !guard(self, EventType::READY) {
            return Ok(());
        }

        self.cache_current_user(event.user.clone()).await;

        for status in event.guilds.values() {
            match status {
                GuildStatus::Offline(u) => {
                    self.unavailable_guild(u.id).await;
                },
                GuildStatus::Online(g) => {
                    self.cache_guild(g.clone()).await;
                },
            }
        }

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<RoleCreate> for InMemoryCache {
    async fn update(&self, event: &RoleCreate) -> Result<(), InMemoryCacheError> {
        if !guard(self, EventType::ROLE_CREATE) {
            return Ok(());
        }

        super::upsert_guild_item(
            &self.0.roles,
            event.guild_id,
            event.role.id,
            event.role.clone(),
        )
        .await;

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<RoleDelete> for InMemoryCache {
    async fn update(&self, event: &RoleDelete) -> Result<(), InMemoryCacheError> {
        if !guard(self, EventType::ROLE_DELETE) {
            return Ok(());
        }

        self.delete_role(event.role_id).await;

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<RoleUpdate> for InMemoryCache {
    async fn update(&self, event: &RoleUpdate) -> Result<(), InMemoryCacheError> {
        if !guard(self, EventType::ROLE_UPDATE) {
            return Ok(());
        }

        self.cache_role(event.guild_id, event.role.clone()).await;

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<TypingStart> for InMemoryCache {
    async fn update(&self, _: &TypingStart) -> Result<(), InMemoryCacheError> {
        Ok(())
    }
}

#[async_trait]
impl UpdateCache<UnavailableGuild> for InMemoryCache {
    async fn update(&self, event: &UnavailableGuild) -> Result<(), InMemoryCacheError> {
        if !guard(self, EventType::UNAVAILABLE_GUILD) {
            return Ok(());
        }

        self.0.guilds.lock().await.remove(&event.id);
        self.0.unavailable_guilds.lock().await.insert(event.id);

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<UpdateVoiceState> for InMemoryCache {
    async fn update(&self, _: &UpdateVoiceState) -> Result<(), InMemoryCacheError> {
        if !guard(self, EventType::UPDATE_VOICE_STATE) {
            return Ok(());
        }

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<UserUpdate> for InMemoryCache {
    async fn update(&self, event: &UserUpdate) -> Result<(), InMemoryCacheError> {
        if !guard(self, EventType::USER_UPDATE) {
            return Ok(());
        }

        self.cache_current_user(event.0.clone()).await;

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<VoiceServerUpdate> for InMemoryCache {
    async fn update(&self, _: &VoiceServerUpdate) -> Result<(), InMemoryCacheError> {
        if !guard(self, EventType::VOICE_SERVER_UPDATE) {
            return Ok(());
        }

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<VoiceStateUpdate> for InMemoryCache {
    async fn update(&self, _: &VoiceStateUpdate) -> Result<(), InMemoryCacheError> {
        if !guard(self, EventType::VOICE_STATE_UPDATE) {
            return Ok(());
        }

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<WebhookUpdate> for InMemoryCache {
    async fn update(&self, _: &WebhookUpdate) -> Result<(), InMemoryCacheError> {
        Ok(())
    }
}
