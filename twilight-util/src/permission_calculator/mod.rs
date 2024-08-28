//! Calculate the permissions of a member on a guild-level or a channel-level.
//!
//! # Examples
//!
//! ## Calculating member permissions in a channel
//!
//! Take a scenario where a member has two roles: the `@everyone` role (with the
//! same ID as the guild) that grants the [View Channel] permission across the
//! whole guild, and a second role that grants the [Send Messages] permission
//! across the whole guild. This means that - across the server - the member
//! will have the [View Channel] and [Send Messages] permissions, unless denied
//! or expanded by channel-specific permission overwrites.
//!
//! In a given channel, there are two permission overwrites: one for the
//! `@everyone` role and one for the member itself. These overwrites look
//! like:
//!
//! - `@everyone` role is allowed the [Embed Links] and [Add Reactions]
//!   permissions; and
//! - member is denied the [Send Messages] permission.
//!
//! Taking into account the guild root-level permissions and the permission
//! overwrites, the end result is that in the specified channel the user has
//! the [View Channel], [Embed Links], and [Add Reactions] permission, but is
//! denied the [Send Messages] permission that their second role was granted on
//! a root level.
//!
//! Let's see that in code:
//!
//! ```
//! use twilight_model::{
//!     channel::{
//!         permission_overwrite::{PermissionOverwrite, PermissionOverwriteType},
//!         ChannelType,
//!     },
//!     guild::Permissions,
//!     id::Id,
//! };
//! use twilight_util::permission_calculator::PermissionCalculator;
//!
//! let guild_id = Id::new(1);
//! let user_id = Id::new(3);
//!
//! // Guild-level @everyone role that, by default, allows everyone to view
//! // channels.
//! let everyone_role = Permissions::VIEW_CHANNEL;
//!
//! // Roles that the member has assigned to them and their permissions. This
//! // should not include the `@everyone` role.
//! let member_roles = &[
//!     // Guild-level permission that grants members with the role the Send
//!     // Messages permission.
//!     (Id::new(2), Permissions::SEND_MESSAGES),
//! ];
//!
//! let channel_overwrites = &[
//!     // All members are given the Add Reactions and Embed Links members via
//!     // the `@everyone` role.
//!     PermissionOverwrite {
//!         allow: Permissions::ADD_REACTIONS | Permissions::EMBED_LINKS,
//!         deny: Permissions::empty(),
//!         id: Id::new(1),
//!         kind: PermissionOverwriteType::Role,
//!     },
//!     // Member is denied the Send Messages permission.
//!     PermissionOverwrite {
//!         allow: Permissions::empty(),
//!         deny: Permissions::SEND_MESSAGES,
//!         id: user_id.cast(),
//!         kind: PermissionOverwriteType::Member,
//!     },
//! ];
//!
//! let calculator = PermissionCalculator::new(guild_id, user_id, everyone_role, member_roles);
//! let calculated_permissions = calculator.in_channel(ChannelType::GuildText, channel_overwrites);
//!
//! // Now that we've got the member's permissions in the channel, we can
//! // check that they have the server-wide View Channel permission and
//! // the Add Reactions permission granted, but their guild-wide Send Messages
//! // permission was denied. Additionally, since the user can't send messages,
//! // their Embed Links permission was removed.
//!
//! let expected = Permissions::ADD_REACTIONS | Permissions::VIEW_CHANNEL;
//! assert!(!calculated_permissions.contains(Permissions::EMBED_LINKS));
//! assert!(!calculated_permissions.contains(Permissions::SEND_MESSAGES));
//! assert_eq!(expected, calculated_permissions);
//! ```
//!
//! [Add Reactions]: twilight_model::guild::Permissions::ADD_REACTIONS
//! [Embed Links]: twilight_model::guild::Permissions::EMBED_LINKS
//! [Send Messages]: twilight_model::guild::Permissions::SEND_MESSAGES
//! [View Channel]: twilight_model::guild::Permissions::VIEW_CHANNEL

mod bitops;
mod preset;

use self::preset::{
    PERMISSIONS_MESSAGING, PERMISSIONS_ROOT_ONLY, PERMISSIONS_STAGE_OMIT, PERMISSIONS_TEXT_OMIT,
    PERMISSIONS_VOICE_OMIT,
};
use twilight_model::{
    channel::{
        permission_overwrite::{PermissionOverwrite, PermissionOverwriteType},
        ChannelType,
    },
    guild::Permissions,
    id::{
        marker::{GuildMarker, RoleMarker, UserMarker},
        Id,
    },
};

