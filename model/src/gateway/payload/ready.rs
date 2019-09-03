use crate::{
    guild::GuildStatus,
    user::CurrentUser,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Ready {
    pub guilds: Vec<GuildStatus>,
    pub session_id: String,
    pub shard: Option<[u64; 2]>,
    pub user: CurrentUser,
    #[serde(rename = "v")]
    pub version: u64,
}
