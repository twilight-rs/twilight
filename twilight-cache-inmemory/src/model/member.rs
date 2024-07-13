use std::ops::Deref;

use serde::Serialize;
use twilight_model::{
    application::interaction::InteractionMember,
    gateway::payload::incoming::MemberUpdate,
    guild::{Member, MemberFlags, PartialMember},
    id::{
        marker::{RoleMarker, UserMarker},
        Id,
    },
    util::{ImageHash, Timestamp},
};

use crate::CacheableMember;

/// Computed components required to complete a full cached interaction member
/// by implementing [`CacheableMember`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComputedInteractionMember {
    /// Member's guild avatar.
    pub avatar: Option<ImageHash>,
    /// Whether the member is deafened in a voice channel.
    pub deaf: Option<bool>,
    /// Member that performed the interaction.
    pub interaction_member: InteractionMember,
    /// Whether the member is muted in a voice channel.
    pub mute: Option<bool>,
    /// ID of the user relating to the member.
    pub user_id: Id<UserMarker>,
}

impl Deref for ComputedInteractionMember {
    type Target = InteractionMember;

    fn deref(&self) -> &Self::Target {
        &self.interaction_member
    }
}

/// Represents a cached [`Member`].
///
/// [`Member`]: twilight_model::guild::Member
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CachedMember {
    pub(crate) avatar: Option<ImageHash>,
    pub(crate) communication_disabled_until: Option<Timestamp>,
    pub(crate) deaf: Option<bool>,
    pub(crate) flags: MemberFlags,
    pub(crate) joined_at: Option<Timestamp>,
    pub(crate) mute: Option<bool>,
    pub(crate) nick: Option<String>,
    pub(crate) pending: bool,
    pub(crate) premium_since: Option<Timestamp>,
    pub(crate) roles: Vec<Id<RoleMarker>>,
    pub(crate) user_id: Id<UserMarker>,
}

impl CachedMember {
    /// Member's guild avatar.
    pub const fn avatar(&self) -> Option<ImageHash> {
        self.avatar
    }

    /// When the user can resume communication in a guild again.
    ///
    /// Checking if this value is [`Some`] is not enough to know if a used is currently
    /// timed out as Discord doesn't send any events when the timeout expires, and
    /// therefore the cache is not updated accordingly. You should ensure that the
    /// provided [`Timestamp`] is not in the past. See [discord-api-docs#4269].
    ///
    /// [discord-api-docs#4269]: https://github.com/discord/discord-api-docs/issues/4269
    pub const fn communication_disabled_until(&self) -> Option<Timestamp> {
        self.communication_disabled_until
    }

    /// Whether the member is deafened in a voice channel.
    pub const fn deaf(&self) -> Option<bool> {
        self.deaf
    }

    /// Flags for the member.
    ///
    /// Defaults to an empty bitfield.
    pub const fn flags(&self) -> MemberFlags {
        self.flags
    }

    /// [`Timestamp`] of this member's join date.
    pub const fn joined_at(&self) -> Option<Timestamp> {
        self.joined_at
    }

    /// Whether the member is muted in a voice channel.
    pub const fn mute(&self) -> Option<bool> {
        self.mute
    }

    /// Nickname of the member.
    pub fn nick(&self) -> Option<&str> {
        self.nick.as_deref()
    }

    /// Whether the member has not yet passed the guild's Membership Screening
    /// requirements.
    pub const fn pending(&self) -> bool {
        self.pending
    }

    /// [`Timestamp`] of the date the member boosted the guild.
    pub const fn premium_since(&self) -> Option<Timestamp> {
        self.premium_since
    }

    /// List of role IDs this member has.
    pub fn roles(&self) -> &[Id<RoleMarker>] {
        &self.roles
    }

    /// ID of the user relating to the member.
    pub const fn user_id(&self) -> Id<UserMarker> {
        self.user_id
    }
}

impl From<Member> for CachedMember {
    fn from(member: Member) -> Self {
        let Member {
            avatar,
            communication_disabled_until,
            deaf,
            flags,
            joined_at,
            mute,
            nick,
            pending,
            premium_since,
            roles,
            user,
        } = member;

        Self {
            avatar,
            communication_disabled_until,
            deaf: Some(deaf),
            flags,
            joined_at,
            mute: Some(mute),
            nick,
            pending,
            premium_since,
            roles,
            user_id: user.id,
        }
    }
}

impl From<ComputedInteractionMember> for CachedMember {
    fn from(member: ComputedInteractionMember) -> Self {
        let ComputedInteractionMember {
            avatar,
            deaf,
            mute,
            user_id,
            interaction_member,
        } = member;
        let InteractionMember {
            avatar: _,
            communication_disabled_until,
            flags,
            joined_at,
            nick,
            pending,
            permissions: _,
            premium_since,
            roles,
        } = interaction_member;

        Self {
            avatar,
            communication_disabled_until,
            deaf,
            flags,
            joined_at,
            mute,
            nick,
            pending,
            premium_since,
            roles,
            user_id,
        }
    }
}

