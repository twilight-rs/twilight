use super::{config::EventType, InMemoryCache, InMemoryCacheError};
use async_trait::async_trait;
use futures::lock::Mutex;
#[allow(unused_imports)]
use log::debug;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    sync::Arc,
};
use twilight_cache_trait::UpdateCache;
use twilight_model::{
    channel::{message::MessageReaction, Channel},
    gateway::{payload::*, presence::Presence},
    guild::GuildStatus,
    id::GuildId,
};

fn guard(this: &InMemoryCache, event_type: EventType) -> bool {
    this.0.config.event_types().contains(event_type)
}

#[async_trait]
impl UpdateCache<InMemoryCache, InMemoryCacheError> for BanAdd {
    async fn update(&self, _: &InMemoryCache) -> Result<(), InMemoryCacheError> {
        Ok(())
    }
}

#[async_trait]
impl UpdateCache<InMemoryCache, InMemoryCacheError> for BanRemove {
    async fn update(&self, _: &InMemoryCache) -> Result<(), InMemoryCacheError> {
        Ok(())
    }
}

#[async_trait]
impl UpdateCache<InMemoryCache, InMemoryCacheError> for ChannelCreate {
    async fn update(&self, cache: &InMemoryCache) -> Result<(), InMemoryCacheError> {
        if !guard(cache, EventType::CHANNEL_CREATE) {
            return Ok(());
        }

        match self.0.clone() {
            Channel::Group(c) => {
                super::upsert_item(&cache.0.groups, c.id, c).await;
            }
            Channel::Guild(c) => {
                if let Some(gid) = super::guild_channel_guild_id(&c) {
                    cache.cache_guild_channel(*gid, c.clone()).await;
                }
            }
            Channel::Private(c) => {
                cache.cache_private_channel(c.clone()).await;
            }
        }

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<InMemoryCache, InMemoryCacheError> for ChannelDelete {
    async fn update(&self, cache: &InMemoryCache) -> Result<(), InMemoryCacheError> {
        if !guard(cache, EventType::CHANNEL_DELETE) {
            return Ok(());
        }

        match self.0 {
            Channel::Group(ref c) => {
                cache.delete_group(c.id).await;
            }
            Channel::Guild(ref c) => {
                let id = *super::guild_channel_id(&c);

                cache.delete_guild_channel(id).await;
            }
            Channel::Private(ref c) => {
                cache.0.channels_private.lock().await.remove(&c.id);
            }
        }

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<InMemoryCache, InMemoryCacheError> for ChannelPinsUpdate {
    async fn update(&self, _: &InMemoryCache) -> Result<(), InMemoryCacheError> {
        Ok(())
    }
}

#[async_trait]
impl UpdateCache<InMemoryCache, InMemoryCacheError> for ChannelUpdate {
    async fn update(&self, cache: &InMemoryCache) -> Result<(), InMemoryCacheError> {
        if !guard(cache, EventType::CHANNEL_UPDATE) {
            return Ok(());
        }

        match self.0.clone() {
            Channel::Group(c) => {
                cache.cache_group(c.clone()).await;
            }
            Channel::Guild(c) => {
                if let Some(gid) = super::guild_channel_guild_id(&c) {
                    cache.cache_guild_channel(*gid, c.clone()).await;
                }
            }
            Channel::Private(c) => {
                cache.cache_private_channel(c.clone()).await;
            }
        }

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<InMemoryCache, InMemoryCacheError> for GuildCreate {
    async fn update(&self, cache: &InMemoryCache) -> Result<(), InMemoryCacheError> {
        if !guard(cache, EventType::GUILD_CREATE) {
            return Ok(());
        }

        cache.cache_guild(self.0.clone()).await;

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<InMemoryCache, InMemoryCacheError> for GuildDelete {
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

        let id = self.id;

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
impl UpdateCache<InMemoryCache, InMemoryCacheError> for GuildEmojisUpdate {
    async fn update(&self, cache: &InMemoryCache) -> Result<(), InMemoryCacheError> {
        if !guard(cache, EventType::GUILD_EMOJIS_UPDATE) {
            return Ok(());
        }

        cache
            .cache_emojis(self.guild_id, self.emojis.values().cloned())
            .await;

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<InMemoryCache, InMemoryCacheError> for GuildIntegrationsUpdate {
    async fn update(&self, _: &InMemoryCache) -> Result<(), InMemoryCacheError> {
        Ok(())
    }
}

#[async_trait]
impl UpdateCache<InMemoryCache, InMemoryCacheError> for GuildUpdate {
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
        guild.max_presences = Some(g.max_presences.unwrap_or(25000));
        guild.mfa_level = g.mfa_level;
        guild.name = g.name.clone();
        guild.owner = g.owner;
        guild.owner_id = g.owner_id;
        guild.permissions = g.permissions;
        guild.preferred_locale = g.preferred_locale.clone();
        guild.premium_tier = g.premium_tier;
        guild
            .premium_subscription_count
            .replace(g.premium_subscription_count.unwrap_or(0));
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
impl UpdateCache<InMemoryCache, InMemoryCacheError> for MemberAdd {
    async fn update(&self, cache: &InMemoryCache) -> Result<(), InMemoryCacheError> {
        if !guard(cache, EventType::MEMBER_ADD) {
            return Ok(());
        }

        cache.cache_member(self.guild_id, self.0.clone()).await;

        let mut guild = cache.0.guild_members.lock().await;
        guild
            .entry(self.guild_id)
            .or_default()
            .insert(self.0.user.id);

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<InMemoryCache, InMemoryCacheError> for MemberChunk {
    async fn update(&self, cache: &InMemoryCache) -> Result<(), InMemoryCacheError> {
        if !guard(cache, EventType::MEMBER_CHUNK) {
            return Ok(());
        }

        if self.members.is_empty() {
            return Ok(());
        }

        cache
            .cache_members(self.guild_id, self.members.values().cloned())
            .await;
        let user_ids = self.members.keys();
        let mut members = cache.0.guild_members.lock().await;

        let guild = members.entry(self.guild_id).or_default();

        for id in user_ids {
            guild.insert(*id);
        }

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<InMemoryCache, InMemoryCacheError> for MemberRemove {
    async fn update(&self, cache: &InMemoryCache) -> Result<(), InMemoryCacheError> {
        if !guard(cache, EventType::MEMBER_REMOVE) {
            return Ok(());
        }

        cache
            .0
            .members
            .lock()
            .await
            .remove(&(self.guild_id, self.user.id));

        let mut guild_members = cache.0.guild_members.lock().await;

        if let Some(members) = guild_members.get_mut(&self.guild_id) {
            members.remove(&self.user.id);
        }

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<InMemoryCache, InMemoryCacheError> for MemberUpdate {
    async fn update(&self, cache: &InMemoryCache) -> Result<(), InMemoryCacheError> {
        if !guard(cache, EventType::MEMBER_UPDATE) {
            return Ok(());
        }

        let mut members = cache.0.members.lock().await;

        let mut member = match members.get_mut(&(self.guild_id, self.user.id)) {
            Some(member) => member,
            None => return Ok(()),
        };
        let mut member = Arc::make_mut(&mut member);

        member.nick = self.nick.clone();
        member.roles = self.roles.clone();

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<InMemoryCache, InMemoryCacheError> for MessageCreate {
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
impl UpdateCache<InMemoryCache, InMemoryCacheError> for MessageDelete {
    async fn update(&self, cache: &InMemoryCache) -> Result<(), InMemoryCacheError> {
        if !guard(cache, EventType::MESSAGE_DELETE) {
            return Ok(());
        }

        let mut channels = cache.0.messages.lock().await;
        let channel = channels.entry(self.channel_id).or_default();
        channel.remove(&self.id);

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<InMemoryCache, InMemoryCacheError> for MessageDeleteBulk {
    async fn update(&self, cache: &InMemoryCache) -> Result<(), InMemoryCacheError> {
        if !guard(cache, EventType::MESSAGE_DELETE_BULK) {
            return Ok(());
        }

        let mut channels = cache.0.messages.lock().await;
        let channel = channels.entry(self.channel_id).or_default();

        for id in &self.ids {
            channel.remove(id);
        }

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<InMemoryCache, InMemoryCacheError> for MessageUpdate {
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
impl UpdateCache<InMemoryCache, InMemoryCacheError> for PresenceUpdate {
    async fn update(&self, cache: &InMemoryCache) -> Result<(), InMemoryCacheError> {
        if !guard(cache, EventType::PRESENCE_UPDATE) {
            return Ok(());
        }

        let presence = Presence {
            activities: self.activities.clone(),
            client_status: self.client_status.clone(),
            game: self.game.clone(),
            guild_id: self.guild_id,
            nick: self.nick.clone(),
            status: self.status,
            user: self.user.clone(),
        };

        cache.cache_presence(self.guild_id, presence).await;

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<InMemoryCache, InMemoryCacheError> for ReactionAdd {
    async fn update(&self, cache: &InMemoryCache) -> Result<(), InMemoryCacheError> {
        if !guard(cache, EventType::REACTION_ADD) {
            return Ok(());
        }

        let mut channels = cache.0.messages.lock().await;
        let channel = channels.entry(self.0.channel_id).or_default();

        let mut message = match channel.get_mut(&self.0.message_id) {
            Some(message) => message,
            None => return Ok(()),
        };

        let msg = Arc::make_mut(&mut message);

        if let Some(reaction) = msg.reactions.iter_mut().find(|r| r.emoji == self.0.emoji) {
            if !reaction.me {
                if let Some(current_user) = cache.current_user().await? {
                    if current_user.id == self.0.user_id {
                        reaction.me = true;
                    }
                }
            }

            reaction.count += 1;
        } else {
            let mut me = false;

            if let Some(current_user) = cache.current_user().await? {
                if current_user.id == self.0.user_id {
                    me = true;
                }
            }

            msg.reactions.push(MessageReaction {
                count: 1,
                emoji: self.0.emoji.clone(),
                me,
            });
        }

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<InMemoryCache, InMemoryCacheError> for ReactionRemove {
    async fn update(&self, cache: &InMemoryCache) -> Result<(), InMemoryCacheError> {
        if !guard(cache, EventType::REACTION_REMOVE) {
            return Ok(());
        }

        let mut channels = cache.0.messages.lock().await;
        let channel = channels.entry(self.0.channel_id).or_default();

        let mut message = match channel.get_mut(&self.0.message_id) {
            Some(message) => message,
            None => return Ok(()),
        };

        let msg = Arc::make_mut(&mut message);

        if let Some(reaction) = msg.reactions.iter_mut().find(|r| r.emoji == self.0.emoji) {
            if reaction.me {
                if let Some(current_user) = cache.current_user().await? {
                    if current_user.id == self.0.user_id {
                        reaction.me = false;
                    }
                }
            }

            if reaction.count > 1 {
                reaction.count -= 1;
            } else {
                msg.reactions.retain(|e| !(e.emoji == self.0.emoji.clone()));
            }
        }

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<InMemoryCache, InMemoryCacheError> for ReactionRemoveAll {
    async fn update(&self, cache: &InMemoryCache) -> Result<(), InMemoryCacheError> {
        if !guard(cache, EventType::REACTION_REMOVE_ALL) {
            return Ok(());
        }

        let mut channels = cache.0.messages.lock().await;
        let channel = channels.entry(self.channel_id).or_default();

        let mut message = match channel.get_mut(&self.message_id) {
            Some(message) => message,
            None => return Ok(()),
        };

        let msg = Arc::make_mut(&mut message);

        msg.reactions.clear();

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<InMemoryCache, InMemoryCacheError> for Ready {
    async fn update(&self, cache: &InMemoryCache) -> Result<(), InMemoryCacheError> {
        if !guard(cache, EventType::READY) {
            return Ok(());
        }

        cache.cache_current_user(self.user.clone()).await;

        for status in self.guilds.values() {
            match status {
                GuildStatus::Offline(u) => {
                    cache.unavailable_guild(u.id).await;
                }
                GuildStatus::Online(g) => {
                    cache.cache_guild(g.clone()).await;
                }
            }
        }

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<InMemoryCache, InMemoryCacheError> for RoleCreate {
    async fn update(&self, cache: &InMemoryCache) -> Result<(), InMemoryCacheError> {
        if !guard(cache, EventType::ROLE_CREATE) {
            return Ok(());
        }

        super::upsert_guild_item(
            &cache.0.roles,
            self.guild_id,
            self.role.id,
            self.role.clone(),
        )
        .await;

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<InMemoryCache, InMemoryCacheError> for RoleDelete {
    async fn update(&self, cache: &InMemoryCache) -> Result<(), InMemoryCacheError> {
        if !guard(cache, EventType::ROLE_DELETE) {
            return Ok(());
        }

        cache.delete_role(self.role_id).await;

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<InMemoryCache, InMemoryCacheError> for RoleUpdate {
    async fn update(&self, cache: &InMemoryCache) -> Result<(), InMemoryCacheError> {
        if !guard(cache, EventType::ROLE_UPDATE) {
            return Ok(());
        }

        cache.cache_role(self.guild_id, self.role.clone()).await;

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<InMemoryCache, InMemoryCacheError> for TypingStart {
    async fn update(&self, _: &InMemoryCache) -> Result<(), InMemoryCacheError> {
        Ok(())
    }
}

#[async_trait]
impl UpdateCache<InMemoryCache, InMemoryCacheError> for UnavailableGuild {
    async fn update(&self, cache: &InMemoryCache) -> Result<(), InMemoryCacheError> {
        if !guard(cache, EventType::UNAVAILABLE_GUILD) {
            return Ok(());
        }

        cache.0.guilds.lock().await.remove(&self.id);
        cache.0.unavailable_guilds.lock().await.insert(self.id);

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<InMemoryCache, InMemoryCacheError> for UpdateVoiceState {
    async fn update(&self, cache: &InMemoryCache) -> Result<(), InMemoryCacheError> {
        if !guard(cache, EventType::UPDATE_VOICE_STATE) {
            return Ok(());
        }

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<InMemoryCache, InMemoryCacheError> for UserUpdate {
    async fn update(&self, cache: &InMemoryCache) -> Result<(), InMemoryCacheError> {
        if !guard(cache, EventType::USER_UPDATE) {
            return Ok(());
        }

        cache.cache_current_user(self.0.clone()).await;

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<InMemoryCache, InMemoryCacheError> for VoiceServerUpdate {
    async fn update(&self, cache: &InMemoryCache) -> Result<(), InMemoryCacheError> {
        if !guard(cache, EventType::VOICE_SERVER_UPDATE) {
            return Ok(());
        }

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<InMemoryCache, InMemoryCacheError> for VoiceStateUpdate {
    async fn update(&self, cache: &InMemoryCache) -> Result<(), InMemoryCacheError> {
        if !guard(cache, EventType::VOICE_STATE_UPDATE) {
            return Ok(());
        }

        cache.cache_voice_state(self.0.clone()).await;

        Ok(())
    }
}

#[async_trait]
impl UpdateCache<InMemoryCache, InMemoryCacheError> for WebhooksUpdate {
    async fn update(&self, _: &InMemoryCache) -> Result<(), InMemoryCacheError> {
        Ok(())
    }
}
