use crate::{
    config::ResourceType,
    model::{CachedGuild, CachedPresence},
    InMemoryCache, UpdateCache,
};
use dashmap::DashMap;
use std::{collections::HashSet, hash::Hash};
use twilight_model::{
    gateway::payload::incoming::{GuildCreate, GuildDelete, GuildUpdate},
    guild::Guild,
    id::GuildId,
};

impl InMemoryCache {
    fn cache_guild(&self, guild: Guild) {
        // The map and set creation needs to occur first, so caching states and
        // objects always has a place to put them.
        if self.wants(ResourceType::CHANNEL) {
            self.guild_channels.insert(guild.id, HashSet::new());
            self.cache_guild_channels(guild.id, guild.channels);
            self.cache_guild_channels(guild.id, guild.threads);
        }

        if self.wants(ResourceType::EMOJI) {
            self.guild_emojis.insert(guild.id, HashSet::new());
            self.cache_emojis(guild.id, guild.emojis);
        }

        if self.wants(ResourceType::MEMBER) {
            self.guild_members.insert(guild.id, HashSet::new());
            self.cache_members(guild.id, guild.members);
        }

        if self.wants(ResourceType::PRESENCE) {
            self.guild_presences.insert(guild.id, HashSet::new());
            self.cache_presences(
                guild.id,
                guild.presences.into_iter().map(CachedPresence::from),
            );
        }

        if self.wants(ResourceType::ROLE) {
            self.guild_roles.insert(guild.id, HashSet::new());
            self.cache_roles(guild.id, guild.roles);
        }

        if self.wants(ResourceType::STICKER) {
            self.guild_stage_instances.insert(guild.id, HashSet::new());
            self.cache_stickers(guild.id, guild.stickers);
        }

        if self.wants(ResourceType::VOICE_STATE) {
            self.voice_state_guilds.insert(guild.id, HashSet::new());
            self.cache_voice_states(guild.voice_states);
        }

        if self.wants(ResourceType::STAGE_INSTANCE) {
            self.guild_stage_instances.insert(guild.id, HashSet::new());
            self.cache_stage_instances(guild.id, guild.stage_instances);
        }

        let guild = CachedGuild {
            id: guild.id,
            afk_channel_id: guild.afk_channel_id,
            afk_timeout: guild.afk_timeout,
            application_id: guild.application_id,
            banner: guild.banner,
            default_message_notifications: guild.default_message_notifications,
            description: guild.description,
            discovery_splash: guild.discovery_splash,
            explicit_content_filter: guild.explicit_content_filter,
            features: guild.features,
            icon: guild.icon,
            joined_at: guild.joined_at,
            large: guild.large,
            max_members: guild.max_members,
            max_presences: guild.max_presences,
            member_count: guild.member_count,
            mfa_level: guild.mfa_level,
            name: guild.name,
            nsfw_level: guild.nsfw_level,
            owner: guild.owner,
            owner_id: guild.owner_id,
            permissions: guild.permissions,
            preferred_locale: guild.preferred_locale,
            premium_subscription_count: guild.premium_subscription_count,
            premium_tier: guild.premium_tier,
            rules_channel_id: guild.rules_channel_id,
            splash: guild.splash,
            system_channel_id: guild.system_channel_id,
            system_channel_flags: guild.system_channel_flags,
            unavailable: guild.unavailable,
            verification_level: guild.verification_level,
            vanity_url_code: guild.vanity_url_code,
            widget_channel_id: guild.widget_channel_id,
            widget_enabled: guild.widget_enabled,
        };

        self.unavailable_guilds.remove(&guild.id());
        self.guilds.insert(guild.id(), guild);
    }
}

impl UpdateCache for GuildCreate {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::GUILD) {
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

        if !cache.wants(ResourceType::GUILD) {
            return;
        }

        let id = self.id;

        cache.guilds.remove(&id);

        if cache.wants(ResourceType::CHANNEL) {
            remove_ids(&cache.guild_channels, &cache.channels_guild, id);
        }

        if cache.wants(ResourceType::EMOJI) {
            remove_ids(&cache.guild_emojis, &cache.emojis, id);
        }

        if cache.wants(ResourceType::ROLE) {
            remove_ids(&cache.guild_roles, &cache.roles, id);
        }

        if cache.wants(ResourceType::STICKER) {
            remove_ids(&cache.guild_stickers, &cache.stickers, id);
        }

