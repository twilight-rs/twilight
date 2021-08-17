use std::slice::Iter;

use serde::Serialize;
use twilight_model::{
    guild::{
        DefaultMessageNotificationLevel, ExplicitContentFilter, MfaLevel, NSFWLevel, Permissions,
        PremiumTier, SystemChannelFlags, VerificationLevel,
    },
    id::{ApplicationId, ChannelId, GuildId, UserId},
};

/// Represents a cached [`Guild`].
///
/// [`Guild`]: twilight_model::guild::Guild
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CachedGuild {
    pub(crate) afk_channel_id: Option<ChannelId>,
    pub(crate) afk_timeout: u64,
    pub(crate) application_id: Option<ApplicationId>,
    pub(crate) banner: Option<String>,
    pub(crate) default_message_notifications: DefaultMessageNotificationLevel,
    pub(crate) description: Option<String>,
    pub(crate) discovery_splash: Option<String>,
    pub(crate) explicit_content_filter: ExplicitContentFilter,
    pub(crate) features: Vec<String>,
    pub(crate) icon: Option<String>,
    pub(crate) id: GuildId,
    pub(crate) joined_at: Option<String>,
    pub(crate) large: bool,
    pub(crate) max_members: Option<u64>,
    pub(crate) max_presences: Option<u64>,
    pub(crate) member_count: Option<u64>,
    pub(crate) mfa_level: MfaLevel,
    pub(crate) name: String,
    pub(crate) nsfw_level: NSFWLevel,
    pub(crate) owner_id: UserId,
    pub(crate) owner: Option<bool>,
    pub(crate) permissions: Option<Permissions>,
    pub(crate) preferred_locale: String,
    pub(crate) premium_subscription_count: Option<u64>,
    pub(crate) premium_tier: PremiumTier,
    pub(crate) rules_channel_id: Option<ChannelId>,
    pub(crate) splash: Option<String>,
    pub(crate) system_channel_id: Option<ChannelId>,
    pub(crate) system_channel_flags: SystemChannelFlags,
    pub(crate) unavailable: bool,
    pub(crate) vanity_url_code: Option<String>,
    pub(crate) verification_level: VerificationLevel,
    pub(crate) widget_channel_id: Option<ChannelId>,
    pub(crate) widget_enabled: Option<bool>,
}

impl CachedGuild {
    /// ID of the AFK channel.
    pub const fn afk_channel_id(&self) -> Option<ChannelId> {
        self.afk_channel_id
    }

    /// AFK timeout in seconds.
    pub const fn afk_timeout(&self) -> u64 {
        self.afk_timeout
    }

    /// For bot created guilds, the ID of the creating application.
    pub const fn application_id(&self) -> Option<ApplicationId> {
        self.application_id
    }

    /// Banner hash.
    ///
    /// See [Discord Docs/Image Formatting].
    ///
    /// [Discord Docs/Image Formatting]: https://discord.com/developers/docs/reference#image-formatting
    pub fn banner(&self) -> Option<&str> {
        self.banner.as_deref()
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
    pub fn discovery_splash(&self) -> Option<&str> {
        self.discovery_splash.as_deref()
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
    pub fn icon(&self) -> Option<&str> {
        self.icon.as_deref()
    }

    /// ID of the guild.
    pub const fn id(&self) -> GuildId {
        self.id
    }

    /// ISO 8601 timestamp of the user's join date.
    pub fn joined_at(&self) -> Option<&str> {
        self.joined_at.as_deref()
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
    pub const fn owner_id(&self) -> UserId {
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

    /// Number of boosts this guild currently has.
    pub const fn premium_subscription_count(&self) -> Option<u64> {
        self.premium_subscription_count
    }

    /// Server boost level.
    pub const fn premium_tier(&self) -> PremiumTier {
        self.premium_tier
    }

    /// For Community guilds, the ID of the rules channel.
    pub const fn rules_channel_id(&self) -> Option<ChannelId> {
        self.rules_channel_id
    }

    /// Splash hash.
    ///
    /// See [Discord Docs/Image Formatting].
    ///
    /// [Discord Docs/Image Formatting]: https://discord.com/developers/docs/reference#image-formatting
    pub fn splash(&self) -> Option<&str> {
        self.splash.as_deref()
    }

    /// ID of the channel where notices are posted.
    ///
    /// Example notices include welcome messages and boost events.
    pub const fn system_channel_id(&self) -> Option<ChannelId> {
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
    pub const fn widget_channel_id(&self) -> Option<ChannelId> {
        self.widget_channel_id
    }

    /// Whether the widget is enabled.
    pub const fn widget_enabled(&self) -> Option<bool> {
        self.widget_enabled
    }
}

pub struct Features<'a> {
    inner: Iter<'a, String>,
}

impl<'a> Iterator for Features<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(AsRef::as_ref)
    }
}
