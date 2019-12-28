pub mod audit_log;

mod ban;
mod default_message_notification_level;
mod embed;
mod emoji;
mod explicit_content_filter;
mod info;
mod integration;
mod integration_account;
mod member;
mod mfa_level;
mod partial_guild;
mod partial_member;
mod permissions;
mod premium_tier;
mod prune;
mod role;
mod status;
mod system_channel_flags;
mod unavailable_guild;
mod verification_level;

pub use self::{
    ban::Ban,
    default_message_notification_level::DefaultMessageNotificationLevel,
    embed::GuildEmbed,
    emoji::Emoji,
    explicit_content_filter::ExplicitContentFilter,
    info::GuildInfo,
    integration::GuildIntegration,
    integration_account::IntegrationAccount,
    member::Member,
    mfa_level::MfaLevel,
    partial_guild::PartialGuild,
    partial_member::PartialMember,
    permissions::Permissions,
    premium_tier::PremiumTier,
    prune::GuildPrune,
    role::Role,
    status::GuildStatus,
    system_channel_flags::SystemChannelFlags,
    unavailable_guild::UnavailableGuild,
    verification_level::VerificationLevel,
};

use crate::{
    channel::GuildChannel,
    gateway::presence::Presence,
    id::{ApplicationId, ChannelId, EmojiId, GuildId, RoleId, UserId},
    voice::VoiceState,
};
use std::collections::HashMap;

#[cfg(feature = "serde-support")]
fn default_max_presences() -> u64 {
    5000
}

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Guild {
    pub id: GuildId,
    pub afk_channel_id: Option<ChannelId>,
    pub afk_timeout: u64,
    pub application_id: Option<ApplicationId>,
    pub banner: Option<String>,
    #[cfg_attr(feature = "serde-support", serde(with = "serde_mappable_seq"))]
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
    pub joined_at: String,
    #[cfg_attr(feature = "serde-support", serde(default))]
    pub large: bool,
    // Not documented so I marked it as optional.
    pub lazy: Option<bool>,
    pub max_members: Option<u64>,
    #[cfg_attr(feature = "serde-support", serde(default = "default_max_presences"))]
    pub max_presences: u64,
    pub member_count: Option<u64>,
    #[cfg_attr(feature = "serde-support", serde(with = "serde_mappable_seq"))]
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
    #[cfg_attr(feature = "serde-support", serde(with = "serde_mappable_seq"))]
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
    #[cfg_attr(feature = "serde-support", serde(with = "serde_mappable_seq"))]
    pub voice_states: HashMap<UserId, VoiceState>,
    pub vanity_url_code: Option<String>,
    pub widget_channel_id: Option<ChannelId>,
    pub widget_enabled: Option<bool>,
}
