pub mod audit_log;
pub mod member;

mod ban;
mod default_message_notification_level;
mod emoji;
mod explicit_content_filter;
mod info;
mod integration;
mod integration_account;
mod mfa_level;
mod partial_guild;
mod partial_member;
mod permissions;
mod premium_tier;
mod preview;
mod prune;
mod role;
mod status;
mod system_channel_flags;
mod unavailable_guild;
mod verification_level;
mod widget;

pub use self::{
    ban::Ban, default_message_notification_level::DefaultMessageNotificationLevel, emoji::Emoji,
    explicit_content_filter::ExplicitContentFilter, info::GuildInfo, integration::GuildIntegration,
    integration_account::IntegrationAccount, member::Member, mfa_level::MfaLevel,
    partial_guild::PartialGuild, partial_member::PartialMember, permissions::Permissions,
    premium_tier::PremiumTier, preview::GuildPreview, prune::GuildPrune, role::Role,
    status::GuildStatus, system_channel_flags::SystemChannelFlags,
    unavailable_guild::UnavailableGuild, verification_level::VerificationLevel,
    widget::GuildWidget,
};

use crate::{
    channel::GuildChannel,
    gateway::presence::Presence,
    id::{ApplicationId, ChannelId, EmojiId, GuildId, RoleId, UserId},
    voice::VoiceState,
};
use std::collections::HashMap;

#[cfg_attr(feature = "serde-support", derive(serde::Serialize))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Guild {
    pub id: GuildId,
    pub afk_channel_id: Option<ChannelId>,
    pub afk_timeout: u64,
    pub application_id: Option<ApplicationId>,
    pub banner: Option<String>,
    #[cfg_attr(feature = "serde-support", serde(default, with = "serde_mappable_seq"))]
    pub channels: HashMap<ChannelId, GuildChannel>,
    pub default_message_notifications: DefaultMessageNotificationLevel,
    pub description: Option<String>,
    pub discovery_splash: Option<String>,
    pub embed_channel_id: Option<ChannelId>,
    pub embed_enabled: Option<bool>,
    #[cfg_attr(feature = "serde-support", serde(with = "serde_mappable_seq"))]
    pub emojis: HashMap<EmojiId, Emoji>,
    pub explicit_content_filter: ExplicitContentFilter,
    pub features: Vec<String>,
    pub icon: Option<String>,
    pub joined_at: Option<String>,
    #[cfg_attr(feature = "serde-support", serde(default))]
    pub large: bool,
    // Not documented so I marked it as optional.
    pub lazy: Option<bool>,
    pub max_members: Option<u64>,
    pub max_presences: Option<u64>,
    pub member_count: Option<u64>,
    #[cfg_attr(feature = "serde-support", serde(default, with = "serde_mappable_seq"))]
    pub members: HashMap<UserId, Member>,
    pub mfa_level: MfaLevel,
    pub name: String,
    pub owner: Option<bool>,
    pub owner_id: UserId,
    pub permissions: Option<Permissions>,
    pub preferred_locale: String,
    pub premium_subscription_count: Option<u64>,
    #[cfg_attr(feature = "serde-support", serde(default))]
    pub premium_tier: PremiumTier,
    #[cfg_attr(feature = "serde-support", serde(default, with = "serde_mappable_seq"))]
    pub presences: HashMap<UserId, Presence>,
    pub region: String,
    #[cfg_attr(feature = "serde-support", serde(with = "serde_mappable_seq"))]
    pub roles: HashMap<RoleId, Role>,
    pub splash: Option<String>,
    pub system_channel_id: Option<ChannelId>,
    pub system_channel_flags: SystemChannelFlags,
    pub rules_channel_id: Option<ChannelId>,
    #[cfg_attr(feature = "serde-support", serde(default))]
    pub unavailable: bool,
    pub verification_level: VerificationLevel,
    #[cfg_attr(feature = "serde-support", serde(default, with = "serde_mappable_seq"))]
    pub voice_states: HashMap<UserId, VoiceState>,
    pub vanity_url_code: Option<String>,
    pub widget_channel_id: Option<ChannelId>,
    pub widget_enabled: Option<bool>,
    pub max_video_channel_users: Option<u64>,
    pub approximate_member_count: Option<u64>,
    pub approximate_presence_count: Option<u64>,
}