/// Calculate the permissions of a member.
///
/// Using the member calculator you can calculate the member's permissions in
/// the [root-level][`root`] of a guild or [in a given channel][`in_channel`].
///
/// [`in_channel`]: Self::in_channel
/// [`root`]: Self::root
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "calculators aren't useful if you don't calculate permissions"]
pub struct PermissionCalculator<'a> {
    /// Permissions of the `@everyone` role for the guild.
    everyone_role: Permissions,
    /// ID of the guild.
    guild_id: Id<GuildMarker>,
    /// Slice of tuples of the member's roles and their permissions.
    member_roles: &'a [(Id<RoleMarker>, Permissions)],
    /// ID of the owner.
    owner_id: Option<Id<UserMarker>>,
    /// ID of the user whose permissions are being calculated.
    user_id: Id<UserMarker>,
}

impl<'a> PermissionCalculator<'a> {
    /// Create a calculator to calculate the permissions of a member.
    ///
    /// `everyone_role` is the permissions of the `@everyone` role on a
    /// guild-level; the permissions may be empty. The `@everyone` role's ID is
    /// the same as that of the `guild_id`.
    ///
    /// The provided member's roles *should not* contain the `@everyone` role.
    #[must_use = "calculators should be used to calculate permissions"]
    pub const fn new(
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
        everyone_role: Permissions,
        member_roles: &'a [(Id<RoleMarker>, Permissions)],
    ) -> Self {
        Self {
            everyone_role,
            guild_id,
            owner_id: None,
            member_roles,
            user_id,
        }
    }

    /// Configure the ID of the owner of the guild.
    ///
    /// This should be used if you don't want to manually take the user ID and
    /// owner ID in account beforehand.
    ///
    /// If the member's ID is the same as the owner's ID then permission
    /// calculating methods such as [`root`] will return all permissions
    /// enabled.
    ///
    /// [`root`]: Self::root
    #[must_use = "calculators should be used to calculate permissions"]
    pub const fn owner_id(mut self, owner_id: Id<UserMarker>) -> Self {
        self.owner_id = Some(owner_id);

        self
    }

    /// Calculate the guild-level permissions of a member.
    #[must_use = "calculating permissions is only useful if they're used"]
    pub const fn root(&self) -> Permissions {
        // If the user is the owner, then we can just return all of the
        // permissions.
        if matches!(self.owner_id, Some(id) if id.get() == self.user_id.get()) {
            return Permissions::all();
        }

        // If the `@everyone` role has the `ADMINISTRATOR` permission for some
        // reason, then we can just return all permissions.
        if self.everyone_role.contains(Permissions::ADMINISTRATOR) {
            return Permissions::all();
        }

        // The permissions that the @everyone role has is the baseline.
        let mut permissions = self.everyone_role;

        // At time of writing `const` functions don't support `for` loops, so we
        // use a `while` loop.
        let member_role_count = self.member_roles.len();
        let mut idx = 0;

        // Loop over all of the member's roles, adding them to the total
        // permissions. Role permissions are only additive.
        //
        // If one of the roles contains the `ADMINISTRATOR` permission then the
        // loop can be short-circuited.
        while idx < member_role_count {
            let (_, role_permissions) = self.member_roles[idx];

            if role_permissions.contains(Permissions::ADMINISTRATOR) {
                return Permissions::all();
            }

            permissions = bitops::insert(permissions, role_permissions);
            idx += 1;
        }

        permissions
    }

