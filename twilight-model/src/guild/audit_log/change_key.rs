use crate::util::known_string::KnownString;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Formatter, Result as FmtResult},
    ops::Deref,
    str::FromStr,
};

/// Type of [`AuditLogChange`].
///
/// For additional information refer to [Discord Docs/Audit Log Change Key][1].
///
/// [`AuditLogChange`]: super::AuditLogChange
/// [1]: https://discord.com/developers/docs/resources/audit-log#audit-log-change-object-audit-log-change-key
#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AuditLogChangeKey(KnownString<64>);

impl AuditLogChangeKey {
    /// AFK voice channel for a guild.
    pub const AFK_CHANNEL_ID: Self = Self::from_bytes(b"afk_channel_id");

    /// Timeout to cause a user to be moved to an AFK voice channel.
    pub const AFK_TIMEOUT: Self = Self::from_bytes(b"afk_timeout");

    /// Allowed permissions of a permission overwrite target.
    pub const ALLOW: Self = Self::from_bytes(b"allow");

    /// ID of an application.
    pub const APPLICATION_ID: Self = Self::from_bytes(b"application_id");

    /// Thread was archived or unarchived.
    pub const ARCHIVED: Self = Self::from_bytes(b"archived");

    /// Asset of a sticker.
    ///
    /// Empty string.
    pub const ASSET: Self = Self::from_bytes(b"asset");

    /// Auto archive duration of a thread.
    pub const AUTO_ARCHIVE_DURATION: Self = Self::from_bytes(b"auto_archive_duration");

    /// Availability of a sticker.
    pub const AVAILABLE: Self = Self::from_bytes(b"available");

    /// Hash of an avatar.
    pub const AVATAR_HASH: Self = Self::from_bytes(b"avatar_hash");

    /// Hash of a guild banner.
    pub const BANNER_HASH: Self = Self::from_bytes(b"banner_hash");

    /// Bitrate of an audio channel.
    pub const BITRATE: Self = Self::from_bytes(b"bitrate");

    /// Channel for an invite code.
    pub const CHANNEL_ID: Self = Self::from_bytes(b"channel_id");

    /// Code of an invite.
    pub const CODE: Self = Self::from_bytes(b"code");

    /// Color of a role.
    pub const COLOR: Self = Self::from_bytes(b"color");

    /// Permissions for a command were updated.
    pub const COMMAND_ID: Self = Self::from_bytes(b"command_id");

    /// Member timeout state changed.
    pub const COMMUNICATION_DISABLED_UNTIL: Self =
        Self::from_bytes(b"communication_disabled_until");

    /// Whether a user is guild deafened.
    pub const DEAF: Self = Self::from_bytes(b"deaf");

    /// Default auto archive duration for new threads.
    pub const DEFAULT_AUTO_ARCHIVE_DURATION: Self =
        Self::from_bytes(b"default_auto_archive_duration");

    /// Default message notification level for a guild.
    pub const DEFAULT_MESSAGE_NOTIFICATIONS: Self =
        Self::from_bytes(b"default_message_notifications");

    /// Denied permissions of a permission overwrite target.
    pub const DENY: Self = Self::from_bytes(b"deny");

    /// Description of a guild.
    pub const DESCRIPTION: Self = Self::from_bytes(b"description");

    /// Hash of a guild's discovery splash.
    pub const DISCOVERY_SPLASH_HASH: Self = Self::from_bytes(b"discovery_splash_hash");

    /// Whether emoticons are enabled.
    pub const ENABLE_EMOTICONS: Self = Self::from_bytes(b"enable_emoticons");

    /// Entity type of guild scheduled event was changed.
    pub const ENTITY_TYPE: Self = Self::from_bytes(b"entity_type");

    /// Behavior of the expiration of an integration.
    pub const EXPIRE_BEHAVIOR: Self = Self::from_bytes(b"expire_behavior");

    /// Grace period of the expiration of an integration.
    pub const EXPIRE_GRACE_PERIOD: Self = Self::from_bytes(b"expire_grace_period");

    /// Explicit content filter level of a guild.
    pub const EXPLICIT_CONTENT_FILTER: Self = Self::from_bytes(b"explicit_content_filter");

