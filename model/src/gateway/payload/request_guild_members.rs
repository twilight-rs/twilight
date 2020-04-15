use crate::{
    gateway::{
        opcode::OpCode,
        payload::request_guild_members::RequestGuildMembersInfo::{MultiUser, Query, SingleUser},
    },
    id::{GuildId, UserId},
};

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
    pub fn new_all(guild_id: impl Into<GuildId>, presences: Option<bool>) -> Self {
        Self {
            d: RequestGuildMembersInfo::new_all(guild_id, presences),
            op: OpCode::RequestGuildMembers,
        }
    }

    pub fn new(
        guild_id: impl Into<GuildId>,
        limit: u64,
        query: impl Into<String>,
        presences: Option<bool>,
    ) -> Self {
        Self {
            d: RequestGuildMembersInfo::new(guild_id, limit, query, presences),
            op: OpCode::RequestGuildMembers,
        }
    }

    pub fn new_single_user(
        guild_id: impl Into<GuildId>,
        user: impl Into<UserId>,
        presence: Option<bool>,
    ) -> Self {
        Self {
            d: RequestGuildMembersInfo::new_single_user(guild_id, user, presence),
            op: OpCode::RequestGuildMembers,
        }
    }

    pub fn new_multi_user(
        guild_id: impl Into<GuildId>,
        users: Vec<UserId>,
        presences: Option<bool>,
    ) -> Self {
        Self {
            d: RequestGuildMembersInfo::new_multi_user(guild_id, users, presences),
            op: OpCode::RequestGuildMembers,
        }
    }
}

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize),
    serde(untagged)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum RequestGuildMembersInfo {
    Query {
        guild_id: GuildId,
        limit: u64,
        query: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        presences: Option<bool>,
    },
    SingleUser {
        guild_id: GuildId,
        user_ids: UserId,
        #[serde(skip_serializing_if = "Option::is_none")]
        presences: Option<bool>,
    },
    MultiUser {
        guild_id: GuildId,
        user_ids: Vec<UserId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        presences: Option<bool>,
    },
}

impl RequestGuildMembersInfo {
    pub fn new_all(guild_id: impl Into<GuildId>, presences: Option<bool>) -> Self {
        Self::_new_query(guild_id.into(), 0, String::from(""), presences)
    }

    pub fn new(
        guild_id: impl Into<GuildId>,
        limit: u64,
        query: impl Into<String>,
        presences: Option<bool>,
    ) -> Self {
        Self::_new_query(guild_id.into(), limit, query.into(), presences)
    }

    fn _new_query(guild_id: GuildId, limit: u64, query: String, presences: Option<bool>) -> Self {
        Query {
            guild_id,
            limit,
            query,
            presences,
        }
    }

    pub fn new_single_user(
        guild_id: impl Into<GuildId>,
        user: impl Into<UserId>,
        presence: Option<bool>,
    ) -> Self {
        Self::_new_single_user(guild_id.into(), user.into(), presence)
    }

    fn _new_single_user(guild_id: GuildId, user: UserId, presences: Option<bool>) -> Self {
        SingleUser {
            guild_id,
            presences,
            user_ids: user,
        }
    }

    pub fn new_multi_user(
        guild_id: impl Into<GuildId>,
        users: Vec<UserId>,
        presences: Option<bool>,
    ) -> Self {
        Self::_new_multi_user(guild_id.into(), users, presences)
    }

    fn _new_multi_user(guild_id: GuildId, user_ids: Vec<UserId>, presences: Option<bool>) -> Self {
        MultiUser {
            guild_id,
            user_ids,
            presences,
        }
    }
}

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RequestGuildMemberInfo {
    pub guild_id: GuildId,
    pub user_ids: u64,
    pub query: String,
    pub presences: bool,
}
