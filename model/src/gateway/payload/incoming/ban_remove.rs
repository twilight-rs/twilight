use crate::{id::GuildId, user::User};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct BanRemove {
    pub guild_id: GuildId,
    pub user: User,
}
