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
    unavailable_guild::UnavailableGuild,
    verification_level::VerificationLevel,
};

use crate::{
    channel::GuildChannel,
    gateway::presence::Presence,
    id::{ApplicationId, ChannelId, GuildId, UserId},
    voice::VoiceState,
};
use serde::{Deserialize, Serialize};

fn default_max_presences() -> u64 {
    5000
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Guild {
    pub id: GuildId,
    pub afk_channel_id: Option<ChannelId>,
    pub afk_timeout: u64,
    pub application_id: Option<ApplicationId>,
    pub banner: Option<String>,
    pub channels: Vec<GuildChannel>,
    pub default_message_notifications: DefaultMessageNotificationLevel,
    pub description: Option<String>,
    pub embed_channel_id: Option<ChannelId>,
    pub embed_enabled: Option<bool>,
    pub emojis: Vec<Emoji>,
    pub explicit_content_filter: ExplicitContentFilter,
    pub features: Vec<String>,
    pub icon: Option<String>,
    #[cfg(feature = "chrono")]
    pub joined_at: chrono::DateTime<chrono::FixedOffset>,
    #[cfg(not(feature = "chrono"))]
    pub joined_at: String,
    pub large: bool,
    pub max_members: Option<u64>,
    #[serde(default = "default_max_presences")]
    pub max_presences: u64,
    pub member_count: u64,
    pub members: Vec<Member>,
    pub mfa_level: MfaLevel,
    pub name: String,
    pub owner: Option<bool>,
    pub owner_id: UserId,
    pub permissions: Option<Permissions>,
    pub preferred_locale: String,
    #[serde(default)]
    pub premium_subscription_count: u64,
    #[serde(default)]
    pub premium_tier: PremiumTier,
    pub presences: Vec<Presence>,
    pub region: String,
    pub roles: Vec<Role>,
    pub splash: Option<String>,
    pub system_channel_id: Option<ChannelId>,
    pub unavailable: bool,
    pub verification_level: VerificationLevel,
    pub voice_states: Vec<VoiceState>,
    pub vanity_url_code: Option<String>,
    pub widget_channel_id: Option<ChannelId>,
    pub widget_enabled: Option<bool>,
}
