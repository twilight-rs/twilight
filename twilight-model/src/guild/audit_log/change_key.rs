use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Type of [`AuditLogChange`].
///
/// For additional information refer to [Discord Docs/Audit Log Change Key][1].
///
/// [`AuditLogChange`]: super::AuditLogChange
/// [1]: https://discord.com/developers/docs/resources/audit-log#audit-log-change-object-audit-log-change-key
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub enum AuditLogChangeKey {
    /// AFK voice channel for a guild.
    AfkChannelId,
    /// Timeout to cause a user to be moved to an AFK voice channel.
    AfkTimeout,
    /// Allowed permissions of a permission overwrite target.
    Allow,
    /// ID of an application.
    ApplicationId,
    /// Thread was archived or unarchived.
    Archived,
    /// Asset of a sticker.
    ///
    /// Empty string.
    Asset,
    /// Auto archive duration of a thread.
    AutoArchiveDuration,
    /// Availability of a sticker.
    Available,
    /// Hash of an avatar.
    AvatarHash,
    /// Hash of a guild banner.
    BannerHash,
    /// Bitrate of an audio channel.
    Bitrate,
    /// Channel for an invite code.
    ChannelId,
    /// Code of an invite.
    Code,
    /// Color of a role.
    Color,
    /// Permissions for a command were updated.
    CommandId,
    /// Member timeout state changed.
    CommunicationDisabledUntil,
    /// Whether a user is guild deafened.
    Deaf,
    /// Default auto archive duration for new threads.
    DefaultAutoArchiveDuration,
    /// Default message notification level for a guild.
    DefaultMessageNotifications,
    /// Denied permissions of a permission overwrite target.
    Deny,
    /// Description of a guild.
    Description,
    /// Hash of a guild's discovery splash.
    DiscoverySplashHash,
    /// Whether emoticons are enabled.
    EnableEmoticons,
    /// Entity type of guild scheduled event was changed.
    EntityType,
    /// Behavior of the expiration of an integration.
    ExpireBehavior,
    /// Grace period of the expiration of an integration.
    ExpireGracePeriod,
    /// Explicit content filter level of a guild.
    ExplicitContentFilter,
    /// Format type of a sticker.
    FormatType,
    /// Guild that a sticker is in.
    GuildId,
    /// Whether a role is hoisted.
    Hoist,
    /// Hash of a guild icon.
    IconHash,
    /// ID of an entity.
    Id,
    /// Hash of a guild scheduled event cover.
    ImageHash,
    /// Invitable state of a private thread.
    Invitable,
    /// ID of the user who created an invite.
    InviterId,
    /// Channel ID for a scheduled event changed.
    Location,
    /// Thread was locked or unlocked.
    Locked,
    /// Maximum age of an invite.
    MaxAge,
    /// Maximum uses of an invite.
    MaxUses,
    /// Whether a role can be mentioned in a message.
    Mentionable,
    /// Multi-Factor Authentication level required of a guild's moderators.
    MfaLevel,
    /// Whether a user is guild muted.
    Mute,
    /// Name of an entity such as a channel or role.
    Name,
    /// Nickname of a member.
    Nick,
    /// Whether a channel is NSFW.
    Nsfw,
    /// NSFW level of a guild.
    NsfwLevel,
    /// ID of the owner of a guild.
    OwnerId,
    /// Permission overwrites on a channel changed.
    PermissionOverwrites,
    /// Default permissions of a role.
    Permissions,
    /// Position of an entity such as a channel or role.
    Position,
    /// Preferred locale of a guild.
    PreferredLocale,
    /// Privacy level of a stage instance.
    PrivacyLevel,
    /// Number of days' worth of inactivity for a guild prune.
    PruneDeleteDays,
    /// ID of a guild's public updates channel.
    PublicUpdatesChannelId,
    /// Ratelimit per user in a textual channel.
    RateLimitPerUser,
    /// Region of a guild changed.
    Region,
    /// Role added to a user.
    #[serde(rename = "$add")]
    RoleAdded,
    /// Role removed from a user.
    #[serde(rename = "$remove")]
    RoleRemoved,
    /// ID of a guild's rules channel.
    RulesChannelId,
    /// Hash of a guild's splash.
    SplashHash,
    /// Status of guild scheduled event was changed.
    Status,
    /// ID of a guild's system channel.
    SystemChannelId,
    /// Related emoji of a sticker.
    Tags,
    /// Whether an invite is temporary.
    Temporary,
    /// Topic of a textual channel.
    Topic,
    /// Type of a created entity.
    Type,
    /// Role unicode emoji.
    UnicodeEmoji,
    /// Maximum number of users in a voice channel.
    UserLimit,
    /// Number of uses of an invite.
    Uses,
    /// Code of a guild's vanity invite.
    VanityUrlCode,
    /// Required verification level of new members in a guild.
    VerificationLevel,
    /// Channel ID of a widget.
    WidgetChannelId,
    /// Whether a widget is enabled.
    WidgetEnabled,
}

