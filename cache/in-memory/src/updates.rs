use super::{config::EventType, InMemoryCache};
use dashmap::DashMap;
use std::{collections::HashSet, hash::Hash, ops::Deref, sync::Arc};
use twilight_model::{
    channel::{message::MessageReaction, Channel, GuildChannel},
    gateway::{event::Event, payload::*, presence::Presence},
    guild::GuildStatus,
    id::GuildId,
};

fn guard(this: &InMemoryCache, event_type: EventType) -> bool {
    this.0.config.event_types().contains(event_type)
}

pub trait UpdateCache {
    // Allow this for presentation purposes in documentation.
    #[allow(unused_variables)]
    fn update(&self, cache: &InMemoryCache) {}
}

impl UpdateCache for Event {
    #[allow(clippy::cognitive_complexity)]
    fn update(&self, c: &InMemoryCache) {
        use Event::*;

        match self {
            BanAdd(_) => {}
            BanRemove(_) => {}
            ChannelCreate(v) => c.update(v),
            ChannelDelete(v) => c.update(v),
            ChannelPinsUpdate(v) => c.update(v),
            ChannelUpdate(v) => c.update(v),
            GatewayHeartbeat(_) => {}
            GatewayHeartbeatAck => {}
            GatewayHello(_) => {}
            GatewayInvalidateSession(_v) => {}
            GatewayReconnect => {}
            GiftCodeUpdate => {}
            GuildCreate(v) => c.update(v.deref()),
            GuildDelete(v) => c.update(v.deref()),
            GuildEmojisUpdate(v) => c.update(v),
            GuildIntegrationsUpdate(v) => c.update(v),
            GuildUpdate(v) => c.update(v.deref()),
            InviteCreate(_) => {}
            InviteDelete(_) => {}
            MemberAdd(v) => c.update(v.deref()),
            MemberRemove(v) => c.update(v),
            MemberUpdate(v) => c.update(v.deref()),
            MemberChunk(v) => c.update(v),
            MessageCreate(v) => c.update(v.deref()),
            MessageDelete(v) => c.update(v),
            MessageDeleteBulk(v) => c.update(v),
            MessageUpdate(v) => c.update(v.deref()),
            PresenceUpdate(v) => c.update(v.deref()),
            PresencesReplace => {}
            ReactionAdd(v) => c.update(v.deref()),
            ReactionRemove(v) => c.update(v.deref()),
            ReactionRemoveAll(v) => c.update(v),
            ReactionRemoveEmoji(_) => {}
            Ready(v) => c.update(v.deref()),
            Resumed => {}
            RoleCreate(v) => c.update(v),
            RoleDelete(v) => c.update(v),
            RoleUpdate(v) => c.update(v),
            ShardConnected(_) => {}
            ShardConnecting(_) => {}
            ShardDisconnected(_) => {}
            ShardIdentifying(_) => {}
            ShardReconnecting(_) => {}
            ShardPayload(_) => {}
            ShardResuming(_) => {}
            TypingStart(v) => c.update(v.deref()),
            UnavailableGuild(v) => c.update(v),
            UserUpdate(v) => c.update(v),
            VoiceServerUpdate(v) => c.update(v),
            VoiceStateUpdate(v) => c.update(v.deref()),
            WebhooksUpdate(v) => c.update(v),
        }
    }
}

impl UpdateCache for BanAdd {}

impl UpdateCache for BanRemove {}

impl UpdateCache for ChannelCreate {
    fn update(&self, cache: &InMemoryCache) {
        if !guard(cache, EventType::CHANNEL_CREATE) {
            return;
        }

        match &self.0 {
            Channel::Group(c) => {
                super::upsert_item(&cache.0.groups, c.id, c.clone());
            }
            Channel::Guild(c) => {
                if let Some(gid) = c.guild_id() {
                    cache.cache_guild_channel(gid, c.clone());
                }
            }
            Channel::Private(c) => {
                cache.cache_private_channel(c.clone());
            }
        }
    }
}

