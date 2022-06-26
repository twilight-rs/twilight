use std::slice::Iter;

use serde::Serialize;
use twilight_model::{
    guild::{
        DefaultMessageNotificationLevel, ExplicitContentFilter, GuildFeature, MfaLevel, NSFWLevel,
        Permissions, PremiumTier, SystemChannelFlags, VerificationLevel,
    },
    id::{
        marker::{ApplicationMarker, ChannelMarker, GuildMarker, UserMarker},
        Id,
    },
    util::{ImageHash, Timestamp},
};

/// Represents a cached [`Guild`].
///
/// [`Guild`]: twilight_model::guild::Guild
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CachedGuild {
    pub(crate) afk_channel_id: Option<Id<ChannelMarker>>,
    pub(crate) afk_timeout: u64,
    pub(crate) application_id: Option<Id<ApplicationMarker>>,
    pub(crate) banner: Option<ImageHash>,
    pub(crate) default_message_notifications: DefaultMessageNotificationLevel,
    pub(crate) description: Option<String>,
    pub(crate) discovery_splash: Option<ImageHash>,
    pub(crate) explicit_content_filter: ExplicitContentFilter,
    pub(crate) features: Vec<GuildFeature>,
    pub(crate) icon: Option<ImageHash>,
    pub(crate) id: Id<GuildMarker>,
    pub(crate) joined_at: Option<Timestamp>,
    pub(crate) large: bool,
    pub(crate) max_members: Option<u64>,
    pub(crate) max_presences: Option<u64>,
    pub(crate) max_video_channel_users: Option<u64>,
    pub(crate) member_count: Option<u64>,
    pub(crate) mfa_level: MfaLevel,
    pub(crate) name: String,
    pub(crate) nsfw_level: NSFWLevel,
    pub(crate) owner_id: Id<UserMarker>,
    pub(crate) owner: Option<bool>,
    pub(crate) permissions: Option<Permissions>,
    pub(crate) preferred_locale: String,
    pub(crate) premium_progress_bar_enabled: bool,
    pub(crate) premium_subscription_count: Option<u64>,
    pub(crate) premium_tier: PremiumTier,
    pub(crate) rules_channel_id: Option<Id<ChannelMarker>>,
    pub(crate) splash: Option<ImageHash>,
    pub(crate) system_channel_id: Option<Id<ChannelMarker>>,
    pub(crate) system_channel_flags: SystemChannelFlags,
    pub(crate) unavailable: bool,
    pub(crate) vanity_url_code: Option<String>,
    pub(crate) verification_level: VerificationLevel,
    pub(crate) widget_channel_id: Option<Id<ChannelMarker>>,
    pub(crate) widget_enabled: Option<bool>,
}

impl CachedGuild {
    /// ID of the AFK channel.
    pub const fn afk_channel_id(&self) -> Option<Id<ChannelMarker>> {
        self.afk_channel_id
    }

    /// AFK timeout in seconds.
    pub const fn afk_timeout(&self) -> u64 {
        self.afk_timeout
    }

    /// For bot created guilds, the ID of the creating application.
    pub const fn application_id(&self) -> Option<Id<ApplicationMarker>> {
        self.application_id
    }

    /// Banner hash.
    ///
    /// See [Discord Docs/Image Formatting].
    ///
    /// [Discord Docs/Image Formatting]: https://discord.com/developers/docs/reference#image-formatting
    pub const fn banner(&self) -> Option<&ImageHash> {
        self.banner.as_ref()
    }

    /// Default message notification level.
    pub const fn default_message_notifications(&self) -> DefaultMessageNotificationLevel {
        self.default_message_notifications
    }

    /// For Community guilds, the description.
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// For discoverable guilds, the discovery splash hash.
    ///
    /// See [Discord Docs/Image Formatting].
    ///
    /// [Discord Docs/Image Formatting]: https://discord.com/developers/docs/reference#image-formatting
    pub const fn discovery_splash(&self) -> Option<&ImageHash> {
        self.discovery_splash.as_ref()
    }

    /// Explicit content filter level.
    pub const fn explicit_content_filter(&self) -> ExplicitContentFilter {
        self.explicit_content_filter
    }