impl AuditLogChangeKey {
    /// Raw name of the key.
    ///
    /// The raw names of keys are in `snake_case` form.
    ///
    /// # Examples
    ///
    /// Check the names of the [`Allow`] and [`BannerHash`] keys:
    ///
    /// ```
    /// use twilight_model::guild::audit_log::AuditLogChangeKey;
    ///
    /// assert_eq!("allow", AuditLogChangeKey::Allow.name());
    /// assert_eq!("banner_hash", AuditLogChangeKey::BannerHash.name());
    /// ```
    ///
    /// [`Allow`]: Self::Allow
    /// [`BannerHash`]: Self::BannerHash
    pub const fn name(self) -> &'static str {
        match self {
            Self::AfkChannelId => "afk_channel_id",
            Self::AfkTimeout => "afk_timeout",
            Self::Allow => "allow",
            Self::ApplicationId => "application_id",
            Self::Archived => "archived",
            Self::Asset => "asset",
            Self::AutoArchiveDuration => "auto_archive_duration",
            Self::Available => "available",
            Self::AvatarHash => "avatar_hash",
            Self::BannerHash => "banner_hash",
            Self::Bitrate => "bitrate",
            Self::ChannelId => "channel_id",
            Self::Code => "code",
            Self::Color => "color",
            Self::CommandId => "command_id",
            Self::CommunicationDisabledUntil => "communication_disabled_until",
            Self::Deaf => "deaf",
            Self::DefaultAutoArchiveDuration => "default_auto_archive_duration",
            Self::DefaultMessageNotifications => "default_message_notifications",
            Self::Deny => "deny",
            Self::Description => "description",
            Self::DiscoverySplashHash => "discovery_splash_hash",
            Self::EnableEmoticons => "enable_emoticons",
            Self::EntityType => "entity_type",
            Self::ExpireBehavior => "expire_behavior",
            Self::ExpireGracePeriod => "expire_grace_period",
            Self::ExplicitContentFilter => "explicit_content_filter",
            Self::FormatType => "format_type",
            Self::GuildId => "guild_id",
            Self::Hoist => "hoist",
            Self::IconHash => "icon_hash",
            Self::Id => "id",
            Self::ImageHash => "image_hash",
            Self::Invitable => "invitable",
            Self::InviterId => "inviter_id",
            Self::Location => "location",
            Self::Locked => "locked",
            Self::MaxAge => "max_age",
            Self::MaxUses => "max_uses",
            Self::Mentionable => "mentionable",
            Self::MfaLevel => "mfa_level",
            Self::Mute => "mute",
            Self::Name => "name",
            Self::Nick => "nick",
            Self::Nsfw => "nsfw",
            Self::NsfwLevel => "nsfw_level",
            Self::OwnerId => "owner_id",
            Self::PermissionOverwrites => "permission_overwrites",
            Self::Permissions => "permissions",
            Self::Position => "position",
            Self::PreferredLocale => "preferred_locale",
            Self::PrivacyLevel => "privacy_level",
            Self::PruneDeleteDays => "prune_delete_days",
            Self::PublicUpdatesChannelId => "public_updates_channel_id",
            Self::RateLimitPerUser => "rate_limit_per_user",
            Self::Region => "region",
            Self::RoleAdded => "$add",
            Self::RoleRemoved => "$remove",
            Self::RulesChannelId => "rules_channel_id",
            Self::SplashHash => "splash_hash",
            Self::Status => "status",
            Self::SystemChannelId => "system_channel_id",
            Self::Tags => "tags",
            Self::Temporary => "temporary",
            Self::Topic => "topic",
            Self::Type => "type",
            Self::UnicodeEmoji => "unicode_emoji",
            Self::UserLimit => "user_limit",
            Self::Uses => "uses",
            Self::VanityUrlCode => "vanity_url_code",
            Self::VerificationLevel => "verification_level",
            Self::WidgetChannelId => "widget_channel_id",
            Self::WidgetEnabled => "widget_enabled",
        }
    }
}

