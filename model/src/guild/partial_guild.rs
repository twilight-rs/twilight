use crate::{
    guild::{
        DefaultMessageNotificationLevel,
        Emoji,
        MfaLevel,
        Permissions,
        PremiumTier,
        Role,
        VerificationLevel,
    },
    id::{ApplicationId, ChannelId, EmojiId, GuildId, RoleId, UserId},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PartialGuild {
    pub id: GuildId,
    pub afk_channel_id: Option<ChannelId>,
    pub afk_timeout: u64,
    pub application_id: Option<ApplicationId>,
    pub banner: Option<String>,
    pub default_message_notifications: DefaultMessageNotificationLevel,
    pub description: Option<String>,
    pub embed_channel_id: Option<ChannelId>,
    pub embed_enabled: bool,
    #[serde(with = "serde_mappable_seq")]
    pub emojis: HashMap<EmojiId, Emoji>,
    pub features: Vec<String>,
    pub icon: Option<String>,
    pub max_members: Option<u64>,
    #[serde(default = "super::default_max_presences")]
    pub max_presences: u64,
    pub mfa_level: MfaLevel,
    pub name: String,
    pub owner: Option<bool>,
    pub owner_id: UserId,
    pub permissions: Option<Permissions>,
    pub preferred_locale: String,
    pub premium_tier: PremiumTier,
    #[serde(default)]
    pub premium_subscription_count: u64,
    pub region: String,
    #[serde(with = "serde_mappable_seq")]
    pub roles: HashMap<RoleId, Role>,
    pub splash: Option<String>,
    pub system_channel_id: Option<ChannelId>,
    pub verification_level: VerificationLevel,
    pub vanity_url_code: Option<String>,
    pub widget_channel_id: Option<ChannelId>,
    pub widget_enabled: Option<bool>,
}
