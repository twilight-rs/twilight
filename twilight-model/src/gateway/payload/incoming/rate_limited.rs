use crate::id::{Id, marker::GuildMarker};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct RateLimited {
    pub opcode: u8,
    pub retry_after: f64,
    pub meta: RateLimitMetadata,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum RateLimitMetadata {
    RequestGuildMembers {
        guild_id: Id<GuildMarker>,
        nonce: Option<String>,
    },
}
