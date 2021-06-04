use serde::Serialize;
use twilight_model::{
    guild::{
        DefaultMessageNotificationLevel, ExplicitContentFilter, MfaLevel, Permissions, PremiumTier,
        SystemChannelFlags, VerificationLevel,
    },
    id::{ApplicationId, ChannelId, GuildId, UserId},
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CachedGuild {
    pub id: GuildId,
    pub afk_channel_id: Option<ChannelId>,
    pub afk_timeout: u64,
    /// Approximate number of slash commands in the guild.
    ///
    /// This field is only present in [`GuildCreate`] events.
    ///
    /// **Note** that this is an *approximate* number of slash commands in the
    /// guild; the value may not be accurate.
    ///
    /// [`GuildCreate`]: twilight_model::gateway::payload::GuildCreate
    pub application_command_count: Option<u64>,
    pub application_id: Option<ApplicationId>,
    pub banner: Option<String>,
    pub default_message_notifications: DefaultMessageNotificationLevel,
    pub description: Option<String>,
    pub discovery_splash: Option<String>,
    pub explicit_content_filter: ExplicitContentFilter,
    pub features: Vec<String>,
    pub icon: Option<String>,
    pub joined_at: Option<String>,
    pub large: bool,
    pub max_members: Option<u64>,
    pub max_presences: Option<u64>,
    pub member_count: Option<u64>,
    pub mfa_level: MfaLevel,
    pub name: String,
    pub nsfw: bool,
    pub owner: Option<bool>,
    pub owner_id: UserId,
    pub permissions: Option<Permissions>,
    pub preferred_locale: String,
    pub premium_subscription_count: Option<u64>,
    pub premium_tier: PremiumTier,
    pub region: String,
    pub rules_channel_id: Option<ChannelId>,
    pub splash: Option<String>,
    pub system_channel_id: Option<ChannelId>,
    pub system_channel_flags: SystemChannelFlags,
    pub unavailable: bool,
    pub verification_level: VerificationLevel,
    pub vanity_url_code: Option<String>,
    pub widget_channel_id: Option<ChannelId>,
    pub widget_enabled: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::CachedGuild;
    use serde::Serialize;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::fmt::Debug;

    assert_fields!(
        CachedGuild: afk_channel_id,
        afk_timeout,
        application_command_count,
        application_id,
        banner,
        default_message_notifications,
        description,
        discovery_splash,
        explicit_content_filter,
        features,
        icon,
        id,
        joined_at,
        large,
        max_members,
        max_presences,
        member_count,
        mfa_level,
        name,
        nsfw,
        owner,
        owner_id,
        permissions,
        preferred_locale,
        premium_subscription_count,
        premium_tier,
        region,
        rules_channel_id,
        splash,
        system_channel_id,
        system_channel_flags,
        unavailable,
        verification_level,
        vanity_url_code,
        widget_channel_id,
        widget_enabled
    );
    assert_impl_all!(
        CachedGuild: Clone,
        Debug,
        Eq,
        PartialEq,
        Serialize,
        Send,
        Sync
    );
}