impl UpdateCache for ChannelDelete {
    fn update(&self, cache: &InMemoryCache) {
        if !guard(cache, EventType::CHANNEL_DELETE) {
            return;
        }

        match self.0 {
            Channel::Group(ref c) => {
                cache.delete_group(c.id);
            }
            Channel::Guild(ref c) => {
                cache.delete_guild_channel(c.id());
            }
            Channel::Private(ref c) => {
                cache.0.channels_private.remove(&c.id);
            }
        }
    }
}

impl UpdateCache for ChannelPinsUpdate {
    fn update(&self, cache: &InMemoryCache) {
        if !guard(cache, EventType::CHANNEL_PINS_UPDATE) {
            return;
        }

        if let Some(mut item) = cache.0.channels_guild.get_mut(&self.channel_id) {
            let channel = Arc::make_mut(&mut item.data);

            if let GuildChannel::Text(text) = channel {
                text.last_pin_timestamp = self.last_pin_timestamp.clone();
            }

            return;
        }

        if let Some(mut channel) = cache.0.channels_private.get_mut(&self.channel_id) {
            Arc::make_mut(&mut channel).last_pin_timestamp = self.last_pin_timestamp.clone();

            return;
        }

        if let Some(mut group) = cache.0.groups.get_mut(&self.channel_id) {
            Arc::make_mut(&mut group).last_pin_timestamp = self.last_pin_timestamp.clone();
        }
    }
}

impl UpdateCache for ChannelUpdate {
    fn update(&self, cache: &InMemoryCache) {
        if !guard(cache, EventType::CHANNEL_UPDATE) {
            return;
        }

        match self.0.clone() {
            Channel::Group(c) => {
                cache.cache_group(c);
            }
            Channel::Guild(c) => {
                if let Some(gid) = c.guild_id() {
                    cache.cache_guild_channel(gid, c);
                }
            }
            Channel::Private(c) => {
                cache.cache_private_channel(c);
            }
        }
    }
}

impl UpdateCache for GuildCreate {
    fn update(&self, cache: &InMemoryCache) {
        if !guard(cache, EventType::GUILD_CREATE) {
            return;
        }

        cache.cache_guild(self.0.clone());
    }
}

impl UpdateCache for GuildDelete {
    fn update(&self, cache: &InMemoryCache) {
        fn remove_ids<T: Eq + Hash, U>(
            guild_map: &DashMap<GuildId, HashSet<T>>,
            container: &DashMap<T, U>,
            guild_id: GuildId,
        ) {
            if let Some((_, ids)) = guild_map.remove(&guild_id) {
                for id in ids {
                    container.remove(&id);
                }
            }
        }

        if !guard(cache, EventType::GUILD_DELETE) {
            return;
        }

        let id = self.id;

        cache.0.guilds.remove(&id);

        remove_ids(&cache.0.guild_channels, &cache.0.channels_guild, id);
        remove_ids(&cache.0.guild_emojis, &cache.0.emojis, id);
        remove_ids(&cache.0.guild_roles, &cache.0.roles, id);
        // Clear out a guilds voice states when a guild leaves
        cache.0.voice_state_guilds.remove(&id);

        if let Some((_, ids)) = cache.0.guild_members.remove(&id) {
            for user_id in ids {
                cache.0.members.remove(&(id, user_id));
            }
        }

        if let Some((_, ids)) = cache.0.guild_presences.remove(&id) {
            for user_id in ids {
                cache.0.presences.remove(&(id, user_id));
            }
        }
    }
}

impl UpdateCache for GuildEmojisUpdate {
    fn update(&self, cache: &InMemoryCache) {
        if !guard(cache, EventType::GUILD_EMOJIS_UPDATE) {
            return;
        }

        cache.cache_emojis(self.guild_id, self.emojis.values().cloned());
    }
}

