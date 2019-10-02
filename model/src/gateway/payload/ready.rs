use crate::{guild::GuildStatus, id::GuildId, user::CurrentUser};
use std::collections::HashMap;

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ready {
    #[cfg_attr(feature = "serde-support", serde(with = "serde_mappable_seq"))]
    pub guilds: HashMap<GuildId, GuildStatus>,
    pub session_id: String,
    pub shard: Option<[u64; 2]>,
    pub user: CurrentUser,
    #[cfg_attr(feature = "serde-support", serde(rename = "v"))]
    pub version: u64,
}
