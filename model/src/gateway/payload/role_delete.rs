use crate::id::{GuildId, RoleId};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RoleDelete {
    pub guild_id: GuildId,
    pub role_id: RoleId,
}
