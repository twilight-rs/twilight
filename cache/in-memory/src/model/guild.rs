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
    /// ID of the guild.
    pub id: GuildId,
    /// ID of the AFK channel.
    pub afk_channel_id: Option<ChannelId>,
    /// AFK timeout in seconds.
    pub afk_timeout: u64,
    /// For bot created guilds, the ID of the creating application.
    pub application_id: Option<ApplicationId>,
    /// Banner hash.
    ///
    /// See [Discord Docs/Image Formatting].
    ///
    /// [Discord Docs/Image Formatting]: https://discord.com/developers/docs/reference#image-formatting
    pub banner: Option<String>,
    /// Default message notification level.
    pub default_message_notifications: DefaultMessageNotificationLevel,
    /// For Community guilds, the description.
    pub description: Option<String>,
    /// For discoverable guilds, the discovery splash hash.
    ///
    /// See [Discord Docs/Image Formatting].
    ///
    /// [Discord Docs/Image Formatting]: https://discord.com/developers/docs/reference#image-formatting
    pub discovery_splash: Option<String>,
    /// Explicit content filter level.
    pub explicit_content_filter: ExplicitContentFilter,
    /// Enabled [guild features].
    ///
    /// [guild features]: https://discord.com/developers/docs/resources/guild#guild-object-guild-features
    pub features: Vec<String>,
    /// Icon hash.
    ///
    /// See [Discord Docs/Image Formatting].
    ///
    /// [Discord Docs/Image Formatting]: https://discord.com/developers/docs/reference#image-formatting
    pub icon: Option<String>,
    /// ISO 8601 timestamp of the user's join date.
    pub joined_at: Option<String>,
    /// Whether this guild is "large".
    pub large: bool,
    /// Maximum members.
    pub max_members: Option<u64>,
    /// Maximum presences.
    pub max_presences: Option<u64>,
    /// Total number of members in the guild.
    pub member_count: Option<u64>,
    /// Required MFA level.
    pub mfa_level: MfaLevel,
    /// Name of the guild.
    pub name: String,
    /// NSFW level.
    pub nsfw_level: NSFWLevel,
    /// Whether the current user is the owner of the guild.
    pub owner: Option<bool>,
    /// ID of the guild's owner.
    pub owner_id: UserId,
    /// Total permissions for the current user in the guild, excluding overwrites.
    pub permissions: Option<Permissions>,
    /// For Community guilds, the preferred locale.
    ///
    /// Used in server discovery and notices from Discord. Defaults to "en-US".
    pub preferred_locale: String,
    /// Number of boosts this guild currently has.
    pub premium_subscription_count: Option<u64>,
    /// Server boost level.
    pub premium_tier: PremiumTier,
    /// For Community guilds, the ID of the rules channel.
    pub rules_channel_id: Option<ChannelId>,
    /// Splash hash.
    ///
    /// See [Discord Docs/Image Formatting].
    ///
    /// [Discord Docs/Image Formatting]: https://discord.com/developers/docs/reference#image-formatting
    pub splash: Option<String>,
    /// System channel flags.
    pub system_channel_flags: SystemChannelFlags,
    /// ID of the channel where notices are posted.
    ///
    /// Example notices include welcome messages and boost events.
    pub system_channel_id: Option<ChannelId>,
    /// Whether the guild is unavailable due to an outage.
    pub unavailable: bool,
    /// Vanity URL code.
    pub vanity_url_code: Option<String>,
    /// Required verification level.
    pub verification_level: VerificationLevel,
    /// ID of the channel that a widget generates an invite to.
    pub widget_channel_id: Option<ChannelId>,
    /// Whether the widget is enabled.
    pub widget_enabled: Option<bool>,
}
