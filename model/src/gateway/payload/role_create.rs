use crate::{guild::Role, id::GuildId};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RoleCreate {
    pub guild_id: GuildId,
    pub role: Role,
}
