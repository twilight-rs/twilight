use crate::{guild::GuildStatus, user::CurrentUser};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Ready {
    pub guilds: Vec<GuildStatus>,
    pub session_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard: Option<[u64; 2]>,
    pub user: CurrentUser,
    #[serde(rename = "v")]
    pub version: u64,
}
