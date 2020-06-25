use crate::{
    gateway::{
        opcode::OpCode,
        payload::request_guild_members::RequestGuildMembersInfo::{MultiUser, Query, SingleUser},
    },
    id::{GuildId, UserId},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct RequestGuildMembers {
    pub d: RequestGuildMembersInfo,
    pub op: OpCode,
}

impl RequestGuildMembers {
    pub fn new_all(guild_id: impl Into<GuildId>, presences: Option<bool>) -> Self {
        Self::new_all_with_nonce(guild_id, presences, None)
    }

    pub fn new_all_with_nonce(
        guild_id: impl Into<GuildId>,
        presences: Option<bool>,
        nonce: Option<String>,
    ) -> Self {
        Self {
            d: RequestGuildMembersInfo::new_all(guild_id, presences, nonce),
            op: OpCode::RequestGuildMembers,
        }
    }

    pub fn new(
        guild_id: impl Into<GuildId>,
        limit: u64,
        query: impl Into<String>,
        presences: Option<bool>,
    ) -> Self {
        Self::new_with_nonce(guild_id, limit, query, presences, None)
    }

    pub fn new_with_nonce(
        guild_id: impl Into<GuildId>,
        limit: u64,
        query: impl Into<String>,
        presences: Option<bool>,
        nonce: Option<String>,
    ) -> Self {
        Self {
            d: RequestGuildMembersInfo::new(guild_id, limit, query, presences, nonce),
            op: OpCode::RequestGuildMembers,
        }
    }

    pub fn new_single_user(
        guild_id: impl Into<GuildId>,
        user: impl Into<UserId>,
        presence: Option<bool>,
    ) -> Self {
        Self::new_single_user_with_nonce(guild_id, user, presence, None)
    }

    pub fn new_single_user_with_nonce(
        guild_id: impl Into<GuildId>,
        user: impl Into<UserId>,
        presence: Option<bool>,
        nonce: Option<String>,
    ) -> Self {
        Self {
            d: RequestGuildMembersInfo::new_single_user(guild_id, user, presence, nonce),
            op: OpCode::RequestGuildMembers,
        }
    }

    pub fn new_multi_user(
        guild_id: impl Into<GuildId>,
        users: Vec<UserId>,
        presences: Option<bool>,
    ) -> Self {
        Self::new_multi_user_with_nonce(guild_id, users, presences, None)
    }

    pub fn new_multi_user_with_nonce(
        guild_id: impl Into<GuildId>,
        users: Vec<UserId>,
        presences: Option<bool>,
        nonce: Option<String>,
    ) -> Self {
        Self {
            d: RequestGuildMembersInfo::new_multi_user(guild_id, users, presences, nonce),
            op: OpCode::RequestGuildMembers,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(untagged)]
pub enum RequestGuildMembersInfo {
    Query {
        guild_id: GuildId,
        limit: u64,
        query: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        presences: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        nonce: Option<String>,
    },
    SingleUser {
        guild_id: GuildId,
        user_ids: UserId,
        #[serde(skip_serializing_if = "Option::is_none")]
        presences: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        nonce: Option<String>,
    },
    MultiUser {
        guild_id: GuildId,
        user_ids: Vec<UserId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        presences: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        nonce: Option<String>,
    },
}

impl RequestGuildMembersInfo {
    pub fn new_all(
        guild_id: impl Into<GuildId>,
        presences: Option<bool>,
        nonce: Option<String>,
    ) -> Self {
        Self::_new_query(guild_id.into(), 0, String::from(""), presences, nonce)
    }

    pub fn new(
        guild_id: impl Into<GuildId>,
        limit: u64,
        query: impl Into<String>,
        presences: Option<bool>,
        nonce: Option<String>,
    ) -> Self {
        Self::_new_query(guild_id.into(), limit, query.into(), presences, nonce)
    }

    fn _new_query(
        guild_id: GuildId,
        limit: u64,
        query: String,
        presences: Option<bool>,
        nonce: Option<String>,
    ) -> Self {
        Query {
            guild_id,
            limit,
            query,
            presences,
            nonce,
        }
    }

    pub fn new_single_user(
        guild_id: impl Into<GuildId>,
        user: impl Into<UserId>,
        presence: Option<bool>,
        nonce: Option<String>,
    ) -> Self {
        Self::_new_single_user(guild_id.into(), user.into(), presence, nonce)
    }

    fn _new_single_user(
        guild_id: GuildId,
        user: UserId,
        presences: Option<bool>,
        nonce: Option<String>,
    ) -> Self {
        SingleUser {
            guild_id,
            presences,
            user_ids: user,
            nonce,
        }
    }

    pub fn new_multi_user(
        guild_id: impl Into<GuildId>,
        users: Vec<UserId>,
        presences: Option<bool>,
        nonce: Option<String>,
    ) -> Self {
        Self::_new_multi_user(guild_id.into(), users, presences, nonce)
    }

    fn _new_multi_user(
        guild_id: GuildId,
        user_ids: Vec<UserId>,
        presences: Option<bool>,
        nonce: Option<String>,
    ) -> Self {
        MultiUser {
            guild_id,
            user_ids,
            presences,
            nonce,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct RequestGuildMemberInfo {
    pub guild_id: GuildId,
    pub user_ids: u64,
    pub query: String,
    pub presences: bool,
}
