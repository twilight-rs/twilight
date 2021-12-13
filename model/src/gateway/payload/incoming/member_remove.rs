use crate::{
    id::{marker, Id},
    user::User,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MemberRemove {
    pub guild_id: Id<marker::Guild>,
    pub user: User,
}
