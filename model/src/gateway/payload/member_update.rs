use crate::{
    id::{GuildId, RoleId},
    user::User,
};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MemberUpdate {
    pub guild_id: GuildId,
    pub nick: Option<String>,
    pub roles: Vec<RoleId>,
    pub user: User,
}
