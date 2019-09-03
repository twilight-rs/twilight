use crate::{
    guild::Member,
    id::GuildId,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MemberAdd {
    pub guild_id: GuildId,
    pub member: Member,
}