#[cfg(feature = "serde-support")]
mod if_serde_support {
    use super::{
        member::{if_serde_support::MemberIntermediary, Member},
        Guild,
    };
    use serde::{
        de::{Deserializer, Error as DeError, MapAccess, Visitor},
        Deserialize,
    };
    use serde_value::Value;
    use std::{
        collections::HashMap,
        fmt::{Formatter, Result as FmtResult},
    };

    impl<'de> Deserialize<'de> for Guild {
        #[allow(clippy::too_many_lines)]
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            #[derive(Debug, Deserialize)]
            #[serde(field_identifier, rename_all = "snake_case")]
            enum Field {
                AfkChannelId,
                AfkTimeout,
                ApplicationId,
                ApproximateMemberCount,
                ApproximatePresenceCount,
                Banner,
                Channels,
                DefaultMessageNotifications,
                Description,
                DiscoverySplash,
                EmbedChannelId,
                EmbedEnabled,
                Emojis,
                ExplicitContentFilter,
                Features,
                Icon,
                Id,
                JoinedAt,
                Large,
                Lazy,
                MaxMembers,
                MaxPresences,
                MaxVideoChannelUsers,
                MemberCount,
                Members,
                MfaLevel,
                Name,
                OwnerId,
                Owner,
                Permissions,
                PreferredLocale,
                PremiumSubscriptionCount,
                PremiumTier,
                Presences,
                Region,
                Roles,
                Splash,
                SystemChannelFlags,
                SystemChannelId,
                RulesChannelId,
                Unavailable,
                VerificationLevel,
                VoiceStates,
                VanityUrlCode,
                WidgetChannelId,
                WidgetEnabled,
            }

            struct GuildVisitor;

            impl<'de> Visitor<'de> for GuildVisitor {
                type Value = Guild;

                fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                    f.write_str("struct Guild")
                }

                #[allow(clippy::too_many_lines)]
                fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
                    let mut afk_channel_id = None::<Option<_>>;
                    let mut afk_timeout = None;
                    let mut application_id = None::<Option<_>>;
                    let mut approximate_member_count = None::<Option<_>>;
                    let mut approximate_presence_count = None::<Option<_>>;
                    let mut banner = None::<Option<_>>;
                    let mut channels = None;
                    let mut default_message_notifications = None;
                    let mut description = None::<Option<_>>;
                    let mut discovery_splash = None::<Option<_>>;
                    let mut embed_channel_id = None::<Option<_>>;
                    let mut embed_enabled = None::<Option<_>>;
                    let mut emojis = None;
                    let mut explicit_content_filter = None;
                    let mut features = None;
                    let mut icon = None::<Option<_>>;
                    let mut id = None;
                    let mut joined_at = None::<Option<_>>;
                    let mut large = None;
                    let mut lazy = None::<Option<_>>;
                    let mut max_members = None::<Option<_>>;
                    let mut max_presences = None::<Option<_>>;
                    let mut max_video_channel_users = None::<Option<_>>;
                    let mut member_count = None::<Option<_>>;
                    let mut members = None;
                    let mut mfa_level = None;
                    let mut name = None;
                    let mut owner = None::<Option<_>>;
                    let mut owner_id = None;
                    let mut permissions = None::<Option<_>>;
                    let mut preferred_locale = None;
                    let mut premium_subscription_count = None::<Option<_>>;
                    let mut premium_tier = None;
                    let mut presences = None;
                    let mut region = None;
                    let mut roles = None;
                    let mut splash = None::<Option<_>>;
                    let mut system_channel_id = None::<Option<_>>;
                    let mut system_channel_flags = None;
                    let mut rules_channel_id = None::<Option<_>>;
                    let mut unavailable = None;
                    let mut verification_level = None;
                    let mut voice_states = None;
                    let mut vanity_url_code = None::<Option<_>>;
                    let mut widget_channel_id = None::<Option<_>>;
                    let mut widget_enabled = None::<Option<_>>;