    /// Format type of a sticker.
    pub const FORMAT_TYPE: Self = Self::from_bytes(b"format_type");

    /// Guild that a sticker is in.
    pub const GUILD_ID: Self = Self::from_bytes(b"guild_id");

    /// Whether a role is hoisted.
    pub const HOIST: Self = Self::from_bytes(b"hoist");

    /// Hash of a guild icon.
    pub const ICON_HASH: Self = Self::from_bytes(b"icon_hash");

    /// ID of an entity.
    pub const ID: Self = Self::from_bytes(b"id");

    /// Hash of a guild scheduled event cover.
    pub const IMAGE_HASH: Self = Self::from_bytes(b"image_hash");

    /// Invitable state of a private thread.
    pub const INVITABLE: Self = Self::from_bytes(b"invitable");

    /// ID of the user who created an invite.
    pub const INVITER_ID: Self = Self::from_bytes(b"inviter_id");

    /// Channel ID for a scheduled event changed.
    pub const LOCATION: Self = Self::from_bytes(b"location");

    /// Thread was locked or unlocked.
    pub const LOCKED: Self = Self::from_bytes(b"locked");

    /// Maximum age of an invite.
    pub const MAX_AGE: Self = Self::from_bytes(b"max_age");

    /// Maximum uses of an invite.
    pub const MAX_USES: Self = Self::from_bytes(b"max_uses");

    /// Whether a role can be mentioned in a message.
    pub const MENTIONABLE: Self = Self::from_bytes(b"mentionable");

    /// Multi-Factor Authentication level required of a guild's moderators.
    pub const MFA_LEVEL: Self = Self::from_bytes(b"mfa_level");

    /// Whether a user is guild muted.
    pub const MUTE: Self = Self::from_bytes(b"mute");

    /// Name of an entity such as a channel or role.
    pub const NAME: Self = Self::from_bytes(b"name");

    /// Nickname of a member.
    pub const NICK: Self = Self::from_bytes(b"nick");

    /// Whether a channel is NSFW.
    pub const NSFW: Self = Self::from_bytes(b"nsfw");

    /// NSFW level of a guild.
    pub const NSFW_LEVEL: Self = Self::from_bytes(b"nsfw_level");

    /// ID of the owner of a guild.
    pub const OWNER_ID: Self = Self::from_bytes(b"owner_id");

    /// Permission overwrites on a channel changed.
    pub const PERMISSION_OVERWRITES: Self = Self::from_bytes(b"permission_overwrites");

    /// Default permissions of a role.
    pub const PERMISSIONS: Self = Self::from_bytes(b"permissions");

    /// Position of an entity such as a channel or role.
    pub const POSITION: Self = Self::from_bytes(b"position");

    /// Preferred locale of a guild.
    pub const PREFERRED_LOCALE: Self = Self::from_bytes(b"preferred_locale");

    /// Privacy level of a stage instance.
    pub const PRIVACY_LEVEL: Self = Self::from_bytes(b"privacy_level");

    /// Number of days' worth of inactivity for a guild prune.
    pub const PRUNE_DELETE_DAYS: Self = Self::from_bytes(b"prune_delete_days");

    /// ID of a guild's public updates channel.
    pub const PUBLIC_UPDATES_CHANNEL_ID: Self = Self::from_bytes(b"public_updates_channel_id");

    /// Ratelimit per user in a textual channel.
    pub const RATE_LIMIT_PER_USER: Self = Self::from_bytes(b"rate_limit_per_user");

    /// Region of a guild changed.
    pub const REGION: Self = Self::from_bytes(b"region");

    /// Role added to a user.
    pub const ROLE_ADDED: Self = Self::from_bytes(b"$add");

    /// Role removed from a user.
    pub const ROLE_REMOVED: Self = Self::from_bytes(b"$remove");

    /// ID of a guild's rules channel.
    pub const RULES_CHANNEL_ID: Self = Self::from_bytes(b"rules_channel_id");

    /// Hash of a guild's splash.
    pub const SPLASH_HASH: Self = Self::from_bytes(b"splash_hash");

    /// Status of guild scheduled event was changed.
    pub const STATUS: Self = Self::from_bytes(b"status");

