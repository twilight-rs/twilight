use crate::gateway::SessionStartLimit;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BotConnectionInfo {
    pub session_start_limit: SessionStartLimit,
    pub shards: u64,
    pub url: String,
}
