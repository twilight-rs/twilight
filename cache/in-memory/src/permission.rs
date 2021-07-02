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
//! use twilight_cache_inmemory::{InMemoryCache, ResourceType};
//!
//! let resource_types = ResourceType::CHANNEL
//!     | ResourceType::MEMBER
//!     | ResourceType::ROLE;
//!
//! let cache = InMemoryCache::builder().resource_types(resource_types).build();
//! ```
//!
//! [`ResourceType`]: crate::ResourceType

use super::InMemoryCache;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::{
    channel::GuildChannel,
    guild::Permissions,
    id::{ChannelId, GuildId, RoleId, UserId},
};
use twilight_util::permission_calculator::PermissionCalculator;

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
    fn from_member_roles(member_roles_error: MemberRolesErrorType) -> Self {
        Self {
            kind: match member_roles_error {
                MemberRolesErrorType::MemberMissing { guild_id, user_id } => {
                    ChannelErrorType::MemberUnavailable { guild_id, user_id }
                }
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
            ChannelErrorType::ChannelUnavailable { channel_id } => f.write_fmt(format_args!(
                "channel {} is either not in the cache or is not a guild channel",
                channel_id
            )),
            ChannelErrorType::MemberUnavailable { guild_id, user_id } => f.write_fmt(format_args!(
                "member (guild: {}; user: {}) is not present in the cache",
                guild_id, user_id
            )),
            ChannelErrorType::RoleUnavailable { role_id } => f.write_fmt(format_args!(
                "member has role {} but it is not present in the cache",
                role_id
            )),
        }
    }
}

impl Error for ChannelError {}

/// Type of [`ChannelError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum ChannelErrorType {
    /// Guild channel is not present in the cache.
    ChannelUnavailable {
        /// ID of the channel.
        channel_id: ChannelId,
    },
    /// User's member information is not in the cache.
    MemberUnavailable {
        /// ID of the guild.
        guild_id: GuildId,
        /// ID of the user.
        user_id: UserId,
    },
    /// One of the member's roles is not in the cache.
    RoleUnavailable {
        /// ID of the role.
        role_id: RoleId,
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
    fn from_member_roles(member_roles_error: MemberRolesErrorType) -> Self {
        Self {
            kind: match member_roles_error {
                MemberRolesErrorType::MemberMissing { guild_id, user_id } => {
                    RootErrorType::MemberUnavailable { guild_id, user_id }
                }
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
            RootErrorType::MemberUnavailable { guild_id, user_id } => f.write_fmt(format_args!(
                "member (guild: {}; user: {}) is not present in the cache",
                guild_id, user_id
            )),
            RootErrorType::RoleUnavailable { role_id } => f.write_fmt(format_args!(
                "member has role {} but it is not present in the cache",
                role_id
            )),
        }
    }
}

impl Error for RootError {}

/// Type of [`RootError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum RootErrorType {
    MemberUnavailable { guild_id: GuildId, user_id: UserId },
    RoleUnavailable { role_id: RoleId },
}

/// Error type that occurred while getting a member's assigned roles'
/// permissions as well as the `@everyone` role's permissions.
enum MemberRolesErrorType {
    /// Member is not in the cache.
    MemberMissing {
        /// ID of the guild.
        guild_id: GuildId,
        /// ID of the user.
        user_id: UserId,
    },
    /// Role is missing from the cache.
    RoleMissing { role_id: RoleId },
}

/// Member's roles' permissions and the guild's `@everyone` role's permissions.
struct MemberRoles {
    /// User's roles and their permissions.
    assigned: Vec<(RoleId, Permissions)>,
    /// Permissions of the guild's `@everyone` role.
    everyone: Permissions,
}

/// Calculate the permissions of a member with information from the cache.
#[derive(Clone, Debug)]
pub struct InMemoryCachePermissions<'a>(&'a InMemoryCache);

impl<'a> InMemoryCachePermissions<'a> {
    pub(super) const fn new(cache: &'a InMemoryCache) -> Self {
        Self(cache)
    }

    /// Immutable reference to the underlying cache.
    pub const fn cache_ref(&'a self) -> &'a InMemoryCache {
        self.0
    }

    /// Consume the statistics interface, returning the underlying cache
    /// reference.
    pub const fn into_cache(self) -> &'a InMemoryCache {
        self.0
    }

