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

use self::{member::MemberMapDeserializer, role::RoleMapDeserializer};
use crate::{
    channel::{GuildChannel, GuildChannelMapDeserializer},
    gateway::presence::{Presence, PresenceMapDeserializer},
    guild::emoji::EmojiMapDeserializer,
    id::{ApplicationId, ChannelId, EmojiId, GuildId, RoleId, UserId},
    voice::voice_state::{VoiceState, VoiceStateMapDeserializer},
};
use serde::{
    de::{Deserializer, Error as DeError, IgnoredAny, MapAccess, Visitor},
    Deserialize, Serialize,
};
use std::{
    collections::HashMap,
    fmt::{Formatter, Result as FmtResult},
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Guild {
    pub afk_channel_id: Option<ChannelId>,
    pub afk_timeout: u64,
    pub application_id: Option<ApplicationId>,
    pub approximate_member_count: Option<u64>,
    pub approximate_presence_count: Option<u64>,
    pub banner: Option<String>,
    #[serde(default, with = "serde_mappable_seq")]
    pub channels: HashMap<ChannelId, GuildChannel>,
    pub default_message_notifications: DefaultMessageNotificationLevel,
    pub description: Option<String>,
    pub discovery_splash: Option<String>,
    pub embed_channel_id: Option<ChannelId>,
    pub embed_enabled: Option<bool>,
    #[serde(with = "serde_mappable_seq")]
    pub emojis: HashMap<EmojiId, Emoji>,
    pub explicit_content_filter: ExplicitContentFilter,
    pub features: Vec<String>,
    pub icon: Option<String>,
    pub id: GuildId,
    pub joined_at: Option<String>,
    #[serde(default)]
    pub large: bool,
    // Not documented so I marked it as optional.
    pub lazy: Option<bool>,
    pub max_members: Option<u64>,
    pub max_presences: Option<u64>,
    pub max_video_channel_users: Option<u64>,
    pub member_count: Option<u64>,
    #[serde(default, with = "serde_mappable_seq")]
    pub members: HashMap<UserId, Member>,
    pub mfa_level: MfaLevel,
    pub name: String,
    pub owner_id: UserId,
    pub owner: Option<bool>,
    pub permissions: Option<Permissions>,
    pub preferred_locale: String,
    pub premium_subscription_count: Option<u64>,
    #[serde(default)]
    pub premium_tier: PremiumTier,
    #[serde(default, with = "serde_mappable_seq")]
    pub presences: HashMap<UserId, Presence>,
    pub region: String,
    #[serde(with = "serde_mappable_seq")]
    pub roles: HashMap<RoleId, Role>,
    pub rules_channel_id: Option<ChannelId>,
    pub splash: Option<String>,
    pub system_channel_flags: SystemChannelFlags,
    pub system_channel_id: Option<ChannelId>,
    #[serde(default)]
    pub unavailable: bool,
    pub vanity_url_code: Option<String>,
    pub verification_level: VerificationLevel,
    #[serde(default, with = "serde_mappable_seq")]
    pub voice_states: HashMap<UserId, VoiceState>,
    pub widget_channel_id: Option<ChannelId>,
    pub widget_enabled: Option<bool>,
}

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

                let span = tracing::trace_span!("deserializing guild");
                let _span_enter = span.enter();

                loop {
                    let span_child = tracing::trace_span!(parent: &span, "iterating over element");
                    let _span_child_enter = span_child.enter();

                    let key = match map.next_key() {
                        Ok(Some(key)) => {
                            tracing::trace!(parent: &span_child, ?key, "found key");

                            key
                        }
                        Ok(None) => break,
                        Err(why) => {
                            // Encountered when we run into an unknown key.
                            map.next_value::<IgnoredAny>()?;

                            tracing::trace!(
                                parent: &span_child,
                                "ran into an unknown key: {:?}",
                                why
                            );

                            continue;
                        }
                    };

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
                                return Err(DeError::duplicate_field("approximate_member_count"));
                            }

                            approximate_member_count = Some(map.next_value()?);
                        }
                        Field::ApproximatePresenceCount => {
                            if approximate_presence_count.is_some() {
                                return Err(DeError::duplicate_field("approximate_presence_count"));
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

                            channels = Some(map.next_value_seed(GuildChannelMapDeserializer)?);
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

                            emojis = Some(map.next_value_seed(EmojiMapDeserializer)?);
                        }
                        Field::ExplicitContentFilter => {
                            if explicit_content_filter.is_some() {
                                return Err(DeError::duplicate_field("explicit_content_filter"));
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
                                return Err(DeError::duplicate_field("max_video_channel_users"));
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

                            let deserializer = MemberMapDeserializer::new(GuildId(0));

                            members = Some(map.next_value_seed(deserializer)?);
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
                                return Err(DeError::duplicate_field("premium_subscription_count"));
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

                            let deserializer = PresenceMapDeserializer::new(GuildId(0));

                            presences = Some(map.next_value_seed(deserializer)?);
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

                            roles = Some(map.next_value_seed(RoleMapDeserializer)?);
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

                            voice_states = Some(map.next_value_seed(VoiceStateMapDeserializer)?);
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

                let afk_timeout =
                    afk_timeout.ok_or_else(|| DeError::missing_field("afk_timeout"))?;
                let default_message_notifications = default_message_notifications
                    .ok_or_else(|| DeError::missing_field("default_message_notifications"))?;
                let explicit_content_filter = explicit_content_filter
                    .ok_or_else(|| DeError::missing_field("explicit_content_filter"))?;
                let features = features.ok_or_else(|| DeError::missing_field("features"))?;
                let id = id.ok_or_else(|| DeError::missing_field("id"))?;
                let mfa_level = mfa_level.ok_or_else(|| DeError::missing_field("mfa_level"))?;
                let name = name.ok_or_else(|| DeError::missing_field("name"))?;
                let owner_id = owner_id.ok_or_else(|| DeError::missing_field("owner_id"))?;
                let preferred_locale =
                    preferred_locale.ok_or_else(|| DeError::missing_field("preferred_locale"))?;
                let region = region.ok_or_else(|| DeError::missing_field("region"))?;
                let roles = roles.ok_or_else(|| DeError::missing_field("roles"))?;
                let system_channel_flags = system_channel_flags
                    .ok_or_else(|| DeError::missing_field("system_channel_flags"))?;
                let verification_level = verification_level
                    .ok_or_else(|| DeError::missing_field("verification_level"))?;

                let afk_channel_id = afk_channel_id.unwrap_or_default();
                let application_id = application_id.unwrap_or_default();
                let approximate_member_count = approximate_member_count.unwrap_or_default();
                let approximate_presence_count = approximate_presence_count.unwrap_or_default();
                let banner = banner.unwrap_or_default();
                let mut channels = channels.unwrap_or_default();
                let description = description.unwrap_or_default();
                let discovery_splash = discovery_splash.unwrap_or_default();
                let embed_channel_id = embed_channel_id.unwrap_or_default();
                let embed_enabled = embed_enabled.unwrap_or_default();
                let emojis = emojis.unwrap_or_default();
                let icon = icon.unwrap_or_default();
                let large = large.unwrap_or_default();
                let joined_at = joined_at.unwrap_or_default();
                let lazy = lazy.unwrap_or_default();
                let max_members = max_members.unwrap_or_default();
                let max_presences = max_presences.unwrap_or_default();
                let max_video_channel_users = max_video_channel_users.unwrap_or_default();
                let member_count = member_count.unwrap_or_default();
                let mut members = members.unwrap_or_default();
                let owner = owner.unwrap_or_default();
                let permissions = permissions.unwrap_or_default();
                let premium_subscription_count = premium_subscription_count.unwrap_or_default();
                let premium_tier = premium_tier.unwrap_or_default();
                let mut presences = presences.unwrap_or_default();
                let rules_channel_id = rules_channel_id.unwrap_or_default();
                let splash = splash.unwrap_or_default();
                let system_channel_id = system_channel_id.unwrap_or_default();
                let unavailable = unavailable.unwrap_or_default();
                let vanity_url_code = vanity_url_code.unwrap_or_default();
                let mut voice_states = voice_states.unwrap_or_default();
                let widget_channel_id = widget_channel_id.unwrap_or_default();
                let widget_enabled = widget_enabled.unwrap_or_default();

                tracing::trace!(
                    parent: &span,
                    ?afk_channel_id,
                    %afk_timeout,
                    ?application_id,
                    ?approximate_member_count,
                    ?approximate_presence_count,
                    ?banner,
                    ?channels,
                    ?default_message_notifications,
                    ?description,
                    ?discovery_splash,
                    ?embed_channel_id,
                    ?embed_enabled,
                    ?emojis,
                    ?explicit_content_filter,
                    ?features,
                    ?icon,
                    %id,
                    ?large,
                    ?lazy,
                    ?joined_at,
                    ?max_members,
                    ?max_presences,
                    ?max_video_channel_users,
                    ?member_count,
                    ?members,
                    ?mfa_level,
                    %name,
                    %owner_id,
                    ?owner,
                    ?permissions,
                    ?preferred_locale,
                    ?premium_subscription_count,
                );

                // Split in two due to generic impl only going up to 32.
                tracing::trace!(parent: &span,
                    ?premium_tier,
                    ?presences,
                    %region,
                    ?rules_channel_id,
                    ?roles,
                    ?splash,
                    ?system_channel_flags,
                    ?system_channel_id,
                    ?unavailable,
                    ?vanity_url_code,
                    ?voice_states,
                    ?widget_channel_id,
                    ?widget_enabled,
                    ?verification_level,
                );

                for channel in channels.values_mut() {
                    match channel {
                        GuildChannel::Category(c) => {
                            c.guild_id.replace(id);
                        }
                        GuildChannel::Text(c) => {
                            c.guild_id.replace(id);
                        }
                        GuildChannel::Voice(c) => {
                            c.guild_id.replace(id);
                        }
                    }
                }

                for member in members.values_mut() {
                    member.guild_id = id;
                }

                for presence in presences.values_mut() {
                    presence.guild_id.replace(id);
                }

                for voice_state in voice_states.values_mut() {
                    voice_state.guild_id.replace(id);
                }

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
