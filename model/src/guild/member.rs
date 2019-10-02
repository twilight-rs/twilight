use crate::{
    id::{GuildId, RoleId},
    user::User,
};
use std::hash::{Hash, Hasher};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, PartialEq)]
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

#[cfg(feature = "serde-support")]
mod serde_support {
    use super::Member;
    use crate::id::UserId;
    use serde_mappable_seq::Key;

    impl Key<'_, UserId> for Member {
        fn key(&self) -> UserId {
            self.user.id
        }
    }
}