    /// ID of a guild's system channel.
    pub const SYSTEM_CHANNEL_ID: Self = Self::from_bytes(b"system_channel_id");

    /// Related emoji of a sticker.
    pub const TAGS: Self = Self::from_bytes(b"tags");

    /// Whether an invite is temporary.
    pub const TEMPORARY: Self = Self::from_bytes(b"temporary");

    /// Topic of a textual channel.
    pub const TOPIC: Self = Self::from_bytes(b"topic");

    /// Type of a created entity.
    pub const TYPE: Self = Self::from_bytes(b"type");

    /// Role unicode emoji.
    pub const UNICODE_EMOJI: Self = Self::from_bytes(b"unicode_emoji");

    /// Maximum number of users in a voice channel.
    pub const USER_LIMIT: Self = Self::from_bytes(b"user_limit");

    /// Number of uses of an invite.
    pub const USES: Self = Self::from_bytes(b"uses");

    /// Code of a guild's vanity invite.
    pub const VANITY_URL_CODE: Self = Self::from_bytes(b"vanity_url_code");

    /// Required verification level of new members in a guild.
    pub const VERIFICATION_LEVEL: Self = Self::from_bytes(b"verification_level");

    /// Channel ID of a widget.
    pub const WIDGET_CHANNEL_ID: Self = Self::from_bytes(b"widget_channel_id");

    /// Whether a widget is enabled.
    pub const WIDGET_ENABLED: Self = Self::from_bytes(b"widget_enabled");

    /// Create a scope from a dynamic value.
    ///
    /// The provided scope must be 64 bytes or smaller.
    pub fn new(scope: &str) -> Option<Self> {
        KnownString::from_str(scope).map(Self)
    }

    /// Get the value of the scope.
    ///
    /// # Panics
    ///
    /// Panics if the scope isn't valid UTF-8.
    pub fn get(&self) -> &str {
        self.0.get()
    }

