use crate::{
    id::{Id, marker::GuildMarker},
    user::User,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MemberRemove {
    pub guild_id: Id<GuildMarker>,
    pub user: User,
}
