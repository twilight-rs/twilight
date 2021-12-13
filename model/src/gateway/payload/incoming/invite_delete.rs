use crate::id::{marker, Id};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct InviteDelete {
    pub channel_id: Id<marker::Channel>,
    pub code: String,
    pub guild_id: Id<marker::Guild>,
}