    /// Enabled [guild features].
    ///
    /// [guild features]: https://discord.com/developers/docs/resources/guild#guild-object-guild-features
    pub fn features(&self) -> Features<'_> {
        Features {
            inner: self.features.iter(),
        }
    }

    /// Icon hash.
    ///
    /// See [Discord Docs/Image Formatting].
    ///
    /// [Discord Docs/Image Formatting]: https://discord.com/developers/docs/reference#image-formatting
    pub const fn icon(&self) -> Option<&ImageHash> {
        self.icon.as_ref()
    }

    /// ID of the guild.
    pub const fn id(&self) -> Id<GuildMarker> {
        self.id
    }

    /// [`Timestamp`] of the user's join date.
    pub const fn joined_at(&self) -> Option<Timestamp> {
        self.joined_at
    }

    /// Whether this guild is "large".
    pub const fn large(&self) -> bool {
        self.large
    }

    /// Maximum members.
    pub const fn max_members(&self) -> Option<u64> {
        self.max_members
    }

    /// Maximum presences.
    pub const fn max_presences(&self) -> Option<u64> {
        self.max_presences
    }

    /// Maximum number of users in a video channel.
    pub const fn max_video_channel_users(&self) -> Option<u64> {
        self.max_video_channel_users
    }

    /// Total number of members in the guild.
    pub const fn member_count(&self) -> Option<u64> {
        self.member_count
    }

    /// Required MFA level.
    pub const fn mfa_level(&self) -> MfaLevel {
        self.mfa_level
    }

    /// Name of the guild.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// NSFW level.
    pub const fn nsfw_level(&self) -> NSFWLevel {
        self.nsfw_level
    }

    /// Whether the current user is the owner of the guild.
    pub const fn owner(&self) -> Option<bool> {
        self.owner
    }

    /// ID of the guild's owner.
    pub const fn owner_id(&self) -> Id<UserMarker> {
        self.owner_id
    }

    /// Total permissions for the current user in the guild, excluding overwrites.
    pub const fn permissions(&self) -> Option<Permissions> {
        self.permissions
    }

    /// Preferred locale for Community guilds.
    ///
    /// Used in server discovery and notices from Discord. Defaults to "en-US".
    pub fn preferred_locale(&self) -> &str {
        &self.preferred_locale
    }

    /// Whether the premium progress bar is enabled.
    pub const fn premium_progress_bar_enabled(&self) -> bool {
        self.premium_progress_bar_enabled
    }

    /// Number of boosts this guild currently has.
    pub const fn premium_subscription_count(&self) -> Option<u64> {
        self.premium_subscription_count
    }

    /// Server boost level.
    pub const fn premium_tier(&self) -> PremiumTier {
        self.premium_tier
    }

    /// For Community guilds, the ID of the rules channel.
    pub const fn rules_channel_id(&self) -> Option<Id<ChannelMarker>> {
        self.rules_channel_id
    }

    /// Splash hash.
    ///
    /// See [Discord Docs/Image Formatting].
    ///
    /// [Discord Docs/Image Formatting]: https://discord.com/developers/docs/reference#image-formatting
    pub const fn splash(&self) -> Option<&ImageHash> {
        self.splash.as_ref()
    }

    /// ID of the channel where notices are posted.
    ///
    /// Example notices include welcome messages and boost events.
    pub const fn system_channel_id(&self) -> Option<Id<ChannelMarker>> {
        self.system_channel_id
    }

    /// System channel flags.
    pub const fn system_channel_flags(&self) -> SystemChannelFlags {
        self.system_channel_flags
    }

    /// Whether the guild is unavailable due to an outage.
    pub const fn unavailable(&self) -> bool {
        self.unavailable
    }

    /// Vanity URL code.
    pub fn vanity_url_code(&self) -> Option<&str> {
        self.vanity_url_code.as_deref()
    }

    /// Required verification level.
    pub const fn verification_level(&self) -> VerificationLevel {
        self.verification_level
    }

    /// ID of the channel that a widget generates an invite to.
    pub const fn widget_channel_id(&self) -> Option<Id<ChannelMarker>> {
        self.widget_channel_id
    }

    /// Whether the widget is enabled.
    pub const fn widget_enabled(&self) -> Option<bool> {
        self.widget_enabled
    }
}

pub struct Features<'a> {
    inner: Iter<'a, GuildFeature>,
}

impl<'a> Iterator for Features<'a> {
    type Item = &'a GuildFeature;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
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
        max_video_channel_users,
        member_count,
        mfa_level,
        name,
        nsfw_level,
        owner_id,
        owner,
        permissions,
        preferred_locale,
        premium_progress_bar_enabled,
        premium_subscription_count,
        premium_tier,
        rules_channel_id,
        splash,
        system_channel_id,
        system_channel_flags,
        unavailable,
        vanity_url_code,
        verification_level,
        widget_channel_id,
        widget_enabled
    );
    assert_impl_all!(
        CachedGuild: Clone,
        Debug,
        Eq,
        PartialEq,
        Send,
        Serialize,
        Sync,
    );
}