    /// Calculate the permissions of the member in a channel, taking into
    /// account a combination of the guild-level permissions and channel-level
    /// permissions.
    ///
    /// **Note** that this method will not return guild-level permissions such
    /// as [Manage Guild Expressions]; if you need the guild-level permissions
    /// use [`root`].
    ///
    /// # Conditional exclusions
    ///
    /// When the member doesn't have the "View Channel" permission then an empty
    /// permission set will be returned. This will happen in the following
    /// circumstances:
    ///
    /// - When the permission is denied on the role level and
    ///   isn't enabled on a role or member permission overwrite;
    /// - When the permission is denied on a role permission overwrite but isn't
    ///   enabled on a member permission overwrite; or
    /// - When permission isn't enabled on a guild level and isn't enabled via a
    ///   permission overwrite.
    ///
    /// When the [Send Messages] permission is denied and is not similarly
    /// enabled like above then the [Attach Files], [Embed Links],
    /// [Mention Everyone], and [Send TTS Messages] permissions will not be
    /// present in the returned permission set.
    ///
    /// # Channel-based exclusions
    ///
    /// Permissions are removed based on the type of a channel. For example,
    /// when calculating the permissions of a voice channel we can know that if
    /// [Send Messages] is granted on a guild-level to everyone then it is
    /// omitted from the permissions for a specific channel.
    ///
    /// ## Stage Channels
    ///
    /// When the given channel type is a guild stage channel then the
    /// following permissions will be removed:
    ///
    /// - [Add Reactions]
    /// - [Attach Files]
    /// - [Deafen Members]
    /// - [Embed Links]
    /// - [Manage Webhooks]
    /// - [Mention Everyone]
    /// - [Priority Speaker]
    /// - [Read Message History]
    /// - [Send Messages]
    /// - [Send TTS Messages]
    /// - [Stream]
    /// - [Speak]
    /// - [Use External Emojis]
    /// - [Use Slash Commands]
    /// - [Use VAD]
    ///
    /// ## Text Channels
    ///
    /// When the given channel type is a guild text channel then the
    /// following permissions will be removed:
    ///
    /// - [Connect]
    /// - [Deafen Members]
    /// - [Move Members]
    /// - [Mute Members]
    /// - [Priority Speaker]
    /// - [Request To Speak]
    /// - [Speak]
    /// - [Stream]
    /// - [Use VAD]
    ///
    /// # Voice Channels
    ///
    /// When the given channel type is a guild voice channel then the
    /// following permissions will be removed:
    ///
    /// - [Add Reactions]
    /// - [Attach Files]
    /// - [Embed Links]
    /// - [Manage Messages]
    /// - [Manage Webhooks]
    /// - [Mention Everyone]
    /// - [Read Message History]
    /// - [Request To Speak]
    /// - [Send Messages]
    /// - [Send TTS Messages]
    /// - [Use External Emojis]
    /// - [Use Slash Commands]
    ///
    /// # Guild-based exclusions
    ///
    /// The following guild-level permissions will always be removed:
    ///
    /// - [Administrator]
    /// - [Ban Members]
    /// - [Change Nickname]
    /// - [Kick Members]
    /// - [Manage Guild Expressions]
    /// - [Manage Guild]
    /// - [Manage Nicknames]
    /// - [View Audit Log]
    /// - [View Guild Insights]
    ///
    /// If you need to know a member's guild-level permissions - such as whether
    /// they have the [View Audit Log] permission - use [`root`] instead.
    ///
    /// # Examples
    ///
    /// See the crate-level documentation for an example.
    ///
    /// [`root`]: Self::root
    /// [Administrator]: twilight_model::guild::Permissions::ADMINISTRATOR
    /// [Add Reactions]: twilight_model::guild::Permissions::ADD_REACTIONS
    /// [Attach Files]: twilight_model::guild::Permissions::ATTACH_FILES
    /// [Ban Members]: twilight_model::guild::Permissions::BAN_MEMBERS
    /// [Change Nickname]: twilight_model::guild::Permissions::CHANGE_NICKNAME
    /// [Connect]: twilight_model::guild::Permissions::CONNECT
    /// [Deafen Members]: twilight_model::guild::Permissions::DEAFEN_MEMBERS
    /// [Embed Links]: twilight_model::guild::Permissions::EMBED_LINKS
    /// [Kick Members]: twilight_model::guild::Permissions::KICK_MEMBERS
    /// [Manage Guild Expressions]: twilight_model::guild::Permissions::MANAGE_GUILD_EXPRESSIONS
    /// [Manage Guild]: twilight_model::guild::Permissions::MANAGE_GUILD
    /// [Manage Messages]: twilight_model::guild::Permissions::MANAGE_MESSAGES
    /// [Manage Nicknames]: twilight_model::guild::Permissions::MANAGE_NICKNAMES
    /// [Manage Webhooks]: twilight_model::guild::Permissions::MANAGE_WEBHOOKS
    /// [Mention Everyone]: twilight_model::guild::Permissions::MENTION_EVERYONE
    /// [Move Members]: twilight_model::guild::Permissions::MOVE_MEMBERS
    /// [Mute Members]: twilight_model::guild::Permissions::MUTE_MEMBERS
    /// [Priority Speaker]: twilight_model::guild::Permissions::PRIORITY_SPEAKER
    /// [Read Message History]: twilight_model::guild::Permissions::READ_MESSAGE_HISTORY
    /// [Request To Speak]: twilight_model::guild::Permissions::REQUEST_TO_SPEAK
    /// [Send Messages]: twilight_model::guild::Permissions::SEND_MESSAGES
    /// [Send TTS Messages]: twilight_model::guild::Permissions::SEND_TTS_MESSAGES
    /// [Speak]: twilight_model::guild::Permissions::SPEAK
    /// [Stream]: twilight_model::guild::Permissions::STREAM
    /// [Use External Emojis]: twilight_model::guild::Permissions::USE_EXTERNAL_EMOJIS
    /// [Use Slash Commands]: twilight_model::guild::Permissions::USE_SLASH_COMMANDS
    /// [Use VAD]: twilight_model::guild::Permissions::USE_VAD
    /// [View Audit Log]: twilight_model::guild::Permissions::VIEW_AUDIT_LOG
    /// [View Guild Insights]: twilight_model::guild::Permissions::VIEW_GUILD_INSIGHTS
    #[must_use = "calculating permissions is only useful if they're used"]
    pub const fn in_channel(
        self,
        channel_type: ChannelType,
        channel_overwrites: &[PermissionOverwrite],
    ) -> Permissions {
        let mut permissions = self.root();

        // If the user contains the administrator privilege from the calculated
        // root permissions, then we do not need to do any more work.
        if permissions.contains(Permissions::ADMINISTRATOR) {
            return Permissions::all();
        }

        permissions = bitops::remove(permissions, PERMISSIONS_ROOT_ONLY);

        permissions = process_permission_overwrites(
            permissions,
            channel_overwrites,
            self.member_roles,
            self.guild_id,
            self.user_id,
        );

        // If the permission set is empty then we don't need to do any removals.
        if permissions.is_empty() {
            return permissions;
        }

        // Remove permissions that can't be used in a channel, i.e. are relevant
        // to guild-level permission calculating.
        permissions = bitops::remove(permissions, PERMISSIONS_ROOT_ONLY);

        // Remove the permissions not used by a channel depending on the channel
        // type.
        if matches!(channel_type, ChannelType::GuildStageVoice) {
            permissions = bitops::remove(permissions, PERMISSIONS_STAGE_OMIT);
        } else if matches!(channel_type, ChannelType::GuildText) {
            permissions = bitops::remove(permissions, PERMISSIONS_TEXT_OMIT);
        } else if matches!(channel_type, ChannelType::GuildVoice) {
            permissions = bitops::remove(permissions, PERMISSIONS_VOICE_OMIT);
        }

        permissions
    }
}

