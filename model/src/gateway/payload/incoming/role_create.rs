use crate::{
    guild::Role,
    id::{marker, Id},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct RoleCreate {
    pub guild_id: Id<marker::Guild>,
    pub role: Role,
}
