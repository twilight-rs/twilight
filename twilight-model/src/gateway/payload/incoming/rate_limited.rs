use crate::{
    gateway::OpCode,
    id::{Id, marker::GuildMarker},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct RateLimited {
    pub opcode: OpCode,
    pub retry_after: f32,
    pub meta: RateLimitMetadata,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[non_exhaustive]
pub enum RateLimitMetadata {
    RequestGuildMembers {
        guild_id: Id<GuildMarker>,
        #[serde(skip_serializing_if = "Option::is_none")]
        nonce: Option<String>,
    },
}
