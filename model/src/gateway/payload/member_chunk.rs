use crate::{
    guild::Member,
    id::GuildId,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MemberChunk {
    pub guild_id: GuildId,
    pub members: Vec<Member>,
}