const fn has_role(roles: &[(Id<RoleMarker>, Permissions)], role_id: Id<RoleMarker>) -> bool {
    let len = roles.len();
    let mut idx = 0;

    while idx < len {
        let (iter_role_id, _) = roles[idx];

        if iter_role_id.get() == role_id.get() {
            return true;
        }

        idx += 1;
    }

    false
}

const fn process_permission_overwrites(
    mut permissions: Permissions,
    channel_overwrites: &[PermissionOverwrite],
    member_roles: &[(Id<RoleMarker>, Permissions)],
    configured_guild_id: Id<GuildMarker>,
    configured_user_id: Id<UserMarker>,
) -> Permissions {
    // Hierarchy documentation:
    // <https://discord.com/developers/docs/topics/permissions>
    let mut member_allow = Permissions::empty();
    let mut member_deny = Permissions::empty();
    let mut roles_allow = Permissions::empty();
    let mut roles_deny = Permissions::empty();

    let channel_overwrite_len = channel_overwrites.len();
    let mut idx = 0;

    while idx < channel_overwrite_len {
        let overwrite = &channel_overwrites[idx];

        match overwrite.kind {
            PermissionOverwriteType::Role => {
                // We need to process the @everyone role first, so apply it
                // straight to the permissions. The other roles' permissions
                // will be applied later.
                if overwrite.id.get() == configured_guild_id.get() {
                    permissions = bitops::remove(permissions, overwrite.deny);
                    permissions = bitops::insert(permissions, overwrite.allow);

                    idx += 1;

                    continue;
                }

                if !has_role(member_roles, overwrite.id.cast()) {
                    idx += 1;

                    continue;
                }

                roles_allow = bitops::insert(roles_allow, overwrite.allow);
                roles_deny = bitops::insert(roles_deny, overwrite.deny);
            }
            PermissionOverwriteType::Member => {
                if overwrite.id.get() == configured_user_id.get() {
                    member_allow = bitops::insert(member_allow, overwrite.allow);
                    member_deny = bitops::insert(member_deny, overwrite.deny);
                }
            }
            // Unknown, impossible to try and calculate with this
            PermissionOverwriteType::Unknown(_) => (),
            _ => unimplemented!(),
        }

        idx += 1;
    }

    let user_view_allowed = member_allow.contains(Permissions::VIEW_CHANNEL);

    let user_view_denied = member_deny.contains(Permissions::VIEW_CHANNEL) && !user_view_allowed;

    let role_view_denied = roles_deny.contains(Permissions::VIEW_CHANNEL)
        && !roles_allow.contains(Permissions::VIEW_CHANNEL)
        && !user_view_allowed;

    if user_view_denied || role_view_denied {
        return Permissions::empty();
    }

    // If the member or any of their roles denies the Send Messages
    // permission, then the rest of the messaging-related permissions can be
    // removed.
    let user_send_allowed = member_allow.contains(Permissions::SEND_MESSAGES);

    let user_send_denied = member_deny.contains(Permissions::SEND_MESSAGES) && !user_send_allowed;

    let role_send_denied = roles_deny.contains(Permissions::SEND_MESSAGES)
        && !roles_allow.contains(Permissions::SEND_MESSAGES)
        && !user_send_allowed;

    if user_send_denied || role_send_denied {
        member_allow = bitops::remove(member_allow, PERMISSIONS_MESSAGING);
        roles_allow = bitops::remove(roles_allow, PERMISSIONS_MESSAGING);
        permissions = bitops::remove(permissions, PERMISSIONS_MESSAGING);
    }

    // Member overwrites take precedence over role overwrites. Permission
    // allows take precedence over denies.
    permissions = bitops::remove(permissions, roles_deny);
    permissions = bitops::insert(permissions, roles_allow);
    permissions = bitops::remove(permissions, member_deny);
    permissions = bitops::insert(permissions, member_allow);

    permissions
}