        if cache.wants(ResourceType::VOICE_STATE) {
            // Clear out a guilds voice states when a guild leaves
            cache.voice_state_guilds.remove(&id);
        }

        if cache.wants(ResourceType::MEMBER) {
            if let Some((_, ids)) = cache.guild_members.remove(&id) {
                for user_id in ids {
                    cache.members.remove(&(id, user_id));
                }
            }
        }

        if cache.wants(ResourceType::PRESENCE) {
            if let Some((_, ids)) = cache.guild_presences.remove(&id) {
                for user_id in ids {
                    cache.presences.remove(&(id, user_id));
                }
            }
        }
    }
}

impl UpdateCache for GuildUpdate {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::GUILD) {
            return;
        }

        if let Some(mut guild) = cache.guilds.get_mut(&self.0.id) {
            guild.afk_channel_id = self.afk_channel_id;
            guild.afk_timeout = self.afk_timeout;
            guild.banner = self.banner.clone();
            guild.default_message_notifications = self.default_message_notifications;
            guild.description = self.description.clone();
            guild.features = self.features.clone();
            guild.icon = self.icon.clone();
            guild.max_members = self.max_members;
            guild.max_presences = Some(self.max_presences.unwrap_or(25000));
            guild.mfa_level = self.mfa_level;
            guild.name = self.name.clone();
            guild.nsfw_level = self.nsfw_level;
            guild.owner = self.owner;
            guild.owner_id = self.owner_id;
            guild.permissions = self.permissions;
            guild.preferred_locale = self.preferred_locale.clone();
            guild.premium_tier = self.premium_tier;
            guild
                .premium_subscription_count
                .replace(self.premium_subscription_count.unwrap_or_default());
            guild.splash = self.splash.clone();
            guild.system_channel_id = self.system_channel_id;
            guild.verification_level = self.verification_level;
            guild.vanity_url_code = self.vanity_url_code.clone();
            guild.widget_channel_id = self.widget_channel_id;
            guild.widget_enabled = self.widget_enabled;
        };
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use twilight_model::{
        channel::{
            thread::{AutoArchiveDuration, PublicThread, ThreadMember, ThreadMetadata},
            ChannelType, GuildChannel, TextChannel,
        },
        datetime::{Timestamp, TimestampParseError},
        guild::{
            DefaultMessageNotificationLevel, ExplicitContentFilter, MfaLevel, NSFWLevel,
            PartialGuild, Permissions, PremiumTier, SystemChannelFlags, VerificationLevel,
        },
        id::{ChannelId, GuildId, UserId},
    };

    #[test]
    fn test_guild_create_channels_have_guild_ids() -> Result<(), TimestampParseError> {
        const DATETIME: &str = "2021-09-19T14:17:32.000000+00:00";

        let timestamp = Timestamp::from_str(DATETIME)?;

        let channels = Vec::from([GuildChannel::Text(TextChannel {
            id: ChannelId::new(111).expect("non zero"),
            guild_id: None,
            kind: ChannelType::GuildText,
            last_message_id: None,
            last_pin_timestamp: None,
            name: "guild channel with no guild id".to_owned(),
            nsfw: true,
            permission_overwrites: Vec::new(),
            parent_id: None,
            position: 1,
            rate_limit_per_user: None,
            topic: None,
        })]);

        let threads = Vec::from([GuildChannel::PublicThread(PublicThread {
            id: ChannelId::new(222).expect("non zero"),
            default_auto_archive_duration: None,
            guild_id: None,
            kind: ChannelType::GuildPublicThread,
            last_message_id: None,
            message_count: 0,
            name: "guild thread with no guild id".to_owned(),
            owner_id: None,
            parent_id: None,
            rate_limit_per_user: None,
            member_count: 0,
            thread_metadata: ThreadMetadata {
                archived: false,
                auto_archive_duration: AutoArchiveDuration::Hour,
                archive_timestamp: timestamp,
                invitable: None,
                locked: false,
            },
            member: Some(ThreadMember {
                flags: 0,
                id: Some(ChannelId::new(1).expect("non zero")),
                join_timestamp: timestamp,
                member: None,
                presence: None,
                user_id: Some(UserId::new(2).expect("non zero")),
            }),
        })]);

        let guild = Guild {
            id: GuildId::new(123).expect("non zero"),
            afk_channel_id: None,
            afk_timeout: 300,
            application_id: None,
            banner: None,
            channels,
            default_message_notifications: DefaultMessageNotificationLevel::Mentions,
            description: None,
            discovery_splash: None,
            emojis: Vec::new(),
            explicit_content_filter: ExplicitContentFilter::AllMembers,
            features: vec![],
            icon: None,
            joined_at: Some(Timestamp::from_secs(1_632_072_645).expect("non zero")),
            large: false,
            max_members: Some(50),
            max_presences: Some(100),
            member_count: Some(25),
            members: Vec::new(),
            mfa_level: MfaLevel::Elevated,
            name: "this is a guild".to_owned(),
            nsfw_level: NSFWLevel::AgeRestricted,
            owner: Some(false),
            owner_id: UserId::new(456).expect("non zero"),
            permissions: Some(Permissions::SEND_MESSAGES),
            preferred_locale: "en-GB".to_owned(),
            premium_subscription_count: Some(0),
            premium_tier: PremiumTier::None,
            presences: Vec::new(),
            roles: Vec::new(),
            splash: None,
            stage_instances: Vec::new(),
            stickers: Vec::new(),
            system_channel_id: None,
            system_channel_flags: SystemChannelFlags::SUPPRESS_JOIN_NOTIFICATIONS,
            rules_channel_id: None,
            threads,
            unavailable: false,
            verification_level: VerificationLevel::VeryHigh,
            voice_states: Vec::new(),
            vanity_url_code: None,
            widget_channel_id: None,
            widget_enabled: None,
            max_video_channel_users: None,
            approximate_member_count: None,
            approximate_presence_count: None,
        };

        let cache = InMemoryCache::new();
        cache.cache_guild(guild);

        let channel = cache
            .guild_channel(ChannelId::new(111).expect("non zero"))
            .unwrap();

        let thread = cache
            .guild_channel(ChannelId::new(222).expect("non zero"))
            .unwrap();

        // The channel was given to the cache without a guild ID, but because
        // it's part of a guild create, the cache can automatically attach the
        // guild ID to it. So now, the channel's guild ID is present with the
        // correct value.
        match channel.resource() {
            GuildChannel::Text(ref c) => {
                assert_eq!(Some(GuildId::new(123).expect("non zero")), c.guild_id);
            }
            _ => panic!("{:?}", channel),
        }

        match thread.resource() {
            GuildChannel::PublicThread(ref c) => {
                assert_eq!(Some(GuildId::new(123).expect("non zero")), c.guild_id);
            }
            _ => panic!("{:?}", channel),
        }

        Ok(())
    }

    #[test]
    fn test_guild_update() {
        let cache = InMemoryCache::new();
        let guild = Guild {
            afk_channel_id: None,
            afk_timeout: 0,
            application_id: None,
            approximate_member_count: None,
            approximate_presence_count: None,
            banner: None,
            channels: Vec::new(),
            default_message_notifications: DefaultMessageNotificationLevel::Mentions,
            description: None,
            discovery_splash: None,
            emojis: Vec::new(),
            explicit_content_filter: ExplicitContentFilter::None,
            features: Vec::new(),
            icon: None,
            id: GuildId::new(1).expect("non zero"),
            joined_at: None,
            large: false,
            max_members: None,
            max_presences: None,
            max_video_channel_users: None,
            member_count: None,
            members: Vec::new(),
            mfa_level: MfaLevel::None,
            name: "test".to_owned(),
            nsfw_level: NSFWLevel::Default,
            owner_id: UserId::new(1).expect("non zero"),
            owner: None,
            permissions: None,
            preferred_locale: "en_us".to_owned(),
            premium_subscription_count: None,
            premium_tier: PremiumTier::None,
            presences: Vec::new(),
            roles: Vec::new(),
            rules_channel_id: None,
            splash: None,
            stage_instances: Vec::new(),
            stickers: Vec::new(),
            system_channel_flags: SystemChannelFlags::empty(),
            system_channel_id: None,
            threads: Vec::new(),
            unavailable: false,
            vanity_url_code: None,
            verification_level: VerificationLevel::VeryHigh,
            voice_states: Vec::new(),
            widget_channel_id: None,
            widget_enabled: None,
        };

        cache.update(&GuildCreate(guild.clone()));

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
            owner_id: UserId::new(2).expect("non zero"),
            owner: guild.owner,
            permissions: guild.permissions,
            preferred_locale: guild.preferred_locale,
            premium_subscription_count: guild.premium_subscription_count,
            premium_tier: guild.premium_tier,
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
}
