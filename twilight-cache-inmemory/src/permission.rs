//! Calculate the permissions for members in on a guild- or channel-level with
//! information from the cache.
//!
//! # Required Configuration
//!
//! Calculating permissions required that some information relevant to the
//! member, their roles, and the channel or guild is available in the cache.
//! These will only be stored in the cache when certain [`ResourceType`]s are
//! enabled. To enable the configurations for both the
//! [`InMemoryCachePermissions::in_channel`] and
//! [`InMemoryCachePermissions::root`] operations you must enable
//! their required [`ResourceType`]s like so:
//!
//! ```
//! use twilight_cache_inmemory::{DefaultInMemoryCache, ResourceType};
//!
//! let resource_types = ResourceType::CHANNEL | ResourceType::MEMBER | ResourceType::ROLE;
//!
//! let cache = DefaultInMemoryCache::builder()
//!     .resource_types(resource_types)
//!     .build();
//! ```
//!
//! # Disabled Member Communication Caveats
//!
//! The permission calculator checks the [current system time] against when a
//! given member had their [communication disabled until]. If a member's
//! communication is disabled, then they are restricted to
//! [read-only permissions]. If the system time is incorrect then this may
//! result in invalid behavior. This behavior can be opted out of via
//! [`InMemoryCachePermissions::check_member_communication_disabled`].
//!
//! [`ResourceType`]: crate::ResourceType
//! [communication timed out until]: CachedMember::communication_disabled_until
//! [current system time]: SystemTime::now
//! [read-only permissions]: MEMBER_COMMUNICATION_DISABLED_ALLOWLIST

use super::InMemoryCache;
use crate::{
    traits::{CacheableChannel, CacheableGuild, CacheableMember, CacheableRole},
    CacheableModels,
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    time::{Duration, SystemTime},
};
use twilight_model::{
    channel::{permission_overwrite::PermissionOverwrite, ChannelType},
    guild::Permissions,
    id::{
        marker::{ChannelMarker, GuildMarker, RoleMarker, UserMarker},
        Id,
    },
};
use twilight_util::permission_calculator::PermissionCalculator;

/// Permissions a member is allowed to have when their
/// [communication has been disabled].
///
/// Refer to the [module level] documentation for more information on how
/// disabled member communication is calculated.
///
/// [communication has been disabled]: crate::model::CachedMember::communication_disabled_until
/// [module level]: crate::permission
pub const MEMBER_COMMUNICATION_DISABLED_ALLOWLIST: Permissions = Permissions::from_bits_truncate(
    Permissions::READ_MESSAGE_HISTORY.bits() | Permissions::VIEW_CHANNEL.bits(),
);

/// Error calculating permissions with the information in a cache.
#[derive(Debug)]
pub struct ChannelError {
    kind: ChannelErrorType,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl ChannelError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &ChannelErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        self.source
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(self) -> (ChannelErrorType, Option<Box<dyn Error + Send + Sync>>) {
        (self.kind, self.source)
    }

    /// Create a root error from an error while retrieving a member's roles.
    // clippy: the contents of `member_roles_error` is consumed
    #[allow(clippy::needless_pass_by_value)]
    fn from_member_roles(member_roles_error: MemberRolesErrorType) -> Self {
        Self {
            kind: match member_roles_error {
                MemberRolesErrorType::RoleMissing { role_id } => {
                    ChannelErrorType::RoleUnavailable { role_id }
                }
            },
            source: None,
        }
    }
}

impl Display for ChannelError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.kind {
            ChannelErrorType::ChannelNotInGuild { channel_id } => {
                f.write_str("channel ")?;
                Display::fmt(&channel_id, f)?;

                f.write_str(" is not in a guild")
            }
            ChannelErrorType::ChannelUnavailable { channel_id } => {
                f.write_str("channel ")?;
                Display::fmt(&channel_id, f)?;

                f.write_str(" is either not in the cache or is not a guild channel")
            }
            ChannelErrorType::MemberUnavailable { guild_id, user_id } => {
                f.write_str("member (guild: ")?;
                Display::fmt(&guild_id, f)?;
                f.write_str("; user: ")?;
                Display::fmt(&user_id, f)?;

                f.write_str(") is not present in the cache")
            }
            ChannelErrorType::ParentChannelNotPresent { thread_id } => {
                f.write_str("thread ")?;
                Display::fmt(&thread_id, f)?;

                f.write_str(" has no parent")
            }
            ChannelErrorType::RoleUnavailable { role_id } => {
                f.write_str("member has role ")?;
                Display::fmt(&role_id, f)?;

                f.write_str(" but it is not present in the cache")
            }
        }
    }
}

impl Error for ChannelError {}

