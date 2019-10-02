use crate::{gateway::opcode::OpCode, id::GuildId};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RequestGuildMembers {
    pub d: RequestGuildMembersInfo,
    pub op: OpCode,
}

impl RequestGuildMembers {
    pub fn new(guild_id: impl Into<GuildId>, limit: u64, query: impl Into<String>) -> Self {
        Self {
            d: RequestGuildMembersInfo::new(guild_id, limit, query),
            op: OpCode::RequestGuildMembers,
        }
    }
}

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RequestGuildMembersInfo {
    pub guild_id: GuildId,
    pub limit: u64,
    pub query: String,
}

impl RequestGuildMembersInfo {
    pub fn new(guild_id: impl Into<GuildId>, limit: u64, query: impl Into<String>) -> Self {
        Self::_new(guild_id.into(), limit, query.into())
    }

    fn _new(guild_id: GuildId, limit: u64, query: String) -> Self {
        Self {
            guild_id,
            limit,
            query,
        }
    }
}
