use std::slice::Iter;

use serde::Serialize;
use twilight_model::{
    gateway::payload::incoming::GuildUpdate,
    guild::{
        scheduled_event::GuildScheduledEvent, AfkTimeout, DefaultMessageNotificationLevel,
        ExplicitContentFilter, Guild, GuildFeature, MfaLevel, NSFWLevel, Permissions, PremiumTier,
        SystemChannelFlags, VerificationLevel,
    },
    id::{
        marker::{ApplicationMarker, ChannelMarker, GuildMarker, UserMarker},
        Id,
    },
    util::{ImageHash, Timestamp},
};

use crate::CacheableGuild;

/// Represents a cached [`Guild`].
///
/// [`Guild`]: twilight_model::guild::Guild
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CachedGuild {
    pub(crate) afk_channel_id: Option<Id<ChannelMarker>>,
    pub(crate) afk_timeout: AfkTimeout,
    pub(crate) application_id: Option<Id<ApplicationMarker>>,
    pub(crate) banner: Option<ImageHash>,
    pub(crate) default_message_notifications: DefaultMessageNotificationLevel,
    pub(crate) description: Option<String>,
    pub(crate) discovery_splash: Option<ImageHash>,
    pub(crate) explicit_content_filter: ExplicitContentFilter,
    pub(crate) features: Vec<GuildFeature>,
    pub(crate) guild_scheduled_events: Vec<GuildScheduledEvent>,
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
    pub(crate) public_updates_channel_id: Option<Id<ChannelMarker>>,
    pub(crate) rules_channel_id: Option<Id<ChannelMarker>>,
    pub(crate) safety_alerts_channel_id: Option<Id<ChannelMarker>>,
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
    pub const fn afk_timeout(&self) -> AfkTimeout {
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

    /// Scheduled guild events.
    pub fn guild_scheduled_events(&self) -> &[GuildScheduledEvent] {
        &self.guild_scheduled_events
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

    /// ID of the where moderators of Community guilds receive notices from
    /// Discord.
    pub const fn public_updates_channel_id(&self) -> Option<Id<ChannelMarker>> {
        self.public_updates_channel_id
    }

    /// For Community guilds, the ID of the rules channel.
    pub const fn rules_channel_id(&self) -> Option<Id<ChannelMarker>> {
        self.rules_channel_id
    }

    /// The ID of the channel where admins and moderators of Community guilds receive safety alerts from Discord.
    pub const fn safety_alerts_channel_id(&self) -> Option<Id<ChannelMarker>> {
        self.safety_alerts_channel_id
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

impl From<Guild> for CachedGuild {
    fn from(guild: Guild) -> Self {
        let Guild {
            afk_channel_id,
            afk_timeout,
            application_id,
            approximate_member_count: _,
            approximate_presence_count: _,
            banner,
            default_message_notifications,
            description,
            discovery_splash,
            explicit_content_filter,
            features,
            guild_scheduled_events,
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
            owner,
            owner_id,
            permissions,
            preferred_locale,
            premium_progress_bar_enabled,
            premium_subscription_count,
            premium_tier,
            public_updates_channel_id,
            rules_channel_id,
            safety_alerts_channel_id,
            splash,
            system_channel_flags,
            system_channel_id,
            unavailable,
            vanity_url_code,
            verification_level,
            widget_channel_id,
            widget_enabled,
            ..
        } = guild;

        Self {
            afk_channel_id,
            afk_timeout,
            application_id,
            banner,
            default_message_notifications,
            description,
            discovery_splash,
            explicit_content_filter,
            features,
            guild_scheduled_events,
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
            owner,
            owner_id,
            permissions,
            preferred_locale,
            premium_progress_bar_enabled,
            premium_subscription_count,
            premium_tier,
            public_updates_channel_id,
            rules_channel_id,
            safety_alerts_channel_id,
            splash,
            system_channel_flags,
            system_channel_id,
            unavailable,
            vanity_url_code,
            verification_level,
            widget_channel_id,
            widget_enabled,
        }
    }
}

impl PartialEq<Guild> for CachedGuild {
    fn eq(&self, other: &Guild) -> bool {
        self.afk_channel_id == other.afk_channel_id
            && self.afk_timeout == other.afk_timeout
            && self.application_id == other.application_id
            && self.banner == other.banner
            && self.default_message_notifications == other.default_message_notifications
            && self.description == other.description
            && self.discovery_splash == other.discovery_splash
            && self.explicit_content_filter == other.explicit_content_filter
            && self.features == other.features
            && self.icon == other.icon
            && self.joined_at == other.joined_at
            && self.large == other.large
            && self.max_members == other.max_members
            && self.max_presences == other.max_presences
            && self.max_video_channel_users == other.max_video_channel_users
            && self.member_count == other.member_count
            && self.mfa_level == other.mfa_level
            && self.name == other.name
            && self.nsfw_level == other.nsfw_level
            && self.owner_id == other.owner_id
            && self.owner == other.owner
            && self.permissions == other.permissions
            && self.preferred_locale == other.preferred_locale
            && self.premium_progress_bar_enabled == other.premium_progress_bar_enabled
            && self.premium_subscription_count == other.premium_subscription_count
            && self.premium_tier == other.premium_tier
            && self.public_updates_channel_id == other.public_updates_channel_id
            && self.rules_channel_id == other.rules_channel_id
            && self.safety_alerts_channel_id == other.safety_alerts_channel_id
            && self.splash == other.splash
            && self.system_channel_id == other.system_channel_id
            && self.system_channel_flags == other.system_channel_flags
            && self.unavailable == other.unavailable
            && self.vanity_url_code == other.vanity_url_code
            && self.verification_level == other.verification_level
            && self.widget_channel_id == other.widget_channel_id
            && self.widget_enabled == other.widget_enabled
    }
}

impl CacheableGuild for CachedGuild {
    fn id(&self) -> Id<GuildMarker> {
        self.id
    }

    #[cfg(feature = "permission-calculator")]
    fn owner_id(&self) -> Id<UserMarker> {
        self.owner_id
    }

    fn set_unavailable(&mut self, unavailable: bool) {
        self.unavailable = unavailable;
    }

    fn update_with_guild_update(&mut self, guild_update: &GuildUpdate) {
        self.afk_channel_id = guild_update.afk_channel_id;
        self.afk_timeout = guild_update.afk_timeout;
        self.banner = guild_update.banner;
        self.default_message_notifications = guild_update.default_message_notifications;
        self.description = guild_update.description.clone();
        self.features = guild_update.features.clone();
        self.icon = guild_update.icon;
        self.max_members = guild_update.max_members;
        self.max_presences = Some(guild_update.max_presences.unwrap_or(25000));
        self.mfa_level = guild_update.mfa_level;
        self.name = guild_update.name.clone();
        self.nsfw_level = guild_update.nsfw_level;
        self.owner = guild_update.owner;
        self.owner_id = guild_update.owner_id;
        self.permissions = guild_update.permissions;
        self.preferred_locale = guild_update.preferred_locale.clone();
        self.premium_tier = guild_update.premium_tier;
        self.premium_subscription_count
            .replace(guild_update.premium_subscription_count.unwrap_or_default());
        self.splash = guild_update.splash;
        self.system_channel_id = guild_update.system_channel_id;
        self.verification_level = guild_update.verification_level;
        self.vanity_url_code = guild_update.vanity_url_code.clone();
        self.widget_channel_id = guild_update.widget_channel_id;
        self.widget_enabled = guild_update.widget_enabled;
    }

    fn increase_member_count(&mut self, amount: u64) {
        self.member_count = self.member_count.map(|count| count + amount);
    }

    fn decrease_member_count(&mut self, amount: u64) {
        self.member_count = self.member_count.map(|count| count - amount);
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
    use super::{CachedGuild, Features};
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
    assert_impl_all!(Features<'_>: Iterator, Send, Sync);
}