impl UpdateCache for GuildIntegrationsUpdate {}

impl UpdateCache for GuildUpdate {
    fn update(&self, cache: &InMemoryCache) {
        if !guard(cache, EventType::GUILD_UPDATE) {
            return;
        }

        let mut guild = match cache
            .0
            .guilds
            .get_mut(&self.0.id)
            .map(|r| Arc::clone(r.value()))
        {
            Some(guild) => guild,
            None => return,
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
            .replace(g.premium_subscription_count.unwrap_or_default());
        guild.region = g.region.clone();
        guild.splash = g.splash.clone();
        guild.system_channel_id = g.system_channel_id;
        guild.verification_level = g.verification_level;
        guild.vanity_url_code = g.vanity_url_code.clone();
        guild.widget_channel_id = g.widget_channel_id;
        guild.widget_enabled = g.widget_enabled;
    }
}

impl UpdateCache for MemberAdd {
    fn update(&self, cache: &InMemoryCache) {
        if !guard(cache, EventType::MEMBER_ADD) {
            return;
        }

        cache.cache_member(self.guild_id, self.0.clone());

        cache
            .0
            .guild_members
            .entry(self.guild_id)
            .or_default()
            .insert(self.0.user.id);
    }
}

impl UpdateCache for MemberChunk {
    fn update(&self, cache: &InMemoryCache) {
        if !guard(cache, EventType::MEMBER_CHUNK) {
            return;
        }

        if self.members.is_empty() {
            return;
        }

        cache.cache_members(self.guild_id, self.members.values().cloned());
        let mut guild = cache.0.guild_members.entry(self.guild_id).or_default();
        guild.extend(self.members.keys());
    }
}

impl UpdateCache for MemberRemove {
    fn update(&self, cache: &InMemoryCache) {
        if !guard(cache, EventType::MEMBER_REMOVE) {
            return;
        }

        cache.0.members.remove(&(self.guild_id, self.user.id));

        if let Some(mut members) = cache.0.guild_members.get_mut(&self.guild_id) {
            members.remove(&self.user.id);
        }

        // Avoid a deadlock by mutating the user, dropping the lock to the map,
        // and then maybe conditionally removing the user later.
        let mut maybe_remove_user = false;

        if let Some(mut user_tuple) = cache.0.users.get_mut(&self.user.id) {
            user_tuple.1.remove(&self.guild_id);

            maybe_remove_user = true;
        }

        if maybe_remove_user {
            cache
                .0
                .users
                .remove_if(&self.user.id, |_, guild_set| guild_set.1.is_empty());
        }
    }
}

impl UpdateCache for MemberUpdate {
    fn update(&self, cache: &InMemoryCache) {
        if !guard(cache, EventType::MEMBER_UPDATE) {
            return;
        }

        let mut member = match cache.0.members.get_mut(&(self.guild_id, self.user.id)) {
            Some(member) => member,
            None => return,
        };
        let mut member = Arc::make_mut(&mut member);

        member.nick = self.nick.clone();
        member.roles = self.roles.clone();
        member.joined_at.replace(self.joined_at.clone());
    }
}

impl UpdateCache for MessageCreate {
    fn update(&self, cache: &InMemoryCache) {
        if !guard(cache, EventType::MESSAGE_CREATE) {
            return;
        }

        let mut channel = cache.0.messages.entry(self.0.channel_id).or_default();

        if channel.len() > cache.0.config.message_cache_size() {
            if let Some(k) = channel.iter().next_back().map(|x| *x.0) {
                channel.remove(&k);
            }
        }

        channel.insert(self.0.id, Arc::new(From::from(self.0.clone())));
    }
}

impl UpdateCache for MessageDelete {
    fn update(&self, cache: &InMemoryCache) {
        if !guard(cache, EventType::MESSAGE_DELETE) {
            return;
        }

        let mut channel = cache.0.messages.entry(self.channel_id).or_default();
        channel.remove(&self.id);
    }
}

impl UpdateCache for MessageDeleteBulk {
    fn update(&self, cache: &InMemoryCache) {
        if !guard(cache, EventType::MESSAGE_DELETE_BULK) {
            return;
        }

        let mut channel = cache.0.messages.entry(self.channel_id).or_default();

        for id in &self.ids {
            channel.remove(id);
        }
    }
}

impl UpdateCache for MessageUpdate {
    fn update(&self, cache: &InMemoryCache) {
        if !guard(cache, EventType::MESSAGE_UPDATE) {
            return;
        }

        let mut channel = cache.0.messages.entry(self.channel_id).or_default();

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
    }
}

impl UpdateCache for PresenceUpdate {
    fn update(&self, cache: &InMemoryCache) {
        if !guard(cache, EventType::PRESENCE_UPDATE) {
            return;
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

        cache.cache_presence(self.guild_id, presence);
    }
}

impl UpdateCache for ReactionAdd {
    fn update(&self, cache: &InMemoryCache) {
        if !guard(cache, EventType::REACTION_ADD) {
            return;
        }

        let mut channel = cache.0.messages.entry(self.0.channel_id).or_default();

        let mut message = match channel.get_mut(&self.0.message_id) {
            Some(message) => message,
            None => return,
        };

        let msg = Arc::make_mut(&mut message);

        if let Some(reaction) = msg.reactions.iter_mut().find(|r| r.emoji == self.0.emoji) {
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

            msg.reactions.push(MessageReaction {
                count: 1,
                emoji: self.0.emoji.clone(),
                me,
            });
        }
    }
}

impl UpdateCache for ReactionRemove {
    fn update(&self, cache: &InMemoryCache) {
        if !guard(cache, EventType::REACTION_REMOVE) {
            return;
        }

        let mut channel = cache.0.messages.entry(self.0.channel_id).or_default();

        let mut message = match channel.get_mut(&self.0.message_id) {
            Some(message) => message,
            None => return,
        };

        let msg = Arc::make_mut(&mut message);

        if let Some(reaction) = msg.reactions.iter_mut().find(|r| r.emoji == self.0.emoji) {
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
                msg.reactions.retain(|e| !(e.emoji == self.0.emoji));
            }
        }
    }
}

impl UpdateCache for ReactionRemoveAll {
    fn update(&self, cache: &InMemoryCache) {
        if !guard(cache, EventType::REACTION_REMOVE_ALL) {
            return;
        }

        let mut channel = cache.0.messages.entry(self.channel_id).or_default();

        let mut message = match channel.get_mut(&self.message_id) {
            Some(message) => message,
            None => return,
        };

        let msg = Arc::make_mut(&mut message);
        msg.reactions.clear();
    }
}

impl UpdateCache for Ready {
    fn update(&self, cache: &InMemoryCache) {
        if !guard(cache, EventType::READY) {
            return;
        }

        cache.cache_current_user(self.user.clone());

        for status in self.guilds.values() {
            match status {
                GuildStatus::Offline(u) => {
                    cache.unavailable_guild(u.id);
                }
                GuildStatus::Online(g) => {
                    cache.cache_guild(g.clone());
                }
            }
        }
    }
}

impl UpdateCache for RoleCreate {
    fn update(&self, cache: &InMemoryCache) {
        if !guard(cache, EventType::ROLE_CREATE) {
            return;
        }

        super::upsert_guild_item(
            &cache.0.roles,
            self.guild_id,
            self.role.id,
            self.role.clone(),
        );
    }
}

impl UpdateCache for RoleDelete {
    fn update(&self, cache: &InMemoryCache) {
        if !guard(cache, EventType::ROLE_DELETE) {
            return;
        }

        cache.delete_role(self.role_id);
    }
}

impl UpdateCache for RoleUpdate {
    fn update(&self, cache: &InMemoryCache) {
        if !guard(cache, EventType::ROLE_UPDATE) {
            return;
        }

        cache.cache_role(self.guild_id, self.role.clone());
    }
}

impl UpdateCache for TypingStart {}

impl UpdateCache for UnavailableGuild {
    fn update(&self, cache: &InMemoryCache) {
        if !guard(cache, EventType::UNAVAILABLE_GUILD) {
            return;
        }

        cache.0.guilds.remove(&self.id);
        cache.0.unavailable_guilds.insert(self.id);
    }
}

impl UpdateCache for UserUpdate {
    fn update(&self, cache: &InMemoryCache) {
        if !guard(cache, EventType::USER_UPDATE) {
            return;
        }

        cache.cache_current_user(self.0.clone());
    }
}

impl UpdateCache for VoiceServerUpdate {
    fn update(&self, cache: &InMemoryCache) {
        if !guard(cache, EventType::VOICE_SERVER_UPDATE) {
            return;
        }
    }
}

impl UpdateCache for VoiceStateUpdate {
    fn update(&self, cache: &InMemoryCache) {
        if !guard(cache, EventType::VOICE_STATE_UPDATE) {
            return;
        }

        cache.cache_voice_state(self.0.clone());
    }
}

impl UpdateCache for WebhooksUpdate {}

#[cfg(test)]
mod tests {
    use super::*;
    use twilight_model::{
        channel::{ChannelType, GuildChannel, TextChannel},
        gateway::payload::ChannelDelete,
        id::{ChannelId, GuildId, UserId},
        voice::VoiceState,
    };