#[cfg(test)]
mod tests {
    use super::{preset::PERMISSIONS_ROOT_ONLY, PermissionCalculator};
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;
    use twilight_model::{
        channel::{
            permission_overwrite::{PermissionOverwrite, PermissionOverwriteType},
            ChannelType,
        },
        guild::Permissions,
        id::Id,
    };

    assert_impl_all!(PermissionCalculator<'_>: Clone, Debug, Eq, PartialEq, Send, Sync);

    #[test]
    fn owner_is_admin() {
        let guild_id = Id::new(1);
        let user_id = Id::new(2);
        let everyone_role = Permissions::SEND_MESSAGES;
        let roles = &[];

        let calculator =
            PermissionCalculator::new(guild_id, user_id, everyone_role, roles).owner_id(user_id);

        assert_eq!(Permissions::all(), calculator.root());
    }

    // Test that a permission overwrite denying the "View Channel" permission
    // implicitly denies all other permissions.
    #[test]
    fn view_channel_deny_implicit() {
        let guild_id = Id::new(1);
        let user_id = Id::new(2);
        let everyone_role = Permissions::MENTION_EVERYONE | Permissions::SEND_MESSAGES;
        let roles = &[(Id::new(3), Permissions::empty())];

        {
            // First, test when it's denied for an overwrite on a role the user
            // has.
            let overwrites = &[PermissionOverwrite {
                allow: Permissions::SEND_TTS_MESSAGES,
                deny: Permissions::VIEW_CHANNEL,
                id: Id::new(3),
                kind: PermissionOverwriteType::Role,
            }];

            let calculated = PermissionCalculator::new(guild_id, user_id, everyone_role, roles)
                .in_channel(ChannelType::GuildText, overwrites);

            assert_eq!(calculated, Permissions::empty());
        }

        // And now that it's denied for an overwrite on the member.
        {
            let overwrites = &[PermissionOverwrite {
                allow: Permissions::SEND_TTS_MESSAGES,
                deny: Permissions::VIEW_CHANNEL,
                id: Id::new(2),
                kind: PermissionOverwriteType::Member,
            }];

            let calculated = PermissionCalculator::new(guild_id, user_id, everyone_role, roles)
                .in_channel(ChannelType::GuildText, overwrites);

            assert_eq!(calculated, Permissions::empty());
        }

        // Member overwrites take precedence over role overwrites.
        {
            let overwrites = &[
                PermissionOverwrite {
                    allow: Permissions::VIEW_CHANNEL,
                    deny: Permissions::empty(),
                    id: Id::new(2),
                    kind: PermissionOverwriteType::Member,
                },
                PermissionOverwrite {
                    allow: Permissions::empty(),
                    deny: Permissions::VIEW_CHANNEL,
                    id: Id::new(3),
                    kind: PermissionOverwriteType::Role,
                },
            ];

            let calculated = PermissionCalculator::new(guild_id, user_id, everyone_role, roles)
                .in_channel(ChannelType::GuildText, overwrites);

            assert_eq!(
                calculated,
                Permissions::VIEW_CHANNEL
                    | Permissions::SEND_MESSAGES
                    | Permissions::MENTION_EVERYONE
            );
        }
    }

    #[test]
    fn remove_text_and_stage_perms_when_voice() {
        let guild_id = Id::new(1);
        let user_id = Id::new(2);
        let everyone_role = Permissions::CONNECT;
        let roles = &[(Id::new(3), Permissions::SEND_MESSAGES)];

        let calculated = PermissionCalculator::new(guild_id, user_id, everyone_role, roles)
            .in_channel(ChannelType::GuildVoice, &[]);

        assert_eq!(calculated, Permissions::CONNECT);
    }

    #[test]
    fn remove_audio_perms_when_text() {
        let guild_id = Id::new(1);
        let user_id = Id::new(2);
        let everyone_role = Permissions::CONNECT;
        let roles = &[(Id::new(3), Permissions::SEND_MESSAGES)];

        let calculated = PermissionCalculator::new(guild_id, user_id, everyone_role, roles)
            .in_channel(ChannelType::GuildText, &[]);

        // The `CONNECT` permission isn't included because text channels don't
        // have the permission.
        assert_eq!(calculated, Permissions::SEND_MESSAGES);
    }

    // Test that denying the "Send Messages" permission denies all message
    // send related permissions.
    #[test]
    fn deny_send_messages_removes_related() {
        let guild_id = Id::new(1);
        let user_id = Id::new(2);
        let everyone_role =
            Permissions::MANAGE_MESSAGES | Permissions::EMBED_LINKS | Permissions::MENTION_EVERYONE;
        let roles = &[(Id::new(3), Permissions::empty())];

        // First, test when it's denied for an overwrite on a role the user has.
        let overwrites = &[PermissionOverwrite {
            allow: Permissions::ATTACH_FILES,
            deny: Permissions::SEND_MESSAGES,
            id: Id::new(3),
            kind: PermissionOverwriteType::Role,
        }];

        let calculated = PermissionCalculator::new(guild_id, user_id, everyone_role, roles)
            .in_channel(ChannelType::GuildText, overwrites);

        assert_eq!(calculated, Permissions::MANAGE_MESSAGES);
    }

    /// Test that a member that has a role with the "administrator" permission
    /// has all denying overwrites ignored.
    #[test]
    fn admin() {
        let member_roles = &[(Id::new(3), Permissions::ADMINISTRATOR)];
        let calc =
            PermissionCalculator::new(Id::new(1), Id::new(2), Permissions::empty(), member_roles);
        assert!(calc.root().is_all());

        // Ensure that the denial of "send messages" doesn't actually occur due
        // to the user being an administrator.
        assert!(calc.in_channel(ChannelType::GuildText, &[]).is_all());
    }

    /// Test that guild-level permissions are removed in the permissions for a
    /// channel of any type.
    #[test]
    fn guild_level_removed_in_channel() {
        const CHANNEL_TYPES: &[ChannelType] = &[
            ChannelType::GuildCategory,
            ChannelType::GuildAnnouncement,
            ChannelType::GuildStageVoice,
            ChannelType::GuildText,
            ChannelType::GuildVoice,
        ];

        // We need to remove the `ADMINISTRATOR` permission or else the
        // calculator will (correctly) return all permissions.
        let mut everyone = PERMISSIONS_ROOT_ONLY;
        everyone.remove(Permissions::ADMINISTRATOR);

        for kind in CHANNEL_TYPES {
            let calc = PermissionCalculator::new(Id::new(1), Id::new(2), everyone, &[]);
            let calculated = calc.in_channel(*kind, &[]);

            assert!(!calculated.intersects(PERMISSIONS_ROOT_ONLY));
        }
    }
}
