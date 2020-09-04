use serde::Serialize;
use std::sync::Arc;
use twilight_model::{
    guild::Member,
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
