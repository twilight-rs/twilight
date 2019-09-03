use crate::{
    id::{GuildId, RoleId},
    user::User,
};
use serde::{Deserialize, Serialize};

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
