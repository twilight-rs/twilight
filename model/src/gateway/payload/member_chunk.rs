use crate::{
    guild::Member,
    id::{GuildId, UserId},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MemberChunk {
    pub guild_id: GuildId,
    #[serde(with = "serde_mappable_seq")]
    pub members: HashMap<UserId, Member>,
}