                    loop {
                        let key = match map.next_key() {
                            Ok(Some(key)) => key,
                            Ok(None) => {
                                log::debug!("breaking");

                                break;
                            }
                            Err(_) => {
                                // Encountered when we run into an unknown key.
                                log::debug!("continuing");

                                continue;
                            }
                        };

                        log::debug!("key {:?}", key);

                        match key {
                            Field::AfkChannelId => {
                                if afk_channel_id.is_some() {
                                    return Err(DeError::duplicate_field("afk_channel_id"));
                                }

                                afk_channel_id = Some(map.next_value()?);
                            }
                            Field::AfkTimeout => {
                                if afk_timeout.is_some() {
                                    return Err(DeError::duplicate_field("afk_timeout"));
                                }

                                afk_timeout = Some(map.next_value()?);
                            }
                            Field::ApplicationId => {
                                if application_id.is_some() {
                                    return Err(DeError::duplicate_field("application_id"));
                                }

                                application_id = Some(map.next_value()?);
                            }
                            Field::ApproximateMemberCount => {
                                if approximate_member_count.is_some() {
                                    return Err(DeError::duplicate_field(
                                        "approximate_member_count",
                                    ));
                                }

                                approximate_member_count = Some(map.next_value()?);
                            }
                            Field::ApproximatePresenceCount => {
                                if approximate_presence_count.is_some() {
                                    return Err(DeError::duplicate_field(
                                        "approximate_presence_count",
                                    ));
                                }

                                approximate_presence_count = Some(map.next_value()?);
                            }
                            Field::Banner => {
                                if banner.is_some() {
                                    return Err(DeError::duplicate_field("banner"));
                                }

                                banner = Some(map.next_value()?);
                            }
                            Field::Channels => {
                                if channels.is_some() {
                                    return Err(DeError::duplicate_field("channels"));
                                }

                                let raw_channels = map.next_value::<Value>()?;
                                channels =
                                    Some(serde_mappable_seq::deserialize(raw_channels).map_err(DeError::custom)?);
                            }
                            Field::DefaultMessageNotifications => {
                                if default_message_notifications.is_some() {
                                    return Err(DeError::duplicate_field(
                                        "default_message_notifications",
                                    ));
                                }

                                default_message_notifications = Some(map.next_value()?);
                            }
                            Field::Description => {
                                if description.is_some() {
                                    return Err(DeError::duplicate_field("description"));
                                }

                                description = Some(map.next_value()?);
                            }
                            Field::DiscoverySplash => {
                                if discovery_splash.is_some() {
                                    return Err(DeError::duplicate_field("discovery_splash"));
                                }

                                discovery_splash = Some(map.next_value()?);
                            }
                            Field::EmbedChannelId => {
                                if embed_channel_id.is_some() {
                                    return Err(DeError::duplicate_field("embed_channel_id"));
                                }

                                embed_channel_id = Some(map.next_value()?);
                            }
                            Field::EmbedEnabled => {
                                if embed_enabled.is_some() {
                                    return Err(DeError::duplicate_field("embed_enabled"));
                                }

                                embed_enabled = Some(map.next_value()?);
                            }
                            Field::Emojis => {
                                if emojis.is_some() {
                                    return Err(DeError::duplicate_field("emojis"));
                                }

                                let raw_emojis = map.next_value::<Value>()?;
                                emojis = Some(serde_mappable_seq::deserialize(raw_emojis).map_err(DeError::custom)?);
                            }
                            Field::ExplicitContentFilter => {
                                if explicit_content_filter.is_some() {
                                    return Err(DeError::duplicate_field(
                                        "explicit_content_filter",
                                    ));
                                }

                                explicit_content_filter = Some(map.next_value()?);
                            }
                            Field::Features => {
                                if features.is_some() {
                                    return Err(DeError::duplicate_field("features"));
                                }

                                features = Some(map.next_value()?);
                            }
                            Field::Icon => {
                                if icon.is_some() {
                                    return Err(DeError::duplicate_field("icon"));
                                }

                                icon = Some(map.next_value()?);
                            }
                            Field::Id => {
                                if id.is_some() {
                                    return Err(DeError::duplicate_field("id"));
                                }

                                id = Some(map.next_value()?);
                            }
                            Field::JoinedAt => {
                                if joined_at.is_some() {
                                    return Err(DeError::duplicate_field("joined_at"));
                                }

                                joined_at = Some(map.next_value()?);
                            }
                            Field::Large => {
                                if large.is_some() {
                                    return Err(DeError::duplicate_field("large"));
                                }

                                large = Some(map.next_value()?);
                            }
                            Field::Lazy => {
                                if lazy.is_some() {
                                    return Err(DeError::duplicate_field("lazy"));
                                }

                                lazy = Some(map.next_value()?);
                            }
                            Field::MaxMembers => {
                                if max_members.is_some() {
                                    return Err(DeError::duplicate_field("max_members"));
                                }

                                max_members = Some(map.next_value()?);
                            }
                            Field::MaxPresences => {
                                if max_presences.is_some() {
                                    return Err(DeError::duplicate_field("max_presences"));
                                }

                                max_presences = Some(map.next_value()?);
                            }
                            Field::MaxVideoChannelUsers => {
                                if max_video_channel_users.is_some() {
                                    return Err(DeError::duplicate_field(
                                        "max_video_channel_users",
                                    ));
                                }

                                max_video_channel_users = Some(map.next_value()?);
                            }
                            Field::MemberCount => {
                                if member_count.is_some() {
                                    return Err(DeError::duplicate_field("member_count"));
                                }

                                member_count = Some(map.next_value()?);
                            }
                            Field::Members => {
                                if members.is_some() {
                                    return Err(DeError::duplicate_field("members"));
                                }

                                members = Some(map.next_value::<Value>()?);
                            }
                            Field::MfaLevel => {
                                if mfa_level.is_some() {
                                    return Err(DeError::duplicate_field("mfa_level"));
                                }

                                mfa_level = Some(map.next_value()?);
                            }
                            Field::Name => {
                                if name.is_some() {
                                    return Err(DeError::duplicate_field("name"));
                                }

                                name = Some(map.next_value()?);
                            }
                            Field::Owner => {
                                if owner.is_some() {
                                    return Err(DeError::duplicate_field("owner"));
                                }

                                owner = Some(map.next_value()?);
                            }
                            Field::OwnerId => {
                                if owner_id.is_some() {
                                    return Err(DeError::duplicate_field("owner_id"));
                                }

                                owner_id = Some(map.next_value()?);
                            }
                            Field::Permissions => {
                                if permissions.is_some() {
                                    return Err(DeError::duplicate_field("permissions"));
                                }

                                permissions = Some(map.next_value()?);
                            }
                            Field::PreferredLocale => {
                                if preferred_locale.is_some() {
                                    return Err(DeError::duplicate_field("preferred_locale"));
                                }

                                preferred_locale = Some(map.next_value()?);
                            }
                            Field::PremiumSubscriptionCount => {
                                if premium_subscription_count.is_some() {
                                    return Err(DeError::duplicate_field(
                                        "premium_subscription_count",
                                    ));
                                }

                                premium_subscription_count = Some(map.next_value()?);
                            }
                            Field::PremiumTier => {
                                if premium_tier.is_some() {
                                    return Err(DeError::duplicate_field("premium_tier"));
                                }

                                premium_tier = Some(map.next_value()?);
                            }
                            Field::Presences => {
                                if presences.is_some() {
                                    return Err(DeError::duplicate_field("presences"));
                                }

                                let raw_presences = map.next_value::<Value>()?;
                                presences =
                                    Some(serde_mappable_seq::deserialize(raw_presences).map_err(DeError::custom)?);
                            }
                            Field::Region => {
                                if region.is_some() {
                                    return Err(DeError::duplicate_field("region"));
                                }

                                region = Some(map.next_value()?);
                            }
                            Field::Roles => {
                                if roles.is_some() {
                                    return Err(DeError::duplicate_field("roles"));
                                }

                                let raw_roles = map.next_value::<Value>()?;
                                roles = Some(serde_mappable_seq::deserialize(raw_roles).map_err(DeError::custom)?);
                            }
                            Field::Splash => {
                                if splash.is_some() {
                                    return Err(DeError::duplicate_field("splash"));
                                }

                                splash = Some(map.next_value()?);
                            }
                            Field::SystemChannelId => {
                                if system_channel_id.is_some() {
                                    return Err(DeError::duplicate_field("system_channel_id"));
                                }

                                system_channel_id = Some(map.next_value()?);
                            }
                            Field::SystemChannelFlags => {
                                if system_channel_flags.is_some() {
                                    return Err(DeError::duplicate_field("system_channel_flags"));
                                }

                                system_channel_flags = Some(map.next_value()?);
                            }
                            Field::RulesChannelId => {
                                if rules_channel_id.is_some() {
                                    return Err(DeError::duplicate_field("rules_channel_id"));
                                }

                                rules_channel_id = Some(map.next_value()?);
                            }
                            Field::Unavailable => {
                                if unavailable.is_some() {
                                    return Err(DeError::duplicate_field("unavailable"));
                                }

                                unavailable = Some(map.next_value()?);
                            }
                            Field::VerificationLevel => {
                                if verification_level.is_some() {
                                    return Err(DeError::duplicate_field("verification_level"));
                                }

                                verification_level = Some(map.next_value()?);
                            }
                            Field::VoiceStates => {
                                if voice_states.is_some() {
                                    return Err(DeError::duplicate_field("voice_states"));
                                }

                                let raw_voice_states = map.next_value::<Value>()?;
                                voice_states = Some(
                                    serde_mappable_seq::deserialize(raw_voice_states).map_err(DeError::custom)?,
                                );
                            }
                            Field::VanityUrlCode => {
                                if vanity_url_code.is_some() {
                                    return Err(DeError::duplicate_field("vanity_url_code"));
                                }

                                vanity_url_code = Some(map.next_value()?);
                            }
                            Field::WidgetChannelId => {
                                if widget_channel_id.is_some() {
                                    return Err(DeError::duplicate_field("widget_channel_id"));
                                }

                                widget_channel_id = Some(map.next_value()?);
                            }
                            Field::WidgetEnabled => {
                                if widget_enabled.is_some() {
                                    return Err(DeError::duplicate_field("widget_enabled"));
                                }

                                widget_enabled = Some(map.next_value()?);
                            }
                        }
                    }

