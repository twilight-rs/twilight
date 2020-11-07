use serde::Serialize;
use std::sync::Arc;
use twilight_model::{
    guild::{Member, PartialMember},
    id::{GuildId, RoleId},
    user::User,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CachedMember {
    pub deaf: bool,
    pub guild_id: GuildId,
    pub joined_at: Option<String>,
    pub mute: bool,
    pub nick: Option<String>,
    pub premium_since: Option<String>,
    pub roles: Vec<RoleId>,
    pub user: Arc<User>,
}

impl PartialEq<Member> for CachedMember {
    fn eq(&self, other: &Member) -> bool {
        (
            self.deaf,
            self.joined_at.as_ref(),
            self.mute,
            &self.nick,
            self.premium_since.as_ref(),
            &self.roles,
        ) == (
            other.deaf,
            other.joined_at.as_ref(),
            other.mute,
            &other.nick,
            other.premium_since.as_ref(),
            &other.roles,
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
            &self.roles,
        ) == (
            other.deaf,
            other.joined_at.as_ref(),
            other.mute,
            &other.nick,
            &other.roles,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::CachedMember;
    use std::sync::Arc;
    use twilight_model::{
        guild::{Member, PartialMember},
        id::{GuildId, RoleId, UserId},
        user::User,
    };

    #[test]
    fn test_eq_member() {
        let user = User {
            avatar: None,
            bot: false,
            discriminator: "0001".to_owned(),
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
        };

        let member = Member {
            deaf: false,
            guild_id: GuildId(3),
            hoisted_role: Some(RoleId(4)),
            joined_at: None,
            mute: true,
            nick: Some("member nick".to_owned()),
            premium_since: None,
            roles: vec![],
            user: user.clone(),
        };
        let cached = CachedMember {
            deaf: false,
            guild_id: GuildId(3),
            joined_at: None,
            mute: true,
            nick: Some("member nick".to_owned()),
            premium_since: None,
            roles: vec![],
            user: Arc::new(user),
        };

        assert_eq!(cached, member);
    }

    #[test]
    fn test_eq_partial_member() {
        let user = User {
            avatar: None,
            bot: false,
            discriminator: "0001".to_owned(),
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
        };

        let member = PartialMember {
            deaf: false,
            joined_at: None,
            mute: true,
            nick: Some("member nick".to_owned()),
            roles: vec![],
        };
        let cached = CachedMember {
            deaf: false,
            guild_id: GuildId(3),
            joined_at: None,
            mute: true,
            nick: Some("member nick".to_owned()),
            premium_since: None,
            roles: vec![],
            user: Arc::new(user),
        };

        assert_eq!(cached, &member);
    }
}