/// Type of [`ChannelError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum ChannelErrorType {
    /// Channel is not in a guild.
    ///
    /// This may be because the channel is a private channel.
    ChannelNotInGuild {
        /// ID of the channel.
        channel_id: Id<ChannelMarker>,
    },
    /// Guild channel is not present in the cache.
    ChannelUnavailable {
        /// ID of the channel.
        channel_id: Id<ChannelMarker>,
    },
    /// The user's member information is not available in the guild.
    ///
    /// This could be because the user is not currently a member of the guild or
    /// because the member entity has not yet been received by the cache.
    MemberUnavailable {
        /// ID of the guild.
        guild_id: Id<GuildMarker>,
        /// ID of the user.
        user_id: Id<UserMarker>,
    },
    /// A thread's parent ID is not present.
    ParentChannelNotPresent {
        /// ID of the thread.
        thread_id: Id<ChannelMarker>,
    },
    /// One of the user's roles is not available in the guild.
    ///
    /// The reasons this could happen could be due to the cache missing a
    /// [`RoleCreate`] event or a user application race condition.
    ///
    /// [`RoleCreate`]: twilight_model::gateway::payload::incoming::RoleCreate
    RoleUnavailable {
        /// ID of the role that the user has but details about is missing.
        role_id: Id<RoleMarker>,
    },
}

/// Error calculating permissions with information in a cache.
#[derive(Debug)]
pub struct RootError {
    kind: RootErrorType,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl RootError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &RootErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        self.source
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(self) -> (RootErrorType, Option<Box<dyn Error + Send + Sync>>) {
        (self.kind, self.source)
    }

    /// Create a root error from an error while retrieving a member's roles.
    // clippy: the contents of `member_roles_error` is consumed
    #[allow(clippy::needless_pass_by_value)]
    fn from_member_roles(member_roles_error: MemberRolesErrorType) -> Self {
        Self {
            kind: match member_roles_error {
                MemberRolesErrorType::RoleMissing { role_id } => {
                    RootErrorType::RoleUnavailable { role_id }
                }
            },
            source: None,
        }
    }
}

impl Display for RootError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.kind {
            RootErrorType::MemberUnavailable { guild_id, user_id } => {
                f.write_str("member (guild: ")?;
                Display::fmt(&guild_id, f)?;
                f.write_str("; user: ")?;
                Display::fmt(&user_id, f)?;

                f.write_str(") is not present in the cache")
            }
            RootErrorType::RoleUnavailable { role_id } => {
                f.write_str("member has role ")?;
                Display::fmt(&role_id, f)?;

                f.write_str(" but it is not present in the cache")
            }
        }
    }
}

impl Error for RootError {}

/// Type of [`RootError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum RootErrorType {
    /// The user's member information is not available in the guild.
    ///
    /// This could be because the user is not currently a member of the guild or
    /// because the member entity has not yet been received by the cache.
    MemberUnavailable {
        /// ID of the guild.
        guild_id: Id<GuildMarker>,
        /// ID of the user.
        user_id: Id<UserMarker>,
    },
    /// One of the user's roles is not available in the guild.
    ///
    /// The reasons this could happen could be due to the cache missing a
    /// [`RoleCreate`] event or a user application race condition.
    ///
    /// [`RoleCreate`]: twilight_model::gateway::payload::incoming::RoleCreate
    RoleUnavailable {
        /// ID of the role that the user has but details about is missing.
        role_id: Id<RoleMarker>,
    },
}

/// Error type that occurred while getting a member's assigned roles'
/// permissions as well as the `@everyone` role's permissions.
enum MemberRolesErrorType {
    /// Role is missing from the cache.
    RoleMissing { role_id: Id<RoleMarker> },
}

/// Member's roles' permissions and the guild's `@everyone` role's permissions.
struct MemberRoles {
    /// User's roles and their permissions.
    assigned: Vec<(Id<RoleMarker>, Permissions)>,
    /// Permissions of the guild's `@everyone` role.
    everyone: Permissions,
}

/// Calculate the permissions of a member with information from the cache.
#[allow(clippy::type_complexity)]
#[derive(Clone, Debug)]
#[must_use = "has no effect if unused"]
pub struct InMemoryCachePermissions<'a, CacheModels: CacheableModels> {
    cache: &'a InMemoryCache<CacheModels>,
    check_member_communication_disabled: bool,
}

impl<'a, CacheModels: CacheableModels> InMemoryCachePermissions<'a, CacheModels> {
    #[allow(clippy::type_complexity)]
    pub(super) const fn new(cache: &'a InMemoryCache<CacheModels>) -> Self {
        Self {
            cache,
            check_member_communication_disabled: true,
        }
    }