                    let afk_channel_id = afk_channel_id.unwrap_or_default();
                    let afk_timeout =
                        afk_timeout.ok_or_else(|| DeError::missing_field("afk_timeout"))?;
                    let application_id = application_id.unwrap_or_default();
                    let approximate_member_count = approximate_member_count.unwrap_or_default();
                    let approximate_presence_count = approximate_presence_count.unwrap_or_default();
                    let banner = banner.unwrap_or_default();
                    let channels = channels.unwrap_or_default();
                    let default_message_notifications = default_message_notifications
                        .ok_or_else(|| DeError::missing_field("default_message_notifications"))?;
                    let description = description.unwrap_or_default();
                    let discovery_splash = discovery_splash.unwrap_or_default();
                    let embed_channel_id = embed_channel_id.unwrap_or_default();
                    let embed_enabled = embed_enabled.unwrap_or_default();
                    let emojis = emojis.unwrap_or_default();
                    let explicit_content_filter = explicit_content_filter
                        .ok_or_else(|| DeError::missing_field("explicit_content_filter"))?;
                    let features = features.ok_or_else(|| DeError::missing_field("features"))?;
                    let icon = icon.unwrap_or_default();
                    let id = id.ok_or_else(|| DeError::missing_field("id"))?;
                    let large = large.unwrap_or_default();
                    let joined_at = joined_at.unwrap_or_default();
                    let lazy = lazy.unwrap_or_default();
                    let max_members = max_members.unwrap_or_default();
                    let max_presences = max_presences.unwrap_or_default();
                    let max_video_channel_users = max_video_channel_users.unwrap_or_default();
                    let member_count = member_count.unwrap_or_default();
                    let mfa_level = mfa_level.ok_or_else(|| DeError::missing_field("mfa_level"))?;
                    let name = name.ok_or_else(|| DeError::missing_field("name"))?;
                    let owner_id = owner_id.ok_or_else(|| DeError::missing_field("owner_id"))?;
                    let owner = owner.unwrap_or_default();
                    let permissions = permissions.unwrap_or_default();
                    let preferred_locale = preferred_locale
                        .ok_or_else(|| DeError::missing_field("preferred_locale"))?;
                    let premium_subscription_count = premium_subscription_count.unwrap_or_default();
                    let premium_tier = premium_tier.unwrap_or_default();
                    let presences = presences.unwrap_or_default();
                    let region = region.ok_or_else(|| DeError::missing_field("region"))?;
                    let roles = roles.ok_or_else(|| DeError::missing_field("roles"))?;
                    let rules_channel_id = rules_channel_id.unwrap_or_default();
                    let splash = splash.unwrap_or_default();
                    let system_channel_flags = system_channel_flags
                        .ok_or_else(|| DeError::missing_field("system_channel_flags"))?;
                    let system_channel_id = system_channel_id.unwrap_or_default();
                    let unavailable = unavailable.unwrap_or_default();
                    let vanity_url_code = vanity_url_code.unwrap_or_default();
                    let verification_level = verification_level
                        .ok_or_else(|| DeError::missing_field("verification_level"))?;
                    let voice_states = voice_states.unwrap_or_default();
                    let widget_channel_id = widget_channel_id.unwrap_or_default();
                    let widget_enabled = widget_enabled.unwrap_or_default();

