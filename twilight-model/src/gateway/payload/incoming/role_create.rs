use crate::{
    guild::Role,
    id::{Id, marker::GuildMarker},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct RoleCreate {
    pub guild_id: Id<GuildMarker>,
    pub role: Role,
}
