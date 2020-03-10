use crate::{
    guild::{
        DefaultMessageNotificationLevel,
        Emoji,
        ExplicitContentFilter,
        MfaLevel,
        Permissions,
        PremiumTier,
        Role,
        SystemChannelFlags,
        VerificationLevel,
    },
    id::{ApplicationId, ChannelId, EmojiId, GuildId, RoleId, UserId},
};
use std::collections::HashMap;

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PartialGuild {
    pub id: GuildId,
    pub afk_channel_id: Option<ChannelId>,
    pub afk_timeout: u64,
    pub application_id: Option<ApplicationId>,
    pub banner: Option<String>,
    pub default_message_notifications: DefaultMessageNotificationLevel,
    pub description: Option<String>,
    pub discovery_splash: Option<String>,
    pub embed_channel_id: Option<ChannelId>,
    #[cfg_attr(feature = "serde-support", serde(default))]
    pub embed_enabled: bool,
    #[cfg_attr(feature = "serde-support", serde(with = "serde_mappable_seq"))]
    pub emojis: HashMap<EmojiId, Emoji>,
    pub explicit_content_filter: ExplicitContentFilter,
    pub features: Vec<String>,
    pub icon: Option<String>,
    pub max_members: Option<u64>,
    pub max_presences: Option<u64>,
    pub member_count: Option<u64>,
    pub mfa_level: MfaLevel,
    pub name: String,
    pub owner: Option<bool>,
    pub owner_id: UserId,
    pub permissions: Option<Permissions>,
    pub preferred_locale: String,
    pub premium_tier: PremiumTier,
    pub premium_subscription_count: Option<u64>,
    pub region: String,
    #[cfg_attr(feature = "serde-support", serde(with = "serde_mappable_seq"))]
    pub roles: HashMap<RoleId, Role>,
    pub rules_channel_id: Option<ChannelId>,
    pub splash: Option<String>,
    pub system_channel_id: Option<ChannelId>,
    pub system_channel_flags: SystemChannelFlags,
    pub verification_level: VerificationLevel,
    pub vanity_url_code: Option<String>,
    pub widget_channel_id: Option<ChannelId>,
    pub widget_enabled: Option<bool>,
}
