use crate::{config::ResourceType, CacheableGuild, CacheableModels, InMemoryCache, UpdateCache};
use dashmap::DashMap;
use std::{collections::HashSet, hash::Hash, mem};
use twilight_model::{
    gateway::payload::incoming::{GuildCreate, GuildDelete, GuildUpdate},
    guild::Guild,
    id::{marker::GuildMarker, Id},
};

impl<CacheModels: CacheableModels> InMemoryCache<CacheModels> {
    #[allow(clippy::too_many_lines)]
    fn cache_guild(&self, mut guild: Guild) {
        // The map and set creation needs to occur first, so caching states and
        // objects always has a place to put them.
        if self.wants(ResourceType::CHANNEL) {
            self.guild_channels.insert(guild.id, HashSet::new());

            let mut channels = mem::take(&mut guild.channels);
            let mut threads = mem::take(&mut guild.threads);

            for channel in &mut channels {
                channel.guild_id = Some(guild.id);
            }

            for channel in &mut threads {
                channel.guild_id = Some(guild.id);
            }

            self.cache_channels(channels);
            self.cache_channels(threads);
        }

        if self.wants(ResourceType::EMOJI) {
            self.guild_emojis.insert(guild.id, HashSet::new());
            self.cache_emojis(guild.id, mem::take(&mut guild.emojis));
        }

        if self.wants(ResourceType::MEMBER) {
            self.guild_members.insert(guild.id, HashSet::new());
            self.cache_members(guild.id, mem::take(&mut guild.members));
        }

        if self.wants(ResourceType::PRESENCE) {
            self.guild_presences.insert(guild.id, HashSet::new());
            self.cache_presences(guild.id, mem::take(&mut guild.presences));
        }

        if self.wants(ResourceType::ROLE) {
            self.guild_roles.insert(guild.id, HashSet::new());
            self.cache_roles(guild.id, mem::take(&mut guild.roles));
        }

        if self.wants(ResourceType::STICKER) {
            self.guild_stage_instances.insert(guild.id, HashSet::new());
            self.cache_stickers(guild.id, mem::take(&mut guild.stickers));
        }

        if self.wants(ResourceType::VOICE_STATE) {
            self.voice_state_guilds.insert(guild.id, HashSet::new());
            self.cache_voice_states(mem::take(&mut guild.voice_states));
        }

        if self.wants(ResourceType::STAGE_INSTANCE) {
            self.guild_stage_instances.insert(guild.id, HashSet::new());
            self.cache_stage_instances(guild.id, mem::take(&mut guild.stage_instances));
        }

        if self.wants(ResourceType::GUILD) {
            let guild = CacheModels::Guild::from(guild);
            self.unavailable_guilds.remove(&guild.id());
            self.guilds.insert(guild.id(), guild);
        }
    }

    pub(crate) fn delete_guild(&self, id: Id<GuildMarker>, unavailable: bool) {
        fn remove_ids<T: Eq + Hash, U>(
            guild_map: &DashMap<Id<GuildMarker>, HashSet<T>>,
            container: &DashMap<T, U>,
            guild_id: Id<GuildMarker>,
        ) {
            if let Some((_, ids)) = guild_map.remove(&guild_id) {
                for id in ids {
                    container.remove(&id);
                }
            }
        }

        if self.wants(ResourceType::GUILD) {
            if unavailable {
                if let Some(mut guild) = self.guilds.get_mut(&id) {
                    guild.set_unavailable(true);
                }
            } else {
                self.guilds.remove(&id);
            }
        }

        if self.wants(ResourceType::CHANNEL) {
            remove_ids(&self.guild_channels, &self.channels, id);
        }

        if self.wants(ResourceType::EMOJI) {
            remove_ids(&self.guild_emojis, &self.emojis, id);
        }

        if self.wants(ResourceType::ROLE) {
            remove_ids(&self.guild_roles, &self.roles, id);
        }

        if self.wants(ResourceType::STICKER) {
            remove_ids(&self.guild_stickers, &self.stickers, id);
        }

        if self.wants(ResourceType::VOICE_STATE) {
            // Clear out a guilds voice states when a guild leaves
            self.voice_state_guilds.remove(&id);
        }

        if self.wants(ResourceType::MEMBER) {
            if let Some((_, ids)) = self.guild_members.remove(&id) {
                for user_id in ids {
                    self.members.remove(&(id, user_id));
                }
            }
        }

        if self.wants(ResourceType::PRESENCE) {
            if let Some((_, ids)) = self.guild_presences.remove(&id) {
                for user_id in ids {
                    self.presences.remove(&(id, user_id));
                }
            }
        }
    }
}