    /// Calculate the permissions of a member in a guild channel.
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
    /// use twilight_cache_inmemory::InMemoryCache;
    /// use twilight_model::id::{ChannelId, UserId};
    ///
    /// let cache = InMemoryCache::new();
    ///
    /// // later on...
    ///
    /// let channel_id = ChannelId(4);
    /// let user_id = UserId(5);
    ///
    /// let permissions = cache.permissions().in_channel(user_id, channel_id)?;
    /// println!(
    ///     "User {} in channel {} has permissions {:?}",
    ///     user_id,
    ///     channel_id,
    ///     permissions,
    /// );
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
    /// [`ResourceType::CHANNEL`]: crate::ResourceType::CHANNEL
    /// [`ResourceType::MEMBER`]: crate::ResourceType::MEMBER
    /// [`ResourceType::ROLE`]: crate::ResourceType::ROLE
    /// [`ResourceType`]: crate::ResourceType
    pub fn in_channel(
        &self,
        user_id: UserId,
        channel_id: ChannelId,
    ) -> Result<Permissions, ChannelError> {
        let channel = (self.0)
            .0
            .channels_guild
            .get(&channel_id)
            .ok_or(ChannelError {
                kind: ChannelErrorType::ChannelUnavailable { channel_id },
                source: None,
            })?;

        let guild_id = channel.data.guild_id().ok_or(ChannelError {
            kind: ChannelErrorType::ChannelUnavailable { channel_id },
            source: None,
        })?;
        let MemberRoles { assigned, everyone } = self
            .member_roles(user_id, guild_id)
            .map_err(ChannelError::from_member_roles)?;

        let overwrites = match &channel.data {
            GuildChannel::Category(c) => &c.permission_overwrites,
            GuildChannel::Stage(c) => &c.permission_overwrites,
            GuildChannel::Text(c) => &c.permission_overwrites,
            GuildChannel::Voice(c) => &c.permission_overwrites,
        };

        let calculator =
            PermissionCalculator::new(guild_id, user_id, everyone, assigned.as_slice());

        Ok(calculator.in_channel(channel.data.kind(), overwrites))
    }

    /// Calculate the guild-level permissions of a member.
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
    /// use twilight_cache_inmemory::InMemoryCache;
    /// use twilight_model::id::{GuildId, UserId};
    ///
    /// let cache = InMemoryCache::new();
    ///
    /// // later on...
    ///
    /// let guild_id = GuildId(4);
    /// let user_id = UserId(5);
    ///
    /// let permissions = cache.permissions().root(user_id, guild_id)?;
    /// println!(
    ///     "User {} in guild {} has permissions {:?}",
    ///     user_id,
    ///     guild_id,
    ///     permissions,
    /// );
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
    /// [`ResourceType::MEMBER`]: crate::ResourceType::MEMBER
    /// [`ResourceType::ROLE`]: crate::ResourceType::ROLE
    /// [`ResourceType`]: crate::ResourceType
    pub fn root(&self, user_id: UserId, guild_id: GuildId) -> Result<Permissions, RootError> {
        let MemberRoles { assigned, everyone } = self
            .member_roles(user_id, guild_id)
            .map_err(RootError::from_member_roles)?;
        let calculator =
            PermissionCalculator::new(guild_id, user_id, everyone, assigned.as_slice());

        Ok(calculator.root())
    }

    /// Retrieve a member's roles' permissions and the guild's `@everyone`
    /// role's permissions.
    ///
    /// # Errors
    ///
    /// Returns [`MemberRolesErrorType::MemberMissing`] if the member is missing
    /// from the cache.
    ///
    /// Returns [`MemberRolesErrorType::RoleMissing`] if a role is missing from
    /// the cache.
    fn member_roles(
        &self,
        user_id: UserId,
        guild_id: GuildId,
    ) -> Result<MemberRoles, MemberRolesErrorType> {
        let member = if let Some(member) = (self.0).0.members.get(&(guild_id, user_id)) {
            member
        } else {
            return Err(MemberRolesErrorType::MemberMissing { guild_id, user_id });
        };

        let mut member_roles = Vec::with_capacity(member.roles.len());

        for role_id in &member.roles {
            let role = if let Some(role) = (self.0).0.roles.get(role_id) {
                role
            } else {
                return Err(MemberRolesErrorType::RoleMissing { role_id: *role_id });
            };

            member_roles.push((*role_id, role.data.permissions));
        }

        // Assume that the `@everyone` role is always present, so do this last.
        let everyone_role_id = RoleId(guild_id.0);

        if let Some(everyone_role) = (self.0).0.roles.get(&everyone_role_id) {
            Ok(MemberRoles {
                assigned: member_roles,
                everyone: everyone_role.data.permissions,
            })
        } else {
            Err(MemberRolesErrorType::RoleMissing {
                role_id: everyone_role_id,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ChannelError, ChannelErrorType, InMemoryCachePermissions, RootError, RootErrorType,
    };
    use static_assertions::{assert_fields, assert_impl_all};
    use std::fmt::Debug;

    assert_fields!(ChannelErrorType::ChannelUnavailable: channel_id);
    assert_fields!(ChannelErrorType::MemberUnavailable: guild_id, user_id);
    assert_fields!(ChannelErrorType::RoleUnavailable: role_id);
    assert_impl_all!(ChannelErrorType: Debug, Send, Sync);
    assert_impl_all!(ChannelError: Debug, Send, Sync);
    assert_impl_all!(InMemoryCachePermissions<'_>: Clone, Debug, Send, Sync);
    assert_fields!(RootErrorType::MemberUnavailable: guild_id, user_id);
    assert_fields!(RootErrorType::RoleUnavailable: role_id);
    assert_impl_all!(RootErrorType: Debug, Send, Sync);
    assert_impl_all!(RootError: Debug, Send, Sync);
}