impl From<(Id<UserMarker>, PartialMember)> for CachedMember {
    fn from((user_id, member): (Id<UserMarker>, PartialMember)) -> Self {
        let PartialMember {
            avatar,
            communication_disabled_until,
            deaf,
            flags,
            joined_at,
            mute,
            nick,
            permissions: _,
            premium_since,
            roles,
            user,
        } = member;

        Self {
            avatar,
            communication_disabled_until,
            deaf: Some(deaf),
            flags,
            joined_at,
            mute: Some(mute),
            nick,
            pending: false,
            premium_since,
            roles,
            user_id: user.map_or(user_id, |user| user.id),
        }
    }
}

impl PartialEq<Member> for CachedMember {
    fn eq(&self, other: &Member) -> bool {
        self.avatar == other.avatar
            && self.communication_disabled_until == other.communication_disabled_until
            && self.deaf == Some(other.deaf)
            && self.joined_at == other.joined_at
            && self.mute == Some(other.mute)
            && self.nick == other.nick
            && self.pending == other.pending
            && self.premium_since == other.premium_since
            && self.roles == other.roles
            && self.user_id == other.user.id
    }
}

impl PartialEq<PartialMember> for CachedMember {
    fn eq(&self, other: &PartialMember) -> bool {
        self.communication_disabled_until == other.communication_disabled_until
            && self.deaf == Some(other.deaf)
            && self.joined_at == other.joined_at
            && self.mute == Some(other.mute)
            && self.nick == other.nick
            && self.premium_since == other.premium_since
            && self.roles == other.roles
    }
}

impl PartialEq<InteractionMember> for CachedMember {
    fn eq(&self, other: &InteractionMember) -> bool {
        self.joined_at == other.joined_at
            && self.nick == other.nick
            && self.premium_since == other.premium_since
            && self.roles == other.roles
    }
}

impl CacheableMember for CachedMember {
    fn roles(&self) -> &[Id<RoleMarker>] {
        &self.roles
    }

    #[cfg(feature = "permission-calculator")]
    fn communication_disabled_until(&self) -> Option<Timestamp> {
        self.communication_disabled_until
    }

    fn avatar(&self) -> Option<ImageHash> {
        self.avatar
    }

    fn deaf(&self) -> Option<bool> {
        self.deaf
    }

    fn mute(&self) -> Option<bool> {
        self.mute
    }

    fn update_with_member_update(&mut self, member_update: &MemberUpdate) {
        self.avatar = member_update.avatar;
        self.deaf = member_update.deaf.or_else(|| self.deaf());
        self.mute = member_update.mute.or_else(|| self.mute());
        self.nick.clone_from(&member_update.nick);
        self.roles.clone_from(&member_update.roles);
        self.joined_at = member_update.joined_at;
        self.pending = member_update.pending;
        self.communication_disabled_until = member_update.communication_disabled_until;
    }
}

#[cfg(test)]
mod tests {
    use super::CachedMember;
    use static_assertions::assert_fields;
    use twilight_model::{
        guild::{Member, MemberFlags, PartialMember},
        id::Id,
        user::User,
        util::Timestamp,
    };

    assert_fields!(
        CachedMember: deaf,
        joined_at,
        mute,
        nick,
        pending,
        premium_since,
        roles,
        user_id
    );

    fn cached_member() -> CachedMember {
        let joined_at = Some(Timestamp::from_secs(1_632_072_645).expect("non zero"));
        let flags = MemberFlags::BYPASSES_VERIFICATION | MemberFlags::DID_REJOIN;
        CachedMember {
            avatar: None,
            communication_disabled_until: None,
            deaf: Some(false),
            flags,
            joined_at,
            mute: Some(true),
            nick: Some("member nick".to_owned()),
            pending: false,
            premium_since: None,
            roles: Vec::new(),
            user_id: user().id,
        }
    }

    fn user() -> User {
        User {
            accent_color: None,
            avatar: None,
            avatar_decoration: None,
            avatar_decoration_data: None,
            banner: None,
            bot: false,
            discriminator: 1,
            email: None,
            flags: None,
            global_name: Some("test".to_owned()),
            id: Id::new(1),
            locale: None,
            mfa_enabled: None,
            name: "bar".to_owned(),
            premium_type: None,
            public_flags: None,
            system: None,
            verified: None,
        }
    }

    #[test]
    fn eq_member() {
        let joined_at = Some(Timestamp::from_secs(1_632_072_645).expect("non zero"));
        let flags = MemberFlags::BYPASSES_VERIFICATION | MemberFlags::DID_REJOIN;

        let member = Member {
            avatar: None,
            communication_disabled_until: None,
            deaf: false,
            flags,
            joined_at,
            mute: true,
            nick: Some("member nick".to_owned()),
            pending: false,
            premium_since: None,
            roles: Vec::new(),
            user: user(),
        };

        assert_eq!(cached_member(), member);
    }

    #[test]
    fn eq_partial_member() {
        let joined_at = Some(Timestamp::from_secs(1_632_072_645).expect("non zero"));
        let flags = MemberFlags::BYPASSES_VERIFICATION | MemberFlags::DID_REJOIN;

        let member = PartialMember {
            avatar: None,
            communication_disabled_until: None,
            deaf: false,
            flags,
            joined_at,
            mute: true,
            nick: Some("member nick".to_owned()),
            permissions: None,
            premium_since: None,
            roles: Vec::new(),
            user: None,
        };

        assert_eq!(cached_member(), member);
    }
}
