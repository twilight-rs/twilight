use crate::gateway::SessionStartLimit;

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct BotConnectionInfo {
    pub session_start_limit: SessionStartLimit,
    pub shards: u64,
    pub url: String,
}
