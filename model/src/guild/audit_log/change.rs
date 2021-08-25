use super::change_key::AuditLogChangeKey;
use crate::{
    channel::stage_instance::PrivacyLevel,
    guild::{
        DefaultMessageNotificationLevel, ExplicitContentFilter, MfaLevel, NSFWLevel, Permissions,
        VerificationLevel,
    },
    id::{ApplicationId, ChannelId, GenericId, RoleId, UserId},
};
use serde::{Deserialize, Serialize};

/// Minimal amount of information about an affected [role].
///
/// The following [`AuditLogChange`]s include this information:
///
/// - [`AuditLogChange::RoleAdded`]
/// - [`AuditLogChange::RoleRemoved`]
///
/// [role]: super::super::Role
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AffectedRole {
    /// ID of the role.
    pub id: RoleId,
    /// Name of the role.
    pub name: String,
}

/// Individual change within an [`AuditLogEntry`].
///
/// [`AuditLogEntry`]: super::AuditLogEntry
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case", tag = "key")]
pub enum AuditLogChange {
    /// AFK channel ID was changed.
    AfkChannelId {
        /// New ID of the AFK channel.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<ChannelId>,
        /// Old ID of the AFK channel.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<ChannelId>,
    },
    /// Timeout to cause a user to be moved to an AFK voice channel.
    AfkTimeout {
        /// New timeout, in seconds.
        #[serde(rename = "new_value")]
        new: u64,
        /// Old timeout, in seconds.
        #[serde(rename = "old_value")]
        old: u64,
    },
    /// Allowed permissions of a permission overwrite target.
    Allow {
        /// New allowed permissions value.
        #[serde(rename = "new_value")]
        new: Permissions,
    },
    /// ID of an application.
    ApplicationId {
        /// Application's ID.
        #[serde(rename = "new_value")]
        new: ApplicationId,
    },
    /// Hash of an avatar.
    AvatarHash {
        /// New hash of an avatar.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<String>,
        /// Old hash of an avatar.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<String>,
    },
    /// Hash of a guild banner.
    BannerHash {
        /// New hash of a guild's banner.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<String>,
        /// Old hash of a guild's banner.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<String>,
    },
    /// Bitrate of an audio channel.
    Bitrate {
        /// New bitrate.
        #[serde(rename = "new_value")]
        new: u64,
        /// Old bitrate.
        #[serde(rename = "old_value")]
        old: Option<u64>,
    },
    /// Channel for an invite code.
    ChannelId {
        /// New invite's channel.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<ChannelId>,
    },
    /// Code of an invite.
    Code {
        /// New invite's code.
        #[serde(rename = "new_value")]
        new: String,
    },
    /// Color of a role.
    Color {
        /// New role color.
        #[serde(rename = "new_value")]
        new: u64,
        /// Old role color.
        #[serde(rename = "old_value")]
        old: u64,
    },
    /// Whether a member is guild deafened.
    Deaf {
        /// Whether a member is now guild deafened.
        #[serde(rename = "new_value")]
        new: bool,
        /// Whether a member was now guild deafened.
        #[serde(rename = "old_value")]
        old: bool,
    },
    /// Default message notification level for a guild.
    DefaultMessageNotifications {
        /// New default message notification level.
        #[serde(rename = "new_value")]
        new: DefaultMessageNotificationLevel,
        /// Old default message notification level.
        #[serde(rename = "old_value")]
        old: DefaultMessageNotificationLevel,
    },
    /// Denied permissions of a permission overwrite target.
    Deny {
        /// New denied permissions level.
        #[serde(rename = "new_value")]
        new: Permissions,
    },
    /// Description of a guild.
    Description {
        /// New guild description.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<String>,
        /// Old guild description.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<String>,
    },
    /// Hash of a guild's discovery splash.
    DiscoverySplashHash {
        /// New discovery splash hash.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<String>,
        /// Old discovery splash hash.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<String>,
    },
    /// Whether emoticons are enabled.
    EnableEmoticons {
        /// Whether emoticons are now enabled.
        #[serde(rename = "new_value")]
        new: bool,
        /// Whether emoticons were enabled.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<bool>,
    },
    /// Behavior of the expiration of an integration.
    ExpireBehavior {
        /// New expiration behavior.
        #[serde(rename = "new_value")]
        new: u64,
    },
    /// Grace period of the expiration of an integration.
    ExpireGracePeriod {
        /// New expiration grace period.
        #[serde(rename = "new_value")]
        new: u64,
    },
    /// Explicit content filter level of a guild.
    ExplicitContentFilter {
        /// New explicit content filter level.
        #[serde(rename = "new_value")]
        new: ExplicitContentFilter,
        /// Old explicit content filter level.
        #[serde(rename = "old_value")]
        old: ExplicitContentFilter,
    },
    /// Whether a role is hoisted.
    Hoist {
        /// Whether a role is now hoisted.
        #[serde(rename = "new_value")]
        new: bool,
        /// Whether a role was hoisted.
        #[serde(rename = "old_value")]
        old: bool,
    },
    /// Hash of a guild icon.
    IconHash {
        /// New hash of a guild's icon.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<String>,
        /// Old hash of a guild's icon.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<String>,
    },
    /// ID of an entity.
    Id {
        /// New entity's ID.
        #[serde(rename = "new_value")]
        new: GenericId,
    },
    /// ID of the user who created an invite.
    InviterId {
        /// User ID.
        #[serde(rename = "new_value")]
        new: UserId,
    },
    /// Maximum age of an invite.
    MaxAge {
        /// New maximum age.
        #[serde(rename = "new_value")]
        new: u64,
    },
    /// Maximum uses of an invite.
    MaxUses {
        /// New maximum uses.
        #[serde(rename = "new_value")]
        new: u64,
    },
    /// Whether a role can be mentioned in a message.
    Mentionable {
        /// Whether a role is now mentionable.
        #[serde(rename = "new_value")]
        new: bool,
        /// Whether a role was mentionable.
        #[serde(rename = "old_value")]
        old: bool,
    },
    /// Multi-Factor Authentication level required of a guild's moderators.
    MfaLevel {
        /// New MFA level of a guild.
        #[serde(rename = "new_value")]
        new: MfaLevel,
        /// Old MFA level of a guild.
        #[serde(rename = "old_value")]
        old: MfaLevel,
    },
    /// Whether a user is guild muted.
    Mute {
        /// Whether a member is now muted.
        #[serde(rename = "new_value")]
        new: bool,
        /// Whether a member was muted.
        #[serde(rename = "old_value")]
        old: bool,
    },
    /// Name of an entity such as a channel or role.
    Name {
        /// New entity name.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<String>,
        /// Old entity name.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<String>,
    },
    /// Nickname of a member.
    Nick {
        /// New member nickname.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<String>,
        /// Old member nickname.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<String>,
    },
    /// NSFW level of a guild.
    NsfwLevel {
        /// New NSFW level.
        #[serde(rename = "new_value")]
        new: NSFWLevel,
        /// Old NSFW level.
        #[serde(rename = "old_value")]
        old: NSFWLevel,
    },
    /// ID of the owner of a guild.
    OwnerId {
        /// New owner's ID.
        #[serde(rename = "new_value")]
        new: UserId,
        /// Old owner's ID.
        #[serde(rename = "old_value")]
        old: UserId,
    },
    /// Default permissions of a role.
    Permissions {
        /// New set of permissions.
        #[serde(rename = "new_value")]
        new: Permissions,
        /// Old set of permissions.
        #[serde(rename = "old_value")]
        old: Permissions,
    },
    /// Position of an entity such as a channel or role.
    Position {
        /// New position value.
        #[serde(rename = "new_value")]
        new: u64,
        /// Old position value.
        #[serde(rename = "old_value")]
        old: u64,
    },
    /// Preferred locale of a guild.
    PreferredLocale {
        /// New preferred locale.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<String>,
        /// Old preferred locale.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<String>,
    },
    /// Privacy level of a stage instance.
    PrivacyLevel {
        /// New privacy level.
        #[serde(rename = "new_value")]
        new: PrivacyLevel,
        /// Old privacy level.
        #[serde(rename = "old_value")]
        old: PrivacyLevel,
    },
    /// Number of days' worth of inactivity for a guild prune.
    PruneDeleteDays {
        /// Number of days.
        #[serde(rename = "new_value")]
        new: u64,
    },
    /// ID of a guild's public updates channel.
    PublicUpdatesChannelId {
        /// New public updates channel ID.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<ChannelId>,
        /// Old public updates channel ID.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<ChannelId>,
    },
    /// Ratelimit per user in a textual channel.
    RateLimitPerUser {
        /// New ratelimit, in seconds.
        #[serde(rename = "new_value")]
        new: u64,
        /// Old ratelimit, in seconds.
        #[serde(rename = "old_value")]
        old: u64,
    },
    /// Role was added to a user.
    RoleAdded {
        /// Minimal information about a added role.
        #[serde(rename = "new_value")]
        new: Vec<AffectedRole>,
    },
    /// Role was removed from a user.
    RoleRemoved {
        /// Minimal information about a removed role.
        #[serde(rename = "new_value")]
        new: Vec<AffectedRole>,
    },
    /// Guild's rules channel.
    RulesChannelId {
        /// New rules channel.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<ChannelId>,
        /// Old rules channel.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<ChannelId>,
    },
    /// Hash of a guild's splash.
    SplashHash {
        /// Old hash of a guild's splash.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<String>,
        /// New hash of a guild's splash.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<String>,
    },
    /// ID of guild's system channel.
    SystemChannelId {
        /// New system channel ID.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<ChannelId>,
        /// Old system channel ID.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<ChannelId>,
    },
    /// Whether an invite is temporary.
    Temporary {
        /// New temporary state.
        #[serde(rename = "new_value")]
        new: bool,
    },
    /// Topic of a textual channel.
    Topic {
        /// Old topic.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<String>,
        /// New topic.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<String>,
    },
    /// Type of a created entity.
    Type {
        /// New target type.
        #[serde(rename = "new_value")]
        new: Option<u64>,
        /// Old target type.
        #[serde(rename = "old_value")]
        old: Option<u64>,
    },
    /// Maximum number of users in a voice channel.
    UserLimit {
        /// New limit.
        #[serde(rename = "new_value")]
        new: u64,
        /// Old limit.
        #[serde(rename = "old_value")]
        old: Option<u64>,
    },
    /// Number of uses of an invite.
    Uses {
        /// Number of uses.
        #[serde(rename = "new_value")]
        new: u64,
    },
    /// Code of a guild's vanity invite.
    VanityUrlCode {
        /// New vanity URL code.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<String>,
        /// Old vanity URL code.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<String>,
    },
    /// Required verification level of new members in a guild.
    VerificationLevel {
        /// New verification level.
        #[serde(rename = "new_value")]
        new: VerificationLevel,
        /// Old verification level.
        #[serde(rename = "old_value")]
        old: VerificationLevel,
    },
    /// Channel ID of a widget.
    WidgetChannelId {
        /// New channel ID.
        #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
        new: Option<ChannelId>,
        /// Old channel ID.
        #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
        old: Option<ChannelId>,
    },
    /// Whether a widget is enabled.
    WidgetEnabled {
        /// New state of a widget being enabled.
        #[serde(rename = "new_value")]
        new: bool,
        /// Old state of a widget being enabled.
        #[serde(rename = "old_value")]
        old: bool,
    },
    /// Other type of change not covered by other variants.
    #[serde(other)]
    Other,
}