    /// Immutable reference to the underlying cache.
    #[allow(clippy::type_complexity)]
    pub const fn cache_ref(&'a self) -> &'a InMemoryCache<CacheModels> {
        self.cache
    }

    /// Consume the statistics interface, returning the underlying cache
    /// reference.
    #[allow(clippy::type_complexity)]
    pub const fn into_cache(self) -> &'a InMemoryCache<CacheModels> {
        self.cache
    }

    /// Whether to check whether a [member's communication is disabled][field].
    ///
    /// Refer to the [module level] documentation for information and caveats.
    ///
    /// Defaults to being enabled.
    ///
    /// [field]: crate::model::CachedMember::communication_disabled_until
    /// [module level]: crate::permission
    pub const fn check_member_communication_disabled(
        mut self,
        check_member_communication_disabled: bool,
    ) -> Self {
        self.check_member_communication_disabled = check_member_communication_disabled;

        self
    }

    /// Calculate the permissions of a member in a guild channel.
    ///
    /// Returns [`Permissions::all`] if the user is the owner of the guild.
    ///
    /// If the member's [communication has been disabled] then they will be
    /// restricted to [read-only permissions]. Refer to the [module level]
    /// documentation for more information.
    ///
    /// The following [`ResourceType`]s must be enabled:
    ///
    /// - [`ResourceType::CHANNEL`]
    /// - [`ResourceType::MEMBER`]
    /// - [`ResourceType::ROLE`]
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use twilight_cache_inmemory::DefaultInMemoryCache;
    /// use twilight_model::id::Id;
    ///
    /// let cache = DefaultInMemoryCache::new();
    ///
    /// // later on...
    ///
    /// let channel_id = Id::new(4);
    /// let user_id = Id::new(5);
    ///
    /// let permissions = cache.permissions().in_channel(user_id, channel_id)?;
    /// println!("User {user_id} in channel {channel_id} has permissions {permissions:?}");
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a [`ChannelErrorType::ChannelUnavailable`] error type if the
    /// guild channel is not in the cache.
    ///
    /// Returns a [`ChannelErrorType::MemberUnavailable`] error type if the
    /// member for the user in the guild is not present.
    ///
    /// Returns a [`ChannelErrorType::RoleUnavailable`] error type if one of the
    /// member's roles is not in the cache.
    ///
    /// [`Permissions::all`]: twilight_model::guild::Permissions::all
    /// [`ResourceType::CHANNEL`]: crate::ResourceType::CHANNEL
    /// [`ResourceType::MEMBER`]: crate::ResourceType::MEMBER
    /// [`ResourceType::ROLE`]: crate::ResourceType::ROLE
    /// [`ResourceType`]: crate::ResourceType
    /// [communication has been disabled]: crate::model::CachedMember::communication_disabled_until
    /// [module level]: crate::permission
    /// [read-only permissions]: MEMBER_COMMUNICATION_DISABLED_ALLOWLIST
    pub fn in_channel(
        &self,
        user_id: Id<UserMarker>,
        channel_id: Id<ChannelMarker>,
    ) -> Result<Permissions, ChannelError> {
        let channel = self.cache.channels.get(&channel_id).ok_or(ChannelError {
            kind: ChannelErrorType::ChannelUnavailable { channel_id },
            source: None,
        })?;

        let guild_id = channel.guild_id().ok_or(ChannelError {
            kind: ChannelErrorType::ChannelNotInGuild { channel_id },
            source: None,
        })?;

        if self.is_owner(user_id, guild_id) {
            return Ok(Permissions::all());
        }

        let member = self.cache.member(guild_id, user_id).ok_or(ChannelError {
            kind: ChannelErrorType::MemberUnavailable { guild_id, user_id },
            source: None,
        })?;

        let MemberRoles { assigned, everyone } = self
            .member_roles(guild_id, &member)
            .map_err(ChannelError::from_member_roles)?;

        let overwrites = match channel.kind() {
            ChannelType::AnnouncementThread
            | ChannelType::PrivateThread
            | ChannelType::PublicThread => self.parent_overwrites(&channel)?,
            _ => channel.permission_overwrites().unwrap_or_default().to_vec(),
        };

        let calculator =
            PermissionCalculator::new(guild_id, user_id, everyone, assigned.as_slice());

        let permissions = calculator.in_channel(channel.kind(), overwrites.as_slice());

        Ok(self.disable_member_communication(&member, permissions))
    }

