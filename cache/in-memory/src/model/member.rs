use serde::Serialize;
use twilight_model::{
    application::interaction::application_command::InteractionMember,
    datetime::Timestamp,
    guild::{Member, PartialMember},
    id::{
        marker::{GuildMarker, RoleMarker, UserMarker},
        Id,
    },
    util::image_hash::ImageHash,
};

/// Represents a cached [`Member`].
///
/// [`Member`]: twilight_model::guild::Member
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CachedMember {
    pub(crate) avatar: Option<ImageHash>,
    pub(crate) communication_disabled_until: Option<Timestamp>,
    pub(crate) deaf: Option<bool>,
    pub(crate) guild_id: Id<GuildMarker>,
    pub(crate) joined_at: Timestamp,
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

    /// ID of the guild this member is a part of.
    pub const fn guild_id(&self) -> Id<GuildMarker> {
        self.guild_id
    }

    /// [`Timestamp`] of this member's join date.
    pub const fn joined_at(&self) -> Timestamp {
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

#[cfg(test)]
mod tests {
    use super::CachedMember;
    use static_assertions::assert_fields;
    use twilight_model::{
        datetime::Timestamp,
        guild::{Member, PartialMember},
        id::Id,
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
            avatar: None,
            communication_disabled_until: None,
            deaf: Some(false),
            guild_id: Id::new(3),
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
    fn test_eq_member() {
        let joined_at = Timestamp::from_secs(1_632_072_645).expect("non zero");

        let member = Member {
            avatar: None,
            communication_disabled_until: None,
            deaf: false,
            guild_id: Id::new(3),
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
            avatar: None,
            communication_disabled_until: None,
            deaf: false,
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