    fn guild_channel_text() -> (GuildId, ChannelId, GuildChannel) {
        let guild_id = GuildId(1);
        let channel_id = ChannelId(2);
        let channel = GuildChannel::Text(TextChannel {
            guild_id: Some(guild_id),
            id: channel_id,
            kind: ChannelType::GuildText,
            last_message_id: None,
            last_pin_timestamp: None,
            name: "test".to_owned(),
            nsfw: false,
            parent_id: None,
            permission_overwrites: Vec::new(),
            position: 3,
            rate_limit_per_user: None,
            topic: None,
        });

        (guild_id, channel_id, channel)
    }

    #[test]
    fn test_channel_delete_guild() {
        let cache = InMemoryCache::new();
        let (guild_id, channel_id, channel) = guild_channel_text();

        cache.cache_guild_channel(guild_id, channel.clone());
        assert_eq!(1, cache.0.channels_guild.len());
        assert!(cache
            .0
            .guild_channels
            .get(&guild_id)
            .unwrap()
            .contains(&channel_id));

        cache.update(&ChannelDelete(Channel::Guild(channel)));
        assert!(cache.0.channels_guild.is_empty());
        assert!(cache.0.guild_channels.get(&guild_id).unwrap().is_empty());
    }

    #[test]
    fn test_channel_update_guild() {
        let cache = InMemoryCache::new();
        let (guild_id, channel_id, channel) = guild_channel_text();

        cache.update(&ChannelUpdate(Channel::Guild(channel)));
        assert_eq!(1, cache.0.channels_guild.len());
        assert!(cache
            .0
            .guild_channels
            .get(&guild_id)
            .unwrap()
            .contains(&channel_id));
    }

    #[test]
    fn test_voice_states_with_no_cached_guilds() {
        let cache = InMemoryCache::builder()
            .event_types(crate::config::EventType::VOICE_STATE_UPDATE)
            .build();

        cache.update(&VoiceStateUpdate(VoiceState {
            channel_id: None,
            deaf: false,
            guild_id: Some(GuildId(1)),
            member: None,
            mute: false,
            self_deaf: false,
            self_mute: false,
            self_stream: false,
            session_id: "38fj3jfkh3pfho3prh2".to_string(),
            suppress: false,
            token: None,
            user_id: UserId(1),
        }));
    }
}
