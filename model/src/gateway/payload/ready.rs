use crate::{guild::GuildStatus, id::GuildId, user::CurrentUser};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Ready {
    #[serde(with = "serde_mappable_seq")]
    pub guilds: HashMap<GuildId, GuildStatus>,
    pub session_id: String,
    pub shard: Option<[u64; 2]>,
    pub user: CurrentUser,
    #[serde(rename = "v")]
    pub version: u64,
}
