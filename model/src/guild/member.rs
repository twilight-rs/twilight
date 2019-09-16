use crate::{
    id::{GuildId, RoleId, UserId},
    user::User,
};
use serde::{Deserialize, Serialize};
use serde_mappable_seq::Key;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Member {
    pub deaf: bool,
    pub guild_id: GuildId,
    #[cfg(feature = "chrono")]
    pub joined_at: Option<chrono::DateTime<chrono::FixedOffset>>,
    #[cfg(not(feature = "chrono"))]
    pub joined_at: Option<String>,
    pub mute: bool,
    pub nick: Option<String>,
    #[cfg(feature = "chrono")]
    pub premium_since: Option<chrono::DateTime<chrono::FixedOffset>>,
    #[cfg(not(feature = "chrono"))]
    pub premium_since: Option<String>,
    pub roles: Vec<RoleId>,
    pub user: User,
}

impl Key<'_,  UserId> for Member {
    fn key(&self) -> UserId {
        self.user.id
    }
}
