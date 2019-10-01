use crate::{
    id::{GuildId, RoleId, UserId},
    user::User,
};
use serde::{Deserialize, Serialize};
use serde_mappable_seq::Key;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Member {
    pub deaf: bool,
    pub guild_id: GuildId,
    pub joined_at: Option<String>,
    pub mute: bool,
    pub nick: Option<String>,
    pub premium_since: Option<String>,
    pub roles: Vec<RoleId>,
    pub user: User,
}

impl Hash for Member {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.guild_id.hash(state);
        self.user.id.hash(state);
    }
}

impl Key<'_, UserId> for Member {
    fn key(&self) -> UserId {
        self.user.id
    }
}
