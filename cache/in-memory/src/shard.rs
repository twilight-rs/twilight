use super::{config::EventType, InMemoryCache, InMemoryCacheError};
use async_trait::async_trait;
use dawn_cache_trait::UpdateCache;
use dawn_gateway::shard::event::Event;
use dawn_model::{gateway::payload::*, guild::GuildStatus, id::GuildId};
use futures_util::lock::Mutex;
#[allow(unused_imports)]
use log::debug;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    sync::Arc,
};

fn guard(this: &InMemoryCache, event_type: EventType) -> bool {
    this.0.config.event_types().contains(event_type)
}

#[async_trait]
impl UpdateCache<InMemoryCache, InMemoryCacheError> for Box<GuildCreate> {
    async fn update(&self, cache: &InMemoryCache) -> Result<(), InMemoryCacheError> {
        debug!("Caching New Guild");
        if !guard(cache, EventType::GUILD_CREATE) {
            return Ok(());
        }

        cache.cache_guild(self.0.clone()).await;

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<InMemoryCache, InMemoryCacheError> for Box<GuildDelete> {
    async fn update(&self, cache: &InMemoryCache) -> Result<(), InMemoryCacheError> {
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

        if !guard(cache, EventType::GUILD_DELETE) {
            return Ok(());
        }

        let id = self.guild.id;

        cache.0.guilds.lock().await.remove(&id);

        remove_ids(&cache.0.guild_channels, &cache.0.channels_guild, id).await;
        remove_ids(&cache.0.guild_emojis, &cache.0.emojis, id).await;
        remove_ids(&cache.0.guild_roles, &cache.0.roles, id).await;
        remove_ids(&cache.0.guild_voice_states, &cache.0.voice_states, id).await;

        if let Some(ids) = cache.0.guild_members.lock().await.remove(&id) {
            let mut members = cache.0.members.lock().await;

            for user_id in ids {
                members.remove(&(id, user_id));
            }
        }

        if let Some(ids) = cache.0.guild_presences.lock().await.remove(&id) {
            let mut presences = cache.0.presences.lock().await;

            for user_id in ids {
                presences.remove(&(Some(id), user_id));
            }
        }

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<InMemoryCache, InMemoryCacheError> for Box<GuildUpdate> {
    async fn update(&self, cache: &InMemoryCache) -> Result<(), InMemoryCacheError> {
        if !guard(cache, EventType::GUILD_UPDATE) {
            return Ok(());
        }

        let mut guilds = cache.0.guilds.lock().await;

        let mut guild = match guilds.get_mut(&self.0.id).cloned() {
            Some(guild) => guild,
            None => return Ok(()),
        };

        let g = &self.0;

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
impl UpdateCache<InMemoryCache, InMemoryCacheError> for Box<MemberAdd> {
    async fn update(&self, cache: &InMemoryCache) -> Result<(), InMemoryCacheError> {
        if !guard(cache, EventType::MEMBER_ADD) {
            return Ok(());
        }

        // This will always be present on members from the gateway.
        let guild_id = match self.guild_id {
            Some(guild_id) => guild_id,
            None => return Ok(()),
        };

        cache.cache_member(guild_id, self.0.clone()).await;

        let mut guild = cache.0.guild_members.lock().await;
        guild.entry(guild_id).or_default().insert(self.0.user.id);

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<InMemoryCache, InMemoryCacheError> for Box<MessageCreate> {
    async fn update(&self, cache: &InMemoryCache) -> Result<(), InMemoryCacheError> {
        if !guard(cache, EventType::MESSAGE_CREATE) {
            return Ok(());
        }

        let mut channels = cache.0.messages.lock().await;
        let channel = channels.entry(self.0.channel_id).or_default();

        if channel.len() > cache.0.config.message_cache_size() {
            if let Some(k) = channel.iter().next_back().map(|x| *x.0) {
                channel.remove(&k);
            }
        }

        channel.insert(self.0.id, Arc::new(From::from(self.0.clone())));

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<InMemoryCache, InMemoryCacheError> for Box<MessageUpdate> {
    async fn update(&self, cache: &InMemoryCache) -> Result<(), InMemoryCacheError> {
        if !guard(cache, EventType::MESSAGE_UPDATE) {
            return Ok(());
        }

        let mut channels = cache.0.messages.lock().await;
        let channel = channels.entry(self.channel_id).or_default();

        if let Some(mut message) = channel.get_mut(&self.id) {
            let mut msg = Arc::make_mut(&mut message);

            if let Some(attachments) = &self.attachments {
                msg.attachments = attachments.clone();
            }

            if let Some(content) = &self.content {
                msg.content = content.clone();
            }

            if let Some(edited_timestamp) = &self.edited_timestamp {
                msg.edited_timestamp.replace(edited_timestamp.clone());
            }

            if let Some(embeds) = &self.embeds {
                msg.embeds = embeds.clone();
            }

            if let Some(mention_everyone) = self.mention_everyone {
                msg.mention_everyone = mention_everyone;
            }

            if let Some(mention_roles) = &self.mention_roles {
                msg.mention_roles = mention_roles.clone();
            }

            if let Some(mentions) = &self.mentions {
                msg.mentions = mentions.iter().map(|x| x.id).collect::<Vec<_>>();
            }

            if let Some(pinned) = self.pinned {
                msg.pinned = pinned;
            }

            if let Some(timestamp) = &self.timestamp {
                msg.timestamp = timestamp.clone();
            }

            if let Some(tts) = self.tts {
                msg.tts = tts;
            }
        }

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<InMemoryCache, InMemoryCacheError> for Box<PresenceUpdate> {
    async fn update(&self, cache: &InMemoryCache) -> Result<(), InMemoryCacheError> {
        if !guard(cache, EventType::PRESENCE_UPDATE) {
            return Ok(());
        }

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<InMemoryCache, InMemoryCacheError> for Box<Ready> {
    async fn update(&self, cache: &InMemoryCache) -> Result<(), InMemoryCacheError> {
        if !guard(cache, EventType::READY) {
            return Ok(());
        }

        cache.cache_current_user(self.user.clone()).await;

        for status in self.guilds.values() {
            match status {
                GuildStatus::Offline(u) => {
                    cache.unavailable_guild(u.id).await;
                },
                GuildStatus::Online(g) => {
                    cache.cache_guild(g.clone()).await;
                },
            }
        }

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<InMemoryCache, InMemoryCacheError> for Box<VoiceStateUpdate> {
    async fn update(&self, cache: &InMemoryCache) -> Result<(), InMemoryCacheError> {
        if !guard(cache, EventType::VOICE_STATE_UPDATE) {
            return Ok(());
        }

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<InMemoryCache, InMemoryCacheError> for Event {
    async fn update(&self, c: &InMemoryCache) -> Result<(), InMemoryCacheError> {
        use dawn_gateway::shard::event::Event::*;

        match self {
            BanAdd(v) => c.update(v).await,
            BanRemove(v) => c.update(v).await,
            ChannelCreate(v) => c.update(v).await,
            ChannelDelete(v) => c.update(v).await,
            ChannelPinsUpdate(v) => c.update(v).await,
            ChannelUpdate(v) => c.update(v).await,
            GatewayHeartbeat(_v) => Ok(()),
            GatewayHeartbeatAck => Ok(()),
            GatewayHello(_v) => Ok(()),
            GatewayInvalidateSession(_v) => Ok(()),
            GatewayReconnect => Ok(()),
            GuildCreate(v) => c.update(v).await,
            GuildDelete(v) => c.update(v).await,
            GuildEmojisUpdate(v) => c.update(v).await,
            GuildIntegrationsUpdate(v) => c.update(v).await,
            GuildUpdate(v) => c.update(v).await,
            MemberAdd(v) => c.update(v).await,
            MemberRemove(v) => c.update(v).await,
            MemberUpdate(v) => c.update(v).await,
            MemberChunk(v) => c.update(v).await,
            MessageCreate(v) => c.update(v).await,
            MessageDelete(v) => c.update(v).await,
            MessageDeleteBulk(v) => c.update(v).await,
            MessageUpdate(v) => c.update(v).await,
            PresenceUpdate(v) => c.update(v).await,
            PresencesReplace => Ok(()),
            ReactionAdd(v) => c.update(v).await,
            ReactionRemove(v) => c.update(v).await,
            ReactionRemoveAll(v) => c.update(v).await,
            Ready(v) => c.update(v).await,
            Resumed => Ok(()),
            RoleCreate(v) => c.update(v).await,
            RoleDelete(v) => c.update(v).await,
            RoleUpdate(v) => c.update(v).await,
            ShardConnected(_v) => Ok(()),
            ShardConnecting(_v) => Ok(()),
            ShardDisconnected(_v) => Ok(()),
            ShardIdentifying(_v) => Ok(()),
            ShardReconnecting(_v) => Ok(()),
            ShardPayload(_v) => Ok(()),
            ShardResuming(_v) => Ok(()),
            TypingStart(v) => c.update(v).await,
            UnavailableGuild(v) => c.update(v).await,
            UserUpdate(v) => c.update(v).await,
            VoiceServerUpdate(v) => c.update(v).await,
            VoiceStateUpdate(v) => c.update(v).await,
            WebhookUpdate(v) => c.update(v).await,
        }
    }
}
