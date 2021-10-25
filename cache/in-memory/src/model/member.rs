use serde::Serialize;
use twilight_model::{
    application::interaction::application_command::InteractionMember,
    datetime::Timestamp,
    guild::{Member, PartialMember},
    id::{GuildId, RoleId, UserId},
};

/// Represents a cached [`Member`].
///
/// [`Member`]: twilight_model::guild::Member
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CachedMember {
    pub(crate) deaf: Option<bool>,
    pub(crate) guild_id: GuildId,
    pub(crate) joined_at: Timestamp,
    pub(crate) mute: Option<bool>,
    pub(crate) nick: Option<String>,
    pub(crate) pending: bool,
    pub(crate) premium_since: Option<Timestamp>,
    pub(crate) roles: Vec<RoleId>,
    pub(crate) user_id: UserId,
}

impl CachedMember {
    /// Whether the member is deafened in a voice channel.
    pub const fn deaf(&self) -> Option<bool> {
        self.deaf
    }

    /// ID of the guild this member is a part of.
    pub const fn guild_id(&self) -> GuildId {
        self.guild_id
    }

    /// [`Timestamp`] of this member's join date.
    pub const fn joined_at(&self) -> Option<Timestamp> {
        Some(self.joined_at)
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
    pub fn roles(&self) -> &[RoleId] {
        &self.roles
    }

    /// ID of the user relating to the member.
    pub const fn user_id(&self) -> UserId {
        self.user_id
    }
}

impl PartialEq<Member> for CachedMember {
    fn eq(&self, other: &Member) -> bool {
        (
            self.deaf,
            self.joined_at,
            self.mute,
            &self.nick,
            self.pending,
            self.premium_since,
            &self.roles,
            self.user_id,
        ) == (
            Some(other.deaf),
            other.joined_at,
            Some(other.mute),
            &other.nick,
            other.pending,
            other.premium_since,
            &other.roles,
            self.user_id,
        )
    }
}

impl PartialEq<&PartialMember> for CachedMember {
    fn eq(&self, other: &&PartialMember) -> bool {
        (
            self.deaf,
            self.joined_at,
            self.mute,
            &self.nick,
            self.premium_since,
            &self.roles,
        ) == (
            Some(other.deaf),
            other.joined_at,
            Some(other.mute),
            &other.nick,
            other.premium_since,
            &other.roles,
        )
    }
}

impl PartialEq<&InteractionMember> for CachedMember {
    fn eq(&self, other: &&InteractionMember) -> bool {
        (
            self.joined_at,
            self.nick.as_deref(),
            self.premium_since,
            &self.roles,
        ) == (
            other.joined_at,
            other.nick.as_deref(),
            other.premium_since,
            &other.roles,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::CachedMember;
    use static_assertions::assert_fields;
    use twilight_model::{
        datetime::Timestamp,
        guild::{Member, PartialMember},
        id::{GuildId, RoleId, UserId},
        user::User,
    };

    assert_fields!(
        CachedMember: deaf,
        guild_id,
        joined_at,
        mute,
        nick,
        pending,
        premium_since,
        roles,
        user_id
    );

    fn cached_member() -> CachedMember {
        let joined_at = Timestamp::from_secs(1_632_072_645).expect("non zero");

        CachedMember {
            deaf: Some(false),
            guild_id: GuildId::new(3).expect("non zero"),
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
            banner: None,
            bot: false,
            discriminator: 1,
            email: None,
            flags: None,
            id: UserId::new(1).expect("non zero"),
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
    fn test_eq_member() {
        let joined_at = Timestamp::from_secs(1_632_072_645).expect("non zero");

        let member = Member {
            deaf: false,
            guild_id: GuildId::new(3).expect("non zero"),
            hoisted_role: Some(RoleId::new(4).expect("non zero")),
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
    fn test_eq_partial_member() {
        let joined_at = Timestamp::from_secs(1_632_072_645).expect("non zero");

        let member = PartialMember {
            deaf: false,
            joined_at,
            mute: true,
            nick: Some("member nick".to_owned()),
            permissions: None,
            premium_since: None,
            roles: Vec::new(),
            user: None,
        };

        assert_eq!(cached_member(), &member);
    }
}
