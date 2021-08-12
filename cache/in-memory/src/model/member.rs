use serde::Serialize;
use twilight_model::{
    application::interaction::application_command::InteractionMember,
    guild::{Member, PartialMember},
    id::{GuildId, RoleId, UserId},
};

/// Represents a cached [`Member`].
///
/// [`Member`]: twilight_model::guild::Member
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CachedMember {
    /// Whether the member is deafened in a voice channel.
    pub deaf: Option<bool>,
    /// ID of the guild this member is a part of.
    pub guild_id: GuildId,
    /// ISO 8601 timestamp of this member's join date.
    pub joined_at: Option<String>,
    /// Whether the member is muted in a voice channel.
    pub mute: Option<bool>,
    /// Nickname of the member.
    pub nick: Option<String>,
    /// Whether the member has not yet passed the guild's Membership Screening requirements.
    pub pending: bool,
    /// ISO 8601 timestamp of the date the member boosted the guild.
    pub premium_since: Option<String>,
    /// List of role IDs this member has.
    pub roles: Vec<RoleId>,
    /// ID of the user relating to the member.
    pub user_id: UserId,
}

impl PartialEq<Member> for CachedMember {
    fn eq(&self, other: &Member) -> bool {
        (
            self.deaf,
            self.joined_at.as_ref(),
            self.mute,
            &self.nick,
            self.pending,
            self.premium_since.as_ref(),
            &self.roles,
            self.user_id,
        ) == (
            Some(other.deaf),
            other.joined_at.as_ref(),
            Some(other.mute),
            &other.nick,
            other.pending,
            other.premium_since.as_ref(),
            &other.roles,
            self.user_id,
        )
    }
}

impl PartialEq<&PartialMember> for CachedMember {
    fn eq(&self, other: &&PartialMember) -> bool {
        (
            self.deaf,
            self.joined_at.as_ref(),
            self.mute,
            &self.nick,
            &self.premium_since,
            &self.roles,
        ) == (
            Some(other.deaf),
            other.joined_at.as_ref(),
            Some(other.mute),
            &other.nick,
            &other.premium_since,
            &other.roles,
        )
    }
}

impl PartialEq<&InteractionMember> for CachedMember {
    fn eq(&self, other: &&InteractionMember) -> bool {
        (
            self.joined_at.as_ref(),
            &self.nick,
            &self.premium_since,
            &self.roles,
        ) == (
            other.joined_at.as_ref(),
            &other.nick,
            &other.premium_since,
            &other.roles,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::CachedMember;
    use static_assertions::assert_fields;
    use twilight_model::{
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
        CachedMember {
            deaf: Some(false),
            guild_id: GuildId(3),
            joined_at: None,
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
            id: UserId(1),
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
        let member = Member {
            deaf: false,
            guild_id: GuildId(3),
            hoisted_role: Some(RoleId(4)),
            joined_at: None,
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
        let member = PartialMember {
            deaf: false,
            joined_at: None,
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