    /// Calculate the guild-level permissions of a member.
    ///
    /// Returns [`Permissions::all`] if the user is the owner of the guild.
    ///
    /// If the member's [communication has been disabled] then they will be
    /// restricted to [read-only permissions]. Refer to the [module level]
    /// documentation for more information.
    ///
    /// The following [`ResourceType`]s must be enabled:
    ///
    /// - [`ResourceType::MEMBER`]
    /// - [`ResourceType::ROLE`]
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use twilight_cache_inmemory::DefaultInMemoryCache;
    /// use twilight_model::id::Id;
    ///
    /// let cache = DefaultInMemoryCache::new();
    ///
    /// // later on...
    ///
    /// let guild_id = Id::new(4);
    /// let user_id = Id::new(5);
    ///
    /// let permissions = cache.permissions().root(user_id, guild_id)?;
    /// println!("User {user_id} in guild {guild_id} has permissions {permissions:?}");
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a [`RootErrorType::MemberUnavailable`] error type if the
    /// member for the user in the guild is not present.
    ///
    /// Returns a [`RootErrorType::RoleUnavailable`] error type if one of the
    /// member's roles is not in the cache.
    ///
    /// [`Permissions::all`]: twilight_model::guild::Permissions::all
    /// [`ResourceType::MEMBER`]: crate::ResourceType::MEMBER
    /// [`ResourceType::ROLE`]: crate::ResourceType::ROLE
    /// [`ResourceType`]: crate::ResourceType
    /// [communication has been disabled]: crate::model::CachedMember::communication_disabled_until
    /// [module level]: crate::permission
    /// [read-only permissions]: MEMBER_COMMUNICATION_DISABLED_ALLOWLIST
    pub fn root(
        &self,
        user_id: Id<UserMarker>,
        guild_id: Id<GuildMarker>,
    ) -> Result<Permissions, RootError> {
        if self.is_owner(user_id, guild_id) {
            return Ok(Permissions::all());
        }

        let member = self.cache.member(guild_id, user_id).ok_or(RootError {
            kind: RootErrorType::MemberUnavailable { guild_id, user_id },
            source: None,
        })?;

        let MemberRoles { assigned, everyone } = self
            .member_roles(guild_id, &member)
            .map_err(RootError::from_member_roles)?;
        let calculator =
            PermissionCalculator::new(guild_id, user_id, everyone, assigned.as_slice());

        let permissions = calculator.root();

        Ok(self.disable_member_communication(&member, permissions))
    }

    /// Determine whether the provided member is disabled and restrict them to
    /// [read-only permissions] if they are.
    ///
    /// Only members whose [`communication_disabled_until`] values is in the
    /// future count as being currently disabled. Members with the
    /// [administrator permission] are never disabled.
    ///
    /// [`communication_disabled_until`]: CachedMember::communication_disabled_until
    /// [administrator permission]: Permissions::ADMINISTRATOR
    /// [read-only permissions]: MEMBER_COMMUNICATION_DISABLED_ALLOWLIST
    fn disable_member_communication(
        &self,
        member: &CacheModels::Member,
        permissions: Permissions,
    ) -> Permissions {
        // Administrators are never disabled.
        if !self.check_member_communication_disabled
            || permissions.contains(Permissions::ADMINISTRATOR)
        {
            return permissions;
        }

        let micros = if let Some(until) = member.communication_disabled_until() {
            until.as_micros()
        } else {
            return permissions;
        };

        let Ok(absolute) = micros.try_into() else {
            return permissions;
        };

        let ends = SystemTime::UNIX_EPOCH + Duration::from_micros(absolute);
        let now = SystemTime::now();

        if now > ends {
            return permissions;
        }

        permissions.intersection(MEMBER_COMMUNICATION_DISABLED_ALLOWLIST)
    }

    /// Determine whether a given user is the owner of a guild.
    ///
    /// Returns true if the user is or false if the user is definitively not the
    /// owner of the guild or the guild is not in the cache.
    fn is_owner(&self, user_id: Id<UserMarker>, guild_id: Id<GuildMarker>) -> bool {
        self.cache
            .guilds
            .get(&guild_id)
            .map(|r| r.owner_id() == user_id)
            .unwrap_or_default()
    }

    /// Retrieve a member's roles' permissions and the guild's `@everyone`
    /// role's permissions.
    ///
    /// # Errors
    ///
    /// Returns [`MemberRolesErrorType::RoleMissing`] if a role is missing from
    /// the cache.
    fn member_roles(
        &self,
        guild_id: Id<GuildMarker>,
        member: &'a CacheModels::Member,
    ) -> Result<MemberRoles, MemberRolesErrorType> {
        let mut member_roles = Vec::with_capacity(member.roles().len());

        for role_id in member.roles() {
            let Some(role) = self.cache.roles.get(role_id) else {
                return Err(MemberRolesErrorType::RoleMissing { role_id: *role_id });
            };

            member_roles.push((*role_id, role.permissions()));
        }

        let everyone_role_id = guild_id.cast();

        if let Some(everyone_role) = self.cache.roles.get(&everyone_role_id) {
            Ok(MemberRoles {
                assigned: member_roles,
                everyone: everyone_role.permissions(),
            })
        } else {
            Err(MemberRolesErrorType::RoleMissing {
                role_id: everyone_role_id,
            })
        }
    }