impl Display for AuditLogChangeKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::AuditLogChangeKey;
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::assert_impl_all;
    use std::{
        fmt::{Debug, Display},
        hash::Hash,
    };

    assert_impl_all!(
        AuditLogChangeKey: Clone,
        Copy,
        Debug,
        Deserialize<'static>,
        Display,
        Eq,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync
    );

    #[test]
    fn name() {
        assert_eq!("afk_channel_id", AuditLogChangeKey::AfkChannelId.name());
        assert_eq!("afk_timeout", AuditLogChangeKey::AfkTimeout.name());
        assert_eq!("allow", AuditLogChangeKey::Allow.name());
        assert_eq!("application_id", AuditLogChangeKey::ApplicationId.name());
        assert_eq!("avatar_hash", AuditLogChangeKey::AvatarHash.name());
        assert_eq!("banner_hash", AuditLogChangeKey::BannerHash.name());
        assert_eq!("bitrate", AuditLogChangeKey::Bitrate.name());
        assert_eq!("channel_id", AuditLogChangeKey::ChannelId.name());
        assert_eq!("code", AuditLogChangeKey::Code.name());
        assert_eq!("color", AuditLogChangeKey::Color.name());
        assert_eq!("command_id", AuditLogChangeKey::CommandId.name());
        assert_eq!(
            "communication_disabled_until",
            AuditLogChangeKey::CommunicationDisabledUntil.name()
        );
        assert_eq!("deaf", AuditLogChangeKey::Deaf.name());
        assert_eq!(
            "default_message_notifications",
            AuditLogChangeKey::DefaultMessageNotifications.name()
        );
        assert_eq!("deny", AuditLogChangeKey::Deny.name());
        assert_eq!("description", AuditLogChangeKey::Description.name());
        assert_eq!(
            "discovery_splash_hash",
            AuditLogChangeKey::DiscoverySplashHash.name()
        );
        assert_eq!(
            "enable_emoticons",
            AuditLogChangeKey::EnableEmoticons.name()
        );
        assert_eq!("expire_behavior", AuditLogChangeKey::ExpireBehavior.name());
        assert_eq!(
            "expire_grace_period",
            AuditLogChangeKey::ExpireGracePeriod.name()
        );
        assert_eq!(
            "explicit_content_filter",
            AuditLogChangeKey::ExplicitContentFilter.name()
        );
        assert_eq!("hoist", AuditLogChangeKey::Hoist.name());
        assert_eq!("icon_hash", AuditLogChangeKey::IconHash.name());
        assert_eq!("id", AuditLogChangeKey::Id.name());
        assert_eq!("image_hash", AuditLogChangeKey::ImageHash.name());
        assert_eq!("invitable", AuditLogChangeKey::Invitable.name());
        assert_eq!("inviter_id", AuditLogChangeKey::InviterId.name());
        assert_eq!("max_age", AuditLogChangeKey::MaxAge.name());
        assert_eq!("max_uses", AuditLogChangeKey::MaxUses.name());
        assert_eq!("mentionable", AuditLogChangeKey::Mentionable.name());
        assert_eq!("mfa_level", AuditLogChangeKey::MfaLevel.name());
        assert_eq!("mute", AuditLogChangeKey::Mute.name());
        assert_eq!("name", AuditLogChangeKey::Name.name());
        assert_eq!("nick", AuditLogChangeKey::Nick.name());
        assert_eq!("nsfw_level", AuditLogChangeKey::NsfwLevel.name());
        assert_eq!("owner_id", AuditLogChangeKey::OwnerId.name());
        assert_eq!("permissions", AuditLogChangeKey::Permissions.name());
        assert_eq!("position", AuditLogChangeKey::Position.name());
        assert_eq!(
            "preferred_locale",
            AuditLogChangeKey::PreferredLocale.name()
        );
        assert_eq!("privacy_level", AuditLogChangeKey::PrivacyLevel.name());
        assert_eq!(
            "prune_delete_days",
            AuditLogChangeKey::PruneDeleteDays.name()
        );
        assert_eq!(
            "public_updates_channel_id",
            AuditLogChangeKey::PublicUpdatesChannelId.name()
        );
        assert_eq!(
            "rate_limit_per_user",
            AuditLogChangeKey::RateLimitPerUser.name()
        );
        assert_eq!("$add", AuditLogChangeKey::RoleAdded.name());
        assert_eq!("$remove", AuditLogChangeKey::RoleRemoved.name());
        assert_eq!("rules_channel_id", AuditLogChangeKey::RulesChannelId.name());
        assert_eq!("splash_hash", AuditLogChangeKey::SplashHash.name());
        assert_eq!(
            "system_channel_id",
            AuditLogChangeKey::SystemChannelId.name()
        );
        assert_eq!("temporary", AuditLogChangeKey::Temporary.name());
        assert_eq!("topic", AuditLogChangeKey::Topic.name());
        assert_eq!("type", AuditLogChangeKey::Type.name());
        assert_eq!("user_limit", AuditLogChangeKey::UserLimit.name());
        assert_eq!("uses", AuditLogChangeKey::Uses.name());
        assert_eq!("vanity_url_code", AuditLogChangeKey::VanityUrlCode.name());
        assert_eq!(
            "verification_level",
            AuditLogChangeKey::VerificationLevel.name()
        );
        assert_eq!(
            "widget_channel_id",
            AuditLogChangeKey::WidgetChannelId.name()
        );
        assert_eq!("widget_enabled", AuditLogChangeKey::WidgetEnabled.name());
    }

    #[allow(clippy::too_many_lines)]
    #[test]
    fn serde() {
        serde_test::assert_tokens(
            &AuditLogChangeKey::AfkChannelId,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "afk_channel_id",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::AfkTimeout,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "afk_timeout",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::Allow,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "allow",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::ApplicationId,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "application_id",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::AvatarHash,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "avatar_hash",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::BannerHash,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "banner_hash",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::Bitrate,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "bitrate",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::ChannelId,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "channel_id",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::Code,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "code",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::Color,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "color",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::CommandId,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "command_id",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::CommunicationDisabledUntil,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "communication_disabled_until",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::Deaf,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "deaf",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::DefaultMessageNotifications,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "default_message_notifications",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::Deny,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "deny",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::Description,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "description",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::DiscoverySplashHash,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "discovery_splash_hash",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::EnableEmoticons,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "enable_emoticons",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::ExpireBehavior,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "expire_behavior",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::ExpireGracePeriod,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "expire_grace_period",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::ExplicitContentFilter,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "explicit_content_filter",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::Hoist,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "hoist",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::IconHash,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "icon_hash",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::Id,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "id",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::ImageHash,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "image_hash",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::Invitable,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "invitable",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::InviterId,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "inviter_id",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::MaxAge,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "max_age",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::MaxUses,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "max_uses",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::Mentionable,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "mentionable",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::MfaLevel,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "mfa_level",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::Mute,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "mute",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::Name,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "name",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::Nick,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "nick",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::NsfwLevel,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "nsfw_level",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::OwnerId,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "owner_id",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::Permissions,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "permissions",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::Position,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "position",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::PreferredLocale,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "preferred_locale",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::PrivacyLevel,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "privacy_level",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::PruneDeleteDays,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "prune_delete_days",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::PublicUpdatesChannelId,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "public_updates_channel_id",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::RateLimitPerUser,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "rate_limit_per_user",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::RoleAdded,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "$add",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::RoleRemoved,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "$remove",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::RulesChannelId,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "rules_channel_id",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::SplashHash,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "splash_hash",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::SystemChannelId,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "system_channel_id",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::Temporary,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "temporary",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::Topic,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "topic",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::Type,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "type",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::UserLimit,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "user_limit",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::Uses,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "uses",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::VanityUrlCode,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "vanity_url_code",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::VerificationLevel,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "verification_level",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::WidgetChannelId,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "widget_channel_id",
            }],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::WidgetEnabled,
            &[Token::UnitVariant {
                name: "AuditLogChangeKey",
                variant: "widget_enabled",
            }],
        );
    }
}