                    let members = match members {
                        Some(value) => {
                            let members =
                                value.deserialize_into::<Vec<MemberIntermediary>>().unwrap();

                            members
                                .into_iter()
                                .map(|member| {
                                    (
                                        member.user.id,
                                        Member {
                                            deaf: member.deaf,
                                            guild_id: id,
                                            hoisted_role: member.hoisted_role,
                                            joined_at: member.joined_at,
                                            mute: member.mute,
                                            nick: member.nick,
                                            premium_since: member.premium_since,
                                            roles: member.roles,
                                            user: member.user,
                                        },
                                    )
                                })
                                .collect::<HashMap<_, _>>()
                        }
                        None => HashMap::default(),
                    };

                    Ok(Guild {
                        afk_channel_id,
                        afk_timeout,
                        application_id,
                        approximate_member_count,
                        approximate_presence_count,
                        banner,
                        channels,
                        default_message_notifications,
                        description,
                        discovery_splash,
                        embed_channel_id,
                        embed_enabled,
                        emojis,
                        explicit_content_filter,
                        features,
                        icon,
                        id,
                        joined_at,
                        large,
                        lazy,
                        max_members,
                        max_presences,
                        max_video_channel_users,
                        member_count,
                        members,
                        mfa_level,
                        name,
                        owner,
                        owner_id,
                        permissions,
                        preferred_locale,
                        premium_subscription_count,
                        premium_tier,
                        presences,
                        region,
                        roles,
                        splash,
                        system_channel_id,
                        system_channel_flags,
                        rules_channel_id,
                        unavailable,
                        verification_level,
                        voice_states,
                        vanity_url_code,
                        widget_channel_id,
                        widget_enabled,
                    })
                }
            }

            const FIELDS: &[&str] = &[
                "afk_channel_id",
                "afk_timeout",
                "application_id",
                "approximate_member_count",
                "approximate_presence_count",
                "banner",
                "channels",
                "default_message_notifications",
                "description",
                "discovery_splash",
                "embed_channel_id",
                "embed_enabled",
                "emojis",
                "explicit_content_filter",
                "features",
                "icon",
                "id",
                "joined_at",
                "large",
                "lazy",
                "max_members",
                "max_presences",
                "max_video_channel_users",
                "member_count",
                "members",
                "mfa_level",
                "name",
                "owner",
                "owner_id",
                "permissions",
                "preferred_locale",
                "premium_subscription_count",
                "premium_tier",
                "presences",
                "region",
                "roles",
                "splash",
                "system_channel_id",
                "system_channel_flags",
                "rules_channel_id",
                "unavailable",
                "verification_level",
                "voice_states",
                "vanity_url_code",
                "widget_channel_id",
                "widget_enabled",
            ];

            deserializer.deserialize_struct("Guild", FIELDS, GuildVisitor)
        }
    }
}