impl AuditLogChange {
    /// Key of an audit log change.
    ///
    /// This may return no key if the variant is [`Other`].
    ///
    /// # Examples
    ///
    /// Check the key of a [`Uses`] change:
    ///
    /// ```
    /// use twilight_model::guild::audit_log::{
    ///     AuditLogChangeKey,
    ///     AuditLogChange,
    /// };
    ///
    /// let change = AuditLogChange::UserLimit {
    ///     new: 6,
    ///     old: Some(3),
    /// };
    ///
    /// assert_eq!(Some(AuditLogChangeKey::UserLimit), change.key());
    /// ```
    ///
    /// [`Other`]: Self::Other
    /// [`Uses`]: Self::Uses
    pub const fn key(&self) -> Option<AuditLogChangeKey> {
        Some(match self {
            Self::AfkChannelId { .. } => AuditLogChangeKey::AfkChannelId,
            Self::AfkTimeout { .. } => AuditLogChangeKey::AfkTimeout,
            Self::Allow { .. } => AuditLogChangeKey::Allow,
            Self::ApplicationId { .. } => AuditLogChangeKey::ApplicationId,
            Self::AvatarHash { .. } => AuditLogChangeKey::AvatarHash,
            Self::BannerHash { .. } => AuditLogChangeKey::BannerHash,
            Self::Bitrate { .. } => AuditLogChangeKey::Bitrate,
            Self::ChannelId { .. } => AuditLogChangeKey::ChannelId,
            Self::Code { .. } => AuditLogChangeKey::Code,
            Self::Color { .. } => AuditLogChangeKey::Color,
            Self::Deaf { .. } => AuditLogChangeKey::Deaf,
            Self::DefaultMessageNotifications { .. } => {
                AuditLogChangeKey::DefaultMessageNotifications
            }
            Self::Deny { .. } => AuditLogChangeKey::Deny,
            Self::Description { .. } => AuditLogChangeKey::Description,
            Self::DiscoverySplashHash { .. } => AuditLogChangeKey::DiscoverySplashHash,
            Self::EnableEmoticons { .. } => AuditLogChangeKey::EnableEmoticons,
            Self::ExpireBehavior { .. } => AuditLogChangeKey::ExpireBehavior,
            Self::ExpireGracePeriod { .. } => AuditLogChangeKey::ExpireGracePeriod,
            Self::ExplicitContentFilter { .. } => AuditLogChangeKey::ExplicitContentFilter,
            Self::Hoist { .. } => AuditLogChangeKey::Hoist,
            Self::IconHash { .. } => AuditLogChangeKey::IconHash,
            Self::Id { .. } => AuditLogChangeKey::Id,
            Self::InviterId { .. } => AuditLogChangeKey::InviterId,
            Self::MaxAge { .. } => AuditLogChangeKey::MaxAge,
            Self::MaxUses { .. } => AuditLogChangeKey::MaxUses,
            Self::Mentionable { .. } => AuditLogChangeKey::Mentionable,
            Self::MfaLevel { .. } => AuditLogChangeKey::MfaLevel,
            Self::Mute { .. } => AuditLogChangeKey::Mute,
            Self::Name { .. } => AuditLogChangeKey::Name,
            Self::Nick { .. } => AuditLogChangeKey::Nick,
            Self::NsfwLevel { .. } => AuditLogChangeKey::NsfwLevel,
            Self::OwnerId { .. } => AuditLogChangeKey::OwnerId,
            Self::Permissions { .. } => AuditLogChangeKey::Permissions,
            Self::Position { .. } => AuditLogChangeKey::Position,
            Self::PreferredLocale { .. } => AuditLogChangeKey::PreferredLocale,
            Self::PrivacyLevel { .. } => AuditLogChangeKey::PrivacyLevel,
            Self::PruneDeleteDays { .. } => AuditLogChangeKey::PruneDeleteDays,
            Self::PublicUpdatesChannelId { .. } => AuditLogChangeKey::PublicUpdatesChannelId,
            Self::RateLimitPerUser { .. } => AuditLogChangeKey::RateLimitPerUser,
            Self::RoleAdded { .. } => AuditLogChangeKey::RoleAdded,
            Self::RoleRemoved { .. } => AuditLogChangeKey::RoleRemoved,
            Self::RulesChannelId { .. } => AuditLogChangeKey::RulesChannelId,
            Self::SplashHash { .. } => AuditLogChangeKey::SplashHash,
            Self::SystemChannelId { .. } => AuditLogChangeKey::SystemChannelId,
            Self::Temporary { .. } => AuditLogChangeKey::Temporary,
            Self::Topic { .. } => AuditLogChangeKey::Topic,
            Self::Type { .. } => AuditLogChangeKey::Type,
            Self::Uses { .. } => AuditLogChangeKey::Uses,
            Self::UserLimit { .. } => AuditLogChangeKey::UserLimit,
            Self::VanityUrlCode { .. } => AuditLogChangeKey::VanityUrlCode,
            Self::VerificationLevel { .. } => AuditLogChangeKey::VerificationLevel,
            Self::WidgetChannelId { .. } => AuditLogChangeKey::WidgetChannelId,
            Self::WidgetEnabled { .. } => AuditLogChangeKey::WidgetEnabled,
            Self::Other => return None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{super::AuditLogChangeKey, AffectedRole, AuditLogChange};
    use crate::{guild::Permissions, id::ChannelId};
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{fmt::Debug, hash::Hash};

    assert_fields!(AffectedRole: id, name);
    assert_fields!(AuditLogChange::AfkChannelId: new, old);
    assert_fields!(AuditLogChange::AfkTimeout: new, old);
    assert_fields!(AuditLogChange::Allow: new);
    assert_fields!(AuditLogChange::ApplicationId: new);
    assert_fields!(AuditLogChange::AvatarHash: new, old);
    assert_fields!(AuditLogChange::BannerHash: new, old);
    assert_fields!(AuditLogChange::Bitrate: new, old);
    assert_fields!(AuditLogChange::ChannelId: new);
    assert_fields!(AuditLogChange::Code: new);
    assert_fields!(AuditLogChange::Color: new, old);
    assert_fields!(AuditLogChange::Deaf: new, old);
    assert_fields!(AuditLogChange::DefaultMessageNotifications: new, old);
    assert_fields!(AuditLogChange::Deny: new);
    assert_fields!(AuditLogChange::Description: new, old);
    assert_fields!(AuditLogChange::DiscoverySplashHash: new, old);
    assert_fields!(AuditLogChange::EnableEmoticons: new, old);
    assert_fields!(AuditLogChange::ExpireBehavior: new);
    assert_fields!(AuditLogChange::ExpireGracePeriod: new);
    assert_fields!(AuditLogChange::ExplicitContentFilter: new, old);
    assert_fields!(AuditLogChange::Hoist: new, old);
    assert_fields!(AuditLogChange::IconHash: new, old);
    assert_fields!(AuditLogChange::Id: new);
    assert_fields!(AuditLogChange::InviterId: new);
    assert_fields!(AuditLogChange::MaxAge: new);
    assert_fields!(AuditLogChange::MaxUses: new);
    assert_fields!(AuditLogChange::Mentionable: new, old);
    assert_fields!(AuditLogChange::MfaLevel: new, old);
    assert_fields!(AuditLogChange::Mute: new, old);
    assert_fields!(AuditLogChange::Name: new, old);
    assert_fields!(AuditLogChange::Nick: new, old);
    assert_fields!(AuditLogChange::NsfwLevel: new, old);
    assert_fields!(AuditLogChange::OwnerId: new, old);
    assert_fields!(AuditLogChange::Permissions: new, old);
    assert_fields!(AuditLogChange::PrivacyLevel: new, old);
    assert_fields!(AuditLogChange::Position: new, old);
    assert_fields!(AuditLogChange::PreferredLocale: new, old);
    assert_fields!(AuditLogChange::PruneDeleteDays: new);
    assert_fields!(AuditLogChange::PublicUpdatesChannelId: new, old);
    assert_fields!(AuditLogChange::RateLimitPerUser: new, old);
    assert_fields!(AuditLogChange::RoleAdded: new);
    assert_fields!(AuditLogChange::RoleRemoved: new);
    assert_fields!(AuditLogChange::RulesChannelId: new, old);
    assert_fields!(AuditLogChange::SplashHash: new, old);
    assert_fields!(AuditLogChange::SystemChannelId: new, old);
    assert_fields!(AuditLogChange::Temporary: new);
    assert_fields!(AuditLogChange::Topic: new);
    assert_fields!(AuditLogChange::Type: new);
    assert_fields!(AuditLogChange::Uses: new);
    assert_fields!(AuditLogChange::UserLimit: new, old);
    assert_fields!(AuditLogChange::VanityUrlCode: new, old);
    assert_fields!(AuditLogChange::VerificationLevel: new, old);
    assert_fields!(AuditLogChange::WidgetChannelId: new, old);
    assert_fields!(AuditLogChange::WidgetEnabled: new, old);
    assert_impl_all!(
        AffectedRole: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync
    );
    assert_impl_all!(
        AuditLogChange: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync
    );

    #[test]
    fn test_afk_channel_id() {
        let value = AuditLogChange::AfkChannelId {
            new: Some(ChannelId::new(1).expect("non zero")),
            old: None,
        };

        assert_eq!(Some(AuditLogChangeKey::AfkChannelId), value.key());

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "AuditLogChange",
                    len: 2,
                },
                Token::String("key"),
                Token::String("afk_channel_id"),
                Token::String("new_value"),
                Token::Some,
                Token::NewtypeStruct { name: "ChannelId" },
                Token::String("1"),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_permissions() {
        let old: Permissions = Permissions::SEND_MESSAGES;
        let new: Permissions = old | Permissions::EMBED_LINKS;

        let value = AuditLogChange::Permissions { new, old };

        assert_eq!(Some(AuditLogChangeKey::Permissions), value.key());

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "AuditLogChange",
                    len: 3,
                },
                Token::String("key"),
                Token::String("permissions"),
                Token::String("new_value"),
                Token::Str("18432"),
                Token::String("old_value"),
                Token::Str("2048"),
                Token::StructEnd,
            ],
        );
    }
}