impl<CacheModels: CacheableModels> UpdateCache<CacheModels> for GuildCreate {
    fn update(&self, cache: &InMemoryCache<CacheModels>) {
        match self {
            GuildCreate::Available(g) => cache.cache_guild(g.clone()),
            GuildCreate::Unavailable(g) => {
                cache.unavailable_guilds.insert(g.id);
            }
        }
    }
}

impl<CacheModels: CacheableModels> UpdateCache<CacheModels> for GuildDelete {
    fn update(&self, cache: &InMemoryCache<CacheModels>) {
        cache.delete_guild(self.id, false);
    }
}

impl<CacheModels: CacheableModels> UpdateCache<CacheModels> for GuildUpdate {
    fn update(&self, cache: &InMemoryCache<CacheModels>) {
        if !cache.wants(ResourceType::GUILD) {
            return;
        }

        if let Some(mut guild) = cache.guilds.get_mut(&self.0.id) {
            guild.update_with_guild_update(self);
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::{test, DefaultInMemoryCache};
    use std::str::FromStr;
    use twilight_model::{
        channel::{
            thread::{AutoArchiveDuration, ThreadMember, ThreadMetadata},
            Channel, ChannelType,
        },
        gateway::payload::incoming::{
            GuildCreate, GuildUpdate, MemberAdd, MemberRemove, UnavailableGuild,
        },
        guild::{
            AfkTimeout, DefaultMessageNotificationLevel, ExplicitContentFilter, Guild, MfaLevel,
            NSFWLevel, PartialGuild, Permissions, PremiumTier, SystemChannelFlags,
            VerificationLevel,
        },
        id::Id,
        util::datetime::{Timestamp, TimestampParseError},
    };

    #[allow(clippy::too_many_lines)]
    #[test]
    fn guild_create_channels_have_guild_ids() -> Result<(), TimestampParseError> {
        const DATETIME: &str = "2021-09-19T14:17:32.000000+00:00";

        let timestamp = Timestamp::from_str(DATETIME)?;

        let channels = Vec::from([Channel {
            application_id: None,
            applied_tags: None,
            available_tags: None,
            bitrate: None,
            default_auto_archive_duration: None,
            default_forum_layout: None,
            default_reaction_emoji: None,
            default_sort_order: None,
            default_thread_rate_limit_per_user: None,
            flags: None,
            guild_id: None,
            icon: None,
            id: Id::new(111),
            invitable: None,
            kind: ChannelType::GuildText,
            last_message_id: None,
            last_pin_timestamp: None,
            managed: None,
            member: None,
            member_count: None,
            message_count: None,
            name: Some("guild channel with no guild id".to_owned()),
            newly_created: None,
            nsfw: Some(true),
            owner_id: None,
            parent_id: None,
            permission_overwrites: Some(Vec::new()),
            position: Some(1),
            rate_limit_per_user: None,
            recipients: None,
            rtc_region: None,
            thread_metadata: None,
            topic: None,
            user_limit: None,
            video_quality_mode: None,
        }]);

        let threads = Vec::from([Channel {
            application_id: None,
            applied_tags: None,
            available_tags: None,
            bitrate: None,
            default_auto_archive_duration: None,
            default_forum_layout: None,
            default_reaction_emoji: None,
            default_sort_order: None,
            default_thread_rate_limit_per_user: None,
            flags: None,
            guild_id: None,
            icon: None,
            id: Id::new(222),
            invitable: None,
            kind: ChannelType::PublicThread,
            last_message_id: None,
            last_pin_timestamp: None,
            managed: Some(true),
            member: Some(ThreadMember {
                flags: 0,
                id: Some(Id::new(1)),
                join_timestamp: timestamp,
                member: None,
                presence: None,
                user_id: Some(Id::new(2)),
            }),
            member_count: Some(0),
            message_count: Some(0),
            name: Some("guild thread with no guild id".to_owned()),
            newly_created: None,
            nsfw: None,
            owner_id: None,
            parent_id: None,
            permission_overwrites: None,
            position: None,
            rate_limit_per_user: None,
            recipients: None,
            rtc_region: None,
            thread_metadata: Some(ThreadMetadata {
                archived: false,
                auto_archive_duration: AutoArchiveDuration::Hour,
                archive_timestamp: timestamp,
                create_timestamp: Some(timestamp),
                invitable: None,
                locked: false,
            }),
            topic: None,
            user_limit: None,
            video_quality_mode: None,
        }]);

        let guild = Guild {
            afk_channel_id: None,
            afk_timeout: AfkTimeout::FIFTEEN_MINUTES,
            application_id: None,
            approximate_member_count: None,
            approximate_presence_count: None,
            banner: None,
            channels,
            default_message_notifications: DefaultMessageNotificationLevel::Mentions,
            description: None,
            discovery_splash: None,
            emojis: Vec::new(),
            explicit_content_filter: ExplicitContentFilter::AllMembers,
            features: vec![],
            icon: None,
            id: Id::new(123),
            joined_at: Some(Timestamp::from_secs(1_632_072_645).expect("non zero")),
            large: false,
            max_members: Some(50),
            max_presences: Some(100),
            max_video_channel_users: None,
            member_count: Some(25),
            members: Vec::new(),
            mfa_level: MfaLevel::Elevated,
            name: "this is a guild".to_owned(),
            nsfw_level: NSFWLevel::AgeRestricted,
            owner_id: Id::new(456),
            owner: Some(false),
            permissions: Some(Permissions::SEND_MESSAGES),
            preferred_locale: "en-GB".to_owned(),
            premium_progress_bar_enabled: true,
            premium_subscription_count: Some(0),
            premium_tier: PremiumTier::None,
            presences: Vec::new(),
            public_updates_channel_id: None,
            roles: Vec::new(),
            rules_channel_id: None,
            safety_alerts_channel_id: Some(Id::new(789)),
            splash: None,
            stage_instances: Vec::new(),
            stickers: Vec::new(),
            system_channel_flags: SystemChannelFlags::SUPPRESS_JOIN_NOTIFICATIONS,
            system_channel_id: None,
            threads,
            unavailable: false,
            vanity_url_code: None,
            verification_level: VerificationLevel::VeryHigh,
            voice_states: Vec::new(),
            widget_channel_id: None,
            widget_enabled: None,
        };

        let cache = DefaultInMemoryCache::new();
        cache.cache_guild(guild);

        let channel = cache.channel(Id::new(111)).unwrap();

        let thread = cache.channel(Id::new(222)).unwrap();

        // The channel was given to the cache without a guild ID, but because
        // it's part of a guild create, the cache can automatically attach the
        // guild ID to it. So now, the channel's guild ID is present with the
        // correct value.
        assert_eq!(Some(Id::new(123)), channel.guild_id);
        assert_eq!(Some(Id::new(123)), thread.guild_id);

        Ok(())
    }

    #[test]
    fn unavailable_available_guild() {
        let cache = DefaultInMemoryCache::new();
        let guild = test::guild(Id::new(1), None);

        cache.update(&GuildCreate::Unavailable(
            twilight_model::guild::UnavailableGuild {
                id: guild.id,
                unavailable: true,
            },
        ));
        assert!(cache.unavailable_guilds.get(&guild.id).is_some());

        cache.update(&GuildCreate::Available(guild.clone()));
        assert_eq!(*cache.guilds.get(&guild.id).unwrap(), guild);
        assert!(cache.unavailable_guilds.get(&guild.id).is_none());

        cache.update(&GuildCreate::Unavailable(
            twilight_model::guild::UnavailableGuild {
                id: guild.id,
                unavailable: true,
            },
        ));
        assert!(cache.unavailable_guilds.get(&guild.id).is_some());
        assert!(cache.guilds.get(&guild.id).unwrap().unavailable);

        cache.update(&GuildCreate::Available(guild.clone()));
        assert!(!cache.guilds.get(&guild.id).unwrap().unavailable);
        assert!(cache.unavailable_guilds.get(&guild.id).is_none());
    }

    #[test]
    fn guild_update() {
        let cache = DefaultInMemoryCache::new();
        let guild = test::guild(Id::new(1), None);

        cache.update(&GuildCreate::Available(guild.clone()));

        let mutation = PartialGuild {
            id: guild.id,
            afk_channel_id: guild.afk_channel_id,
            afk_timeout: guild.afk_timeout,
            application_id: guild.application_id,
            banner: guild.banner,
            default_message_notifications: guild.default_message_notifications,
            description: guild.description,
            discovery_splash: guild.discovery_splash,
            emojis: guild.emojis,
            explicit_content_filter: guild.explicit_content_filter,
            features: guild.features,
            icon: guild.icon,
            max_members: guild.max_members,
            max_presences: guild.max_presences,
            member_count: guild.member_count,
            mfa_level: guild.mfa_level,
            name: "test2222".to_owned(),
            nsfw_level: guild.nsfw_level,
            owner_id: Id::new(2),
            owner: guild.owner,
            permissions: guild.permissions,
            preferred_locale: guild.preferred_locale,
            premium_progress_bar_enabled: guild.premium_progress_bar_enabled,
            premium_subscription_count: guild.premium_subscription_count,
            premium_tier: guild.premium_tier,
            public_updates_channel_id: None,
            roles: guild.roles,
            rules_channel_id: guild.rules_channel_id,
            splash: guild.splash,
            system_channel_flags: guild.system_channel_flags,
            system_channel_id: guild.system_channel_id,
            verification_level: guild.verification_level,
            vanity_url_code: guild.vanity_url_code,
            widget_channel_id: guild.widget_channel_id,
            widget_enabled: guild.widget_enabled,
        };

        cache.update(&GuildUpdate(mutation.clone()));

        assert_eq!(cache.guild(guild.id).unwrap().name, mutation.name);
        assert_eq!(cache.guild(guild.id).unwrap().owner_id, mutation.owner_id);
        assert_eq!(cache.guild(guild.id).unwrap().id, mutation.id);
    }

    #[test]
    fn guild_member_count() {
        let user_id = Id::new(2);
        let guild_id = Id::new(1);
        let cache = DefaultInMemoryCache::new();
        let user = test::user(user_id);
        let member = test::member(user_id);
        let guild = test::guild(guild_id, Some(1));

        cache.update(&GuildCreate::Available(guild));
        cache.update(&MemberAdd { guild_id, member });

        assert_eq!(cache.guild(guild_id).unwrap().member_count, Some(2));

        cache.update(&MemberRemove { guild_id, user });

        assert_eq!(cache.guild(guild_id).unwrap().member_count, Some(1));
    }

    #[test]
    fn guild_members_size_after_unavailable() {
        let user_id = Id::new(2);
        let guild_id = Id::new(1);
        let cache = DefaultInMemoryCache::new();
        let member = test::member(user_id);
        let mut guild = test::guild(guild_id, Some(1));
        guild.members.push(member);

        cache.update(&GuildCreate::Available(guild.clone()));

        assert_eq!(
            1,
            cache
                .guild_members(guild_id)
                .map(|members| members.len())
                .unwrap_or_default()
        );

        cache.update(&UnavailableGuild { id: guild_id });

        assert_eq!(
            0,
            cache
                .guild_members(guild_id)
                .map(|members| members.len())
                .unwrap_or_default()
        );
        assert!(cache.guild(guild_id).unwrap().unavailable);

        cache.update(&GuildCreate::Available(guild));

        assert_eq!(
            1,
            cache
                .guild_members(guild_id)
                .map(|members| members.len())
                .unwrap_or_default()
        );
        assert!(!cache.guild(guild_id).unwrap().unavailable);
    }
}