    /// Given a thread channel, retrieve its parent from the cache, and combine
    /// parent and child permissions.
    fn parent_overwrites(
        &self,
        thread: &CacheModels::Channel,
    ) -> Result<Vec<PermissionOverwrite>, ChannelError> {
        let parent_id = thread.parent_id().ok_or(ChannelError {
            kind: ChannelErrorType::ParentChannelNotPresent {
                thread_id: thread.id(),
            },
            source: None,
        })?;

        let channel = self.cache.channels.get(&parent_id).ok_or(ChannelError {
            kind: ChannelErrorType::ChannelUnavailable {
                channel_id: parent_id,
            },
            source: None,
        })?;

        if channel.guild_id().is_some() {
            let channel_overwrites = channel.permission_overwrites().unwrap_or_default();
            let thread_overwrites = thread.permission_overwrites().unwrap_or_default();

            let mut overwrites =
                Vec::with_capacity(channel_overwrites.len() + thread_overwrites.len());

            overwrites.extend_from_slice(channel_overwrites);
            overwrites.extend_from_slice(thread_overwrites);

            Ok(overwrites)
        } else {
            Err(ChannelError {
                kind: ChannelErrorType::ChannelNotInGuild {
                    channel_id: channel.id(),
                },
                source: None,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ChannelError, ChannelErrorType, InMemoryCachePermissions, RootError, RootErrorType,
    };
    use crate::{test, DefaultCacheModels, DefaultInMemoryCache};
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{
        error::Error,
        fmt::Debug,
        str::FromStr,
        time::{Duration, SystemTime},
    };
    use twilight_model::{
        channel::{
            permission_overwrite::{PermissionOverwrite, PermissionOverwriteType},
            Channel, ChannelType,
        },
        gateway::payload::incoming::{
            ChannelCreate, GuildCreate, MemberAdd, MemberUpdate, RoleCreate, ThreadCreate,
        },
        guild::{
            AfkTimeout, DefaultMessageNotificationLevel, ExplicitContentFilter, Guild, MfaLevel,
            NSFWLevel, Permissions, PremiumTier, Role, SystemChannelFlags, VerificationLevel,
        },
        id::{
            marker::{ChannelMarker, GuildMarker, RoleMarker, UserMarker},
            Id,
        },
        util::Timestamp,
    };

    assert_fields!(ChannelErrorType::ChannelUnavailable: channel_id);
    assert_fields!(ChannelErrorType::MemberUnavailable: guild_id, user_id);
    assert_fields!(ChannelErrorType::RoleUnavailable: role_id);
    assert_impl_all!(ChannelErrorType: Debug, Send, Sync);
    assert_impl_all!(ChannelError: Debug, Send, Sync);
    assert_impl_all!(InMemoryCachePermissions<'_, DefaultCacheModels>: Clone, Debug, Send, Sync);
    assert_fields!(RootErrorType::MemberUnavailable: guild_id, user_id);
    assert_fields!(RootErrorType::RoleUnavailable: role_id);
    assert_impl_all!(RootErrorType: Debug, Send, Sync);
    assert_impl_all!(RootError: Debug, Send, Sync);

    /// Guild ID used in tests.
    const GUILD_ID: Id<GuildMarker> = Id::new(1);

    /// ID of the `@everyone` role.
    const EVERYONE_ROLE_ID: Id<RoleMarker> = GUILD_ID.cast();

    /// User ID used in tests.
    const USER_ID: Id<UserMarker> = Id::new(2);

    /// ID of another role.
    const OTHER_ROLE_ID: Id<RoleMarker> = Id::new(3);

    /// ID of the user that owns the guild with the ID [`GUILD_ID`].
    const OWNER_ID: Id<UserMarker> = Id::new(4);

    /// ID of the #general channel in the guild.
    ///
    /// This has the same ID as the [`GUILD_ID`].
    const CHANNEL_ID: Id<ChannelMarker> = GUILD_ID.cast();

    /// ID of a thread created in the general channel.
    const THREAD_ID: Id<ChannelMarker> = Id::new(5);

    /// ID of the safety alerts channel.
    const SAFETY_ALERTS_CHANNEL_ID: Id<ChannelMarker> = Id::new(6);

    fn base_guild() -> Guild {
        Guild {
            id: GUILD_ID,
            afk_channel_id: None,
            afk_timeout: AfkTimeout::FIVE_MINUTES,
            application_id: None,
            banner: None,
            channels: Vec::new(),
            default_message_notifications: DefaultMessageNotificationLevel::Mentions,
            description: None,
            discovery_splash: None,
            emojis: Vec::new(),
            explicit_content_filter: ExplicitContentFilter::AllMembers,
            features: Vec::new(),
            icon: None,
            joined_at: None,
            large: false,
            max_members: None,
            max_presences: None,
            member_count: None,
            members: Vec::new(),
            mfa_level: MfaLevel::Elevated,
            name: "this is a guild".to_owned(),
            nsfw_level: NSFWLevel::AgeRestricted,
            owner: Some(false),
            owner_id: OWNER_ID,
            permissions: None,
            preferred_locale: "en-GB".to_owned(),
            premium_progress_bar_enabled: false,
            premium_subscription_count: Some(0),
            premium_tier: PremiumTier::None,
            presences: Vec::new(),
            public_updates_channel_id: None,
            roles: Vec::from([
                // Give the `@everyone` role a guild level and channel level
                // permission.
                role_with_permissions(
                    EVERYONE_ROLE_ID,
                    Permissions::CREATE_INVITE | Permissions::VIEW_AUDIT_LOG,
                ),
            ]),
            safety_alerts_channel_id: Some(SAFETY_ALERTS_CHANNEL_ID),
            splash: None,
            stage_instances: Vec::new(),
            stickers: Vec::new(),
            system_channel_id: None,
            system_channel_flags: SystemChannelFlags::SUPPRESS_JOIN_NOTIFICATIONS,
            threads: Vec::new(),
            rules_channel_id: None,
            unavailable: false,
            verification_level: VerificationLevel::VeryHigh,
            voice_states: Vec::new(),
            vanity_url_code: None,
            widget_channel_id: None,
            widget_enabled: None,
            max_video_channel_users: None,
            approximate_member_count: None,
            approximate_presence_count: None,
        }
    }

    fn channel() -> Channel {
        Channel {
            application_id: None,
            applied_tags: None,
            available_tags: None,
            bitrate: None,
            default_auto_archive_duration: None,
            default_forum_layout: None,
            default_reaction_emoji: None,
            default_sort_order: None,
            default_thread_rate_limit_per_user: None,
            flags: None,
            guild_id: Some(GUILD_ID),
            icon: None,
            id: CHANNEL_ID,
            invitable: None,
            kind: ChannelType::GuildText,
            last_message_id: None,
            last_pin_timestamp: None,
            managed: None,
            member: None,
            member_count: None,
            message_count: None,
            name: Some("test".to_owned()),
            newly_created: None,
            nsfw: Some(false),
            owner_id: None,
            parent_id: None,
            permission_overwrites: Some(Vec::from([
                PermissionOverwrite {
                    allow: Permissions::empty(),
                    deny: Permissions::CREATE_INVITE,
                    id: EVERYONE_ROLE_ID.cast(),
                    kind: PermissionOverwriteType::Role,
                },
                PermissionOverwrite {
                    allow: Permissions::EMBED_LINKS,
                    deny: Permissions::empty(),
                    id: USER_ID.cast(),
                    kind: PermissionOverwriteType::Member,
                },
            ])),
            position: Some(0),
            rate_limit_per_user: None,
            recipients: None,
            rtc_region: None,
            thread_metadata: None,
            topic: None,
            user_limit: None,
            video_quality_mode: None,
        }
    }

    fn thread() -> Channel {
        Channel {
            application_id: None,
            applied_tags: None,
            available_tags: None,
            bitrate: None,
            default_auto_archive_duration: None,
            default_forum_layout: None,
            default_reaction_emoji: None,
            default_sort_order: None,
            default_thread_rate_limit_per_user: None,
            flags: None,
            guild_id: Some(GUILD_ID),
            icon: None,
            id: THREAD_ID,
            invitable: None,
            kind: ChannelType::PublicThread,
            last_message_id: None,
            last_pin_timestamp: None,
            managed: None,
            member: None,
            member_count: None,
            message_count: None,
            name: Some("test thread".to_owned()),
            newly_created: None,
            nsfw: Some(false),
            owner_id: None,
            parent_id: Some(CHANNEL_ID),
            permission_overwrites: Some(Vec::from([PermissionOverwrite {
                allow: Permissions::ATTACH_FILES,
                deny: Permissions::empty(),
                id: EVERYONE_ROLE_ID.cast(),
                kind: PermissionOverwriteType::Role,
            }])),
            position: Some(0),
            rate_limit_per_user: None,
            recipients: None,
            rtc_region: None,
            thread_metadata: None,
            topic: None,
            user_limit: None,
            video_quality_mode: None,
        }
    }

    fn role_with_permissions(id: Id<RoleMarker>, permissions: Permissions) -> Role {
        let mut role = test::role(id);
        role.permissions = permissions;

        role
    }

    const fn role_create(guild_id: Id<GuildMarker>, role: Role) -> RoleCreate {
        RoleCreate { guild_id, role }
    }

    /// Test that the permissions interface returns the correct errors depending
    /// on what information is unavailable during [`root`] operations.
    ///
    /// [`root`]: super::InMemoryCachePermissions::root
    #[test]
    fn root_errors() {
        let cache = DefaultInMemoryCache::new();
        let permissions = cache.permissions();
        assert!(matches!(
            permissions.root(USER_ID, GUILD_ID).unwrap_err().kind(),
            &RootErrorType::MemberUnavailable { guild_id: g_id, user_id: u_id }
            if g_id == GUILD_ID && u_id == USER_ID
        ));

        cache.update(&MemberAdd {
            guild_id: GUILD_ID,
            member: test::member(USER_ID),
        });

        assert!(matches!(
            permissions.root(USER_ID, GUILD_ID).unwrap_err().kind(),
            &RootErrorType::RoleUnavailable { role_id }
            if role_id == EVERYONE_ROLE_ID
        ));
    }

    /// Test that the permissions interface returns the correct permissions for
    /// a member on a root level.
    ///
    /// Notably [`root`] doesn't require that the guild *itself* is in the
    /// cache.
    ///
    /// [`root`]: super::InMemoryCachePermissions::root
    #[test]
    fn root() -> Result<(), Box<dyn Error>> {
        let joined_at = Some(Timestamp::from_str("2021-09-19T14:17:32.000000+00:00")?);

        let cache = DefaultInMemoryCache::new();
        let permissions = cache.permissions();

        cache.update(&GuildCreate::Available(base_guild()));
        cache.update(&MemberAdd {
            guild_id: GUILD_ID,
            member: test::member(USER_ID),
        });
        cache.update(&MemberUpdate {
            avatar: None,
            communication_disabled_until: None,
            guild_id: GUILD_ID,
            deaf: None,
            joined_at,
            mute: None,
            nick: None,
            pending: false,
            premium_since: None,
            roles: Vec::from([OTHER_ROLE_ID]),
            user: test::user(USER_ID),
        });
        cache.update(&role_create(
            GUILD_ID,
            role_with_permissions(
                OTHER_ROLE_ID,
                Permissions::SEND_MESSAGES | Permissions::BAN_MEMBERS,
            ),
        ));

        let expected = Permissions::CREATE_INVITE
            | Permissions::BAN_MEMBERS
            | Permissions::VIEW_AUDIT_LOG
            | Permissions::SEND_MESSAGES;

        assert_eq!(expected, permissions.root(USER_ID, GUILD_ID)?);

        Ok(())
    }

    /// Test that the permissions interface returns the correct errors and
    /// permissions depending on what information is unavailable during
    /// [`in_channel`] operations.
    ///
    /// [`in_channel`]: super::InMemoryCachePermissions::in_channel
    #[test]
    fn in_channel() -> Result<(), Box<dyn Error>> {
        let cache = DefaultInMemoryCache::new();
        let permissions = cache.permissions();

        cache.update(&GuildCreate::Available(base_guild()));
        assert!(matches!(
            permissions.in_channel(USER_ID, CHANNEL_ID).unwrap_err().kind(),
            ChannelErrorType::ChannelUnavailable { channel_id: c_id }
            if *c_id == CHANNEL_ID
        ));

        cache.update(&ChannelCreate(channel()));
        assert!(matches!(
            permissions.in_channel(USER_ID, CHANNEL_ID).unwrap_err().kind(),
            ChannelErrorType::MemberUnavailable { guild_id: g_id, user_id: u_id }
            if *g_id == GUILD_ID && *u_id == USER_ID
        ));
        let mut member = test::member(USER_ID);
        member.roles.push(OTHER_ROLE_ID);

        cache.update(&MemberAdd {
            guild_id: GUILD_ID,
            member,
        });
        assert!(matches!(
            permissions.in_channel(USER_ID, CHANNEL_ID).unwrap_err().kind(),
            &ChannelErrorType::RoleUnavailable { role_id }
            if role_id == OTHER_ROLE_ID
        ));

        cache.update(&role_create(
            GUILD_ID,
            role_with_permissions(
                OTHER_ROLE_ID,
                Permissions::SEND_MESSAGES | Permissions::BAN_MEMBERS,
            ),
        ));

        assert_eq!(
            Permissions::EMBED_LINKS | Permissions::SEND_MESSAGES,
            permissions.in_channel(USER_ID, CHANNEL_ID)?,
        );

        cache.update(&ThreadCreate(thread()));

        assert_eq!(
            Permissions::EMBED_LINKS | Permissions::SEND_MESSAGES | Permissions::ATTACH_FILES,
            permissions.in_channel(USER_ID, THREAD_ID)?
        );

        Ok(())
    }

    /// Test that [`in_channel`] and [`root`] both return [`Permissions::all`]
    /// if the user is also the owner of the guild.
    ///
    /// Only the guild needs to be in the cache to short-circuit on this
    /// condition.
    ///
    /// [`in_channel`]: super::InMemoryCachePermissions::in_channel
    /// [`root`]: super::InMemoryCachePermissions::root
    #[test]
    fn owner() -> Result<(), Box<dyn Error>> {
        let cache = DefaultInMemoryCache::new();
        let permissions = cache.permissions();
        cache.update(&GuildCreate::Available(base_guild()));

        assert!(permissions.root(OWNER_ID, GUILD_ID)?.is_all());

        cache.update(&ChannelCreate(channel()));
        assert!(permissions.in_channel(OWNER_ID, CHANNEL_ID)?.is_all());

        Ok(())
    }

    /// Test the behavior of a member having their communication disabled.
    ///
    /// In particular, we want to test that:
    ///
    /// - if a member is timed out they will be limited to the intersection of
    ///   the [`Permissions::READ_MESSAGE_HISTORY`] and
    ///   [`Permissions::VIEW_CHANNEL`] permissions on a [guild level][`root`]
    /// - the same is true on a [channel level][`in_channel`]
    /// - administrators are never timed out
    /// - checking whether the member's communication is disabled is configurable
    ///
    /// [`in_channel`]: super::InMemoryCachePermissions::in_channel
    /// [`root`]: super::InMemoryCachePermissions::root
    #[test]
    fn member_communication_disabled() -> Result<(), Box<dyn Error>> {
        fn acceptable_time(in_future: bool) -> Result<Timestamp, Box<dyn Error>> {
            const TIME_RANGE: Duration = Duration::from_secs(60);

            let now = SystemTime::now();

            let system_time = if in_future {
                now + TIME_RANGE
            } else {
                now - TIME_RANGE
            };

            let since = system_time.duration_since(SystemTime::UNIX_EPOCH)?;
            let micros = since.as_micros().try_into()?;

            Timestamp::from_micros(micros).map_err(From::from)
        }

        let cache = DefaultInMemoryCache::new();
        let mut permissions = cache.permissions();

        let in_past = acceptable_time(false)?;
        let in_future = acceptable_time(true)?;

        let mut guild = base_guild();
        let everyone_permissions = Permissions::CREATE_INVITE
            | Permissions::READ_MESSAGE_HISTORY
            | Permissions::VIEW_AUDIT_LOG
            | Permissions::VIEW_CHANNEL;
        guild.roles = Vec::from([role_with_permissions(
            EVERYONE_ROLE_ID,
            everyone_permissions,
        )]);

        cache.update(&GuildCreate::Available(guild));
        let mut member = test::member(USER_ID);
        member.communication_disabled_until = Some(in_future);
        cache.update(&MemberAdd {
            guild_id: GUILD_ID,
            member,
        });
        assert_eq!(
            Permissions::VIEW_CHANNEL | Permissions::READ_MESSAGE_HISTORY,
            permissions.root(USER_ID, GUILD_ID)?
        );

        cache.update(&ChannelCreate(channel()));
        assert_eq!(
            Permissions::VIEW_CHANNEL | Permissions::READ_MESSAGE_HISTORY,
            permissions.in_channel(USER_ID, CHANNEL_ID)?
        );

        // check that comparison can be disabled
        permissions = permissions.check_member_communication_disabled(false);
        assert_eq!(everyone_permissions, permissions.root(USER_ID, GUILD_ID)?);
        permissions = permissions.check_member_communication_disabled(true);

        // check administrators are never disabled
        cache.update(&role_create(
            GUILD_ID,
            role_with_permissions(OTHER_ROLE_ID, Permissions::ADMINISTRATOR),
        ));
        cache.update(&MemberUpdate {
            avatar: None,
            communication_disabled_until: Some(in_past),
            guild_id: GUILD_ID,
            deaf: None,
            joined_at: Some(Timestamp::from_secs(1).unwrap()),
            mute: None,
            nick: None,
            pending: false,
            premium_since: None,
            roles: Vec::from([OTHER_ROLE_ID]),
            user: test::user(USER_ID),
        });
        assert_eq!(Permissions::all(), permissions.root(USER_ID, GUILD_ID)?);

        Ok(())
    }
}