    /// Name of the associated constant.
    ///
    /// Returns `None` if the value doesn't have a defined constant.
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::AFK_CHANNEL_ID => "AFK_CHANNEL_ID",
            Self::AFK_TIMEOUT => "AFK_TIMEOUT",
            Self::ALLOW => "ALLOW",
            Self::APPLICATION_ID => "APPLICATION_ID",
            Self::ARCHIVED => "ARCHIVED",
            Self::ASSET => "ASSET",
            Self::AUTO_ARCHIVE_DURATION => "AUTO_ARCHIVE_DURATION",
            Self::AVAILABLE => "AVAILABLE",
            Self::AVATAR_HASH => "AVATAR_HASH",
            Self::BANNER_HASH => "BANNER_HASH",
            Self::BITRATE => "BITRATE",
            Self::CHANNEL_ID => "CHANNEL_ID",
            Self::CODE => "CODE",
            Self::COLOR => "COLOR",
            Self::COMMAND_ID => "COMMAND_ID",
            Self::COMMUNICATION_DISABLED_UNTIL => "COMMUNICATION_DISABLED_UNTIL",
            Self::DEAF => "DEAF",
            Self::DEFAULT_AUTO_ARCHIVE_DURATION => "DEFAULT_AUTO_ARCHIVE_DURATION",
            Self::DEFAULT_MESSAGE_NOTIFICATIONS => "DEFAULT_MESSAGE_NOTIFICATIONS",
            Self::DENY => "DENY",
            Self::DESCRIPTION => "DESCRIPTION",
            Self::DISCOVERY_SPLASH_HASH => "DISCOVERY_SPLASH_HASH",
            Self::ENABLE_EMOTICONS => "ENABLE_EMOTICONS",
            Self::ENTITY_TYPE => "ENTITY_TYPE",
            Self::EXPIRE_BEHAVIOR => "EXPIRE_BEHAVIOR",
            Self::EXPIRE_GRACE_PERIOD => "EXPIRE_GRACE_PERIOD",
            Self::EXPLICIT_CONTENT_FILTER => "EXPLICIT_CONTENT_FILTER",
            Self::FORMAT_TYPE => "FORMAT_TYPE",
            Self::GUILD_ID => "GUILD_ID",
            Self::HOIST => "HOIST",
            Self::ICON_HASH => "ICON_HASH",
            Self::ID => "ID",
            Self::IMAGE_HASH => "IMAGE_HASH",
            Self::INVITABLE => "INVITABLE",
            Self::INVITER_ID => "INVITER_ID",
            Self::LOCATION => "LOCATION",
            Self::LOCKED => "LOCKED",
            Self::MAX_AGE => "MAX_AGE",
            Self::MAX_USES => "MAX_USES",
            Self::MENTIONABLE => "MENTIONABLE",
            Self::MFA_LEVEL => "MFA_LEVEL",
            Self::MUTE => "MUTE",
            Self::NAME => "NAME",
            Self::NICK => "NICK",
            Self::NSFW => "NSFW",
            Self::NSFW_LEVEL => "NSFW_LEVEL",
            Self::OWNER_ID => "OWNER_ID",
            Self::PERMISSION_OVERWRITES => "PERMISSION_OVERWRITES",
            Self::PERMISSIONS => "PERMISSIONS",
            Self::POSITION => "POSITION",
            Self::PREFERRED_LOCALE => "PREFERRED_LOCALE",
            Self::PRIVACY_LEVEL => "PRIVACY_LEVEL",
            Self::PRUNE_DELETE_DAYS => "PRUNE_DELETE_DAYS",
            Self::PUBLIC_UPDATES_CHANNEL_ID => "PUBLIC_UPDATES_CHANNEL_ID",
            Self::RATE_LIMIT_PER_USER => "RATE_LIMIT_PER_USER",
            Self::REGION => "REGION",
            Self::ROLE_ADDED => "ROLE_ADDED",
            Self::ROLE_REMOVED => "ROLE_REMOVED",
            Self::RULES_CHANNEL_ID => "RULES_CHANNEL_ID",
            Self::SPLASH_HASH => "SPLASH_HASH",
            Self::STATUS => "STATUS",
            Self::SYSTEM_CHANNEL_ID => "SYSTEM_CHANNEL_ID",
            Self::TAGS => "TAGS",
            Self::TEMPORARY => "TEMPORARY",
            Self::TOPIC => "TOPIC",
            Self::TYPE => "TYPE",
            Self::UNICODE_EMOJI => "UNICODE_EMOJI",
            Self::USER_LIMIT => "USER_LIMIT",
            Self::USES => "USES",
            Self::VANITY_URL_CODE => "VANITY_URL_CODE",
            Self::VERIFICATION_LEVEL => "VERIFICATION_LEVEL",
            Self::WIDGET_CHANNEL_ID => "WIDGET_CHANNEL_ID",
            Self::WIDGET_ENABLED => "WIDGET_ENABLED",
            _ => return None,
        })
    }

    /// Create a scope from a set of bytes.
    const fn from_bytes(input: &[u8]) -> Self {
        Self(KnownString::from_bytes(input))
    }
}

impl AsRef<str> for AuditLogChangeKey {
    fn as_ref(&self) -> &str {
        self.get()
    }
}

impl Debug for AuditLogChangeKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.name().unwrap_or_else(|| self.get()))
    }
}

impl Deref for AuditLogChangeKey {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl FromStr for AuditLogChangeKey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl ToString for AuditLogChangeKey {
    fn to_string(&self) -> String {
        KnownString::to_string(&self.0)
    }
}

impl TryFrom<&str> for AuditLogChangeKey {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(())
    }
}

#[cfg(test)]
mod tests {
    use super::AuditLogChangeKey;
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, hash::Hash, str::FromStr};

    assert_impl_all!(
        AuditLogChangeKey: AsRef<str>,
        Clone,
        Copy,
        Debug,
        Deserialize<'static>,
        Eq,
        FromStr,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync,
        ToString,
        TryFrom<&'static str>,
    );

    #[test]
    fn name() {
        assert_eq!("afk_channel_id", AuditLogChangeKey::AFK_CHANNEL_ID.get());
        assert_eq!("afk_timeout", AuditLogChangeKey::AFK_TIMEOUT.get());
        assert_eq!("allow", AuditLogChangeKey::ALLOW.get());
        assert_eq!("application_id", AuditLogChangeKey::APPLICATION_ID.get());
        assert_eq!("avatar_hash", AuditLogChangeKey::AVATAR_HASH.get());
        assert_eq!("banner_hash", AuditLogChangeKey::BANNER_HASH.get());
        assert_eq!("bitrate", AuditLogChangeKey::BITRATE.get());
        assert_eq!("channel_id", AuditLogChangeKey::CHANNEL_ID.get());
        assert_eq!("code", AuditLogChangeKey::CODE.get());
        assert_eq!("color", AuditLogChangeKey::COLOR.get());
        assert_eq!("command_id", AuditLogChangeKey::COMMAND_ID.get());
        assert_eq!(
            "communication_disabled_until",
            AuditLogChangeKey::COMMUNICATION_DISABLED_UNTIL.get()
        );
        assert_eq!("deaf", AuditLogChangeKey::DEAF.get());
        assert_eq!(
            "default_message_notifications",
            AuditLogChangeKey::DEFAULT_MESSAGE_NOTIFICATIONS.get()
        );
        assert_eq!("deny", AuditLogChangeKey::DENY.get());
        assert_eq!("description", AuditLogChangeKey::DESCRIPTION.get());
        assert_eq!(
            "discovery_splash_hash",
            AuditLogChangeKey::DISCOVERY_SPLASH_HASH.get()
        );
        assert_eq!(
            "enable_emoticons",
            AuditLogChangeKey::ENABLE_EMOTICONS.get()
        );
        assert_eq!("expire_behavior", AuditLogChangeKey::EXPIRE_BEHAVIOR.get());
        assert_eq!(
            "expire_grace_period",
            AuditLogChangeKey::EXPIRE_GRACE_PERIOD.get()
        );
        assert_eq!(
            "explicit_content_filter",
            AuditLogChangeKey::EXPLICIT_CONTENT_FILTER.get()
        );
        assert_eq!("hoist", AuditLogChangeKey::HOIST.get());
        assert_eq!("icon_hash", AuditLogChangeKey::ICON_HASH.get());
        assert_eq!("id", AuditLogChangeKey::ID.get());
        assert_eq!("image_hash", AuditLogChangeKey::IMAGE_HASH.get());
        assert_eq!("invitable", AuditLogChangeKey::INVITABLE.get());
        assert_eq!("inviter_id", AuditLogChangeKey::INVITER_ID.get());
        assert_eq!("max_age", AuditLogChangeKey::MAX_AGE.get());
        assert_eq!("max_uses", AuditLogChangeKey::MAX_USES.get());
        assert_eq!("mentionable", AuditLogChangeKey::MENTIONABLE.get());
        assert_eq!("mfa_level", AuditLogChangeKey::MFA_LEVEL.get());
        assert_eq!("mute", AuditLogChangeKey::MUTE.get());
        assert_eq!("name", AuditLogChangeKey::NAME.get());
        assert_eq!("nick", AuditLogChangeKey::NICK.get());
        assert_eq!("nsfw_level", AuditLogChangeKey::NSFW_LEVEL.get());
        assert_eq!("owner_id", AuditLogChangeKey::OWNER_ID.get());
        assert_eq!("permissions", AuditLogChangeKey::PERMISSIONS.get());
        assert_eq!("position", AuditLogChangeKey::POSITION.get());
        assert_eq!(
            "preferred_locale",
            AuditLogChangeKey::PREFERRED_LOCALE.get()
        );
        assert_eq!("privacy_level", AuditLogChangeKey::PRIVACY_LEVEL.get());
        assert_eq!(
            "prune_delete_days",
            AuditLogChangeKey::PRUNE_DELETE_DAYS.get()
        );
        assert_eq!(
            "public_updates_channel_id",
            AuditLogChangeKey::PUBLIC_UPDATES_CHANNEL_ID.get()
        );
        assert_eq!(
            "rate_limit_per_user",
            AuditLogChangeKey::RATE_LIMIT_PER_USER.get()
        );
        assert_eq!("$add", AuditLogChangeKey::ROLE_ADDED.get());
        assert_eq!("$remove", AuditLogChangeKey::ROLE_REMOVED.get());
        assert_eq!(
            "rules_channel_id",
            AuditLogChangeKey::RULES_CHANNEL_ID.get()
        );
        assert_eq!("splash_hash", AuditLogChangeKey::SPLASH_HASH.get());
        assert_eq!(
            "system_channel_id",
            AuditLogChangeKey::SYSTEM_CHANNEL_ID.get()
        );
        assert_eq!("temporary", AuditLogChangeKey::TEMPORARY.get());
        assert_eq!("topic", AuditLogChangeKey::TOPIC.get());
        assert_eq!("type", AuditLogChangeKey::TYPE.get());
        assert_eq!("user_limit", AuditLogChangeKey::USER_LIMIT.get());
        assert_eq!("uses", AuditLogChangeKey::USES.get());
        assert_eq!("vanity_url_code", AuditLogChangeKey::VANITY_URL_CODE.get());
        assert_eq!(
            "verification_level",
            AuditLogChangeKey::VERIFICATION_LEVEL.get()
        );
        assert_eq!(
            "widget_channel_id",
            AuditLogChangeKey::WIDGET_CHANNEL_ID.get()
        );
        assert_eq!("widget_enabled", AuditLogChangeKey::WIDGET_ENABLED.get());
    }

    #[allow(clippy::too_many_lines)]
    #[test]
    fn serde() {
        serde_test::assert_tokens(
            &AuditLogChangeKey::AFK_CHANNEL_ID,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("afk_channel_id"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::AFK_TIMEOUT,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("afk_timeout"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::ALLOW,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("allow"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::APPLICATION_ID,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("application_id"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::AVATAR_HASH,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("avatar_hash"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::BANNER_HASH,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("banner_hash"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::BITRATE,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("bitrate"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::CHANNEL_ID,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("channel_id"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::CODE,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("code"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::COLOR,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("color"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::COMMAND_ID,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("command_id"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::COMMUNICATION_DISABLED_UNTIL,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("communication_disabled_until"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::DEAF,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("deaf"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::DEFAULT_MESSAGE_NOTIFICATIONS,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("default_message_notifications"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::DENY,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("deny"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::DESCRIPTION,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("description"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::DISCOVERY_SPLASH_HASH,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("discovery_splash_hash"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::ENABLE_EMOTICONS,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("enable_emoticons"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::EXPIRE_BEHAVIOR,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("expire_behavior"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::EXPIRE_GRACE_PERIOD,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("expire_grace_period"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::EXPLICIT_CONTENT_FILTER,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("explicit_content_filter"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::HOIST,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("hoist"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::ICON_HASH,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("icon_hash"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::ID,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("id"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::IMAGE_HASH,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("image_hash"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::INVITABLE,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("invitable"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::INVITER_ID,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("inviter_id"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::MAX_AGE,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("max_age"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::MAX_USES,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("max_uses"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::MENTIONABLE,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("mentionable"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::MFA_LEVEL,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("mfa_level"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::MUTE,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("mute"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::NAME,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("name"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::NICK,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("nick"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::NSFW_LEVEL,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("nsfw_level"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::OWNER_ID,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("owner_id"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::PERMISSIONS,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("permissions"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::POSITION,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("position"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::PREFERRED_LOCALE,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("preferred_locale"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::PRIVACY_LEVEL,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("privacy_level"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::PRUNE_DELETE_DAYS,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("prune_delete_days"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::PUBLIC_UPDATES_CHANNEL_ID,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("public_updates_channel_id"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::RATE_LIMIT_PER_USER,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("rate_limit_per_user"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::ROLE_ADDED,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("$add"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::ROLE_REMOVED,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("$remove"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::RULES_CHANNEL_ID,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("rules_channel_id"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::SPLASH_HASH,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("splash_hash"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::SYSTEM_CHANNEL_ID,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("system_channel_id"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::TEMPORARY,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("temporary"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::TOPIC,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("topic"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::TYPE,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("type"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::USER_LIMIT,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("user_limit"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::USES,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("uses"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::VANITY_URL_CODE,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("vanity_url_code"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::VERIFICATION_LEVEL,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("verification_level"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::WIDGET_CHANNEL_ID,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("widget_channel_id"),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogChangeKey::WIDGET_ENABLED,
            &[
                Token::NewtypeStruct {
                    name: "AuditLogChangeKey",
                },
                Token::Str("widget_enabled"),
            ],
        );
    }
}
