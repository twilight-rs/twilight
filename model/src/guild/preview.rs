use crate::{guild::Emoji, id::GuildId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GuildPreview {
    pub approximate_member_count: u64,
    pub approximate_presence_count: u64,
    pub description: Option<String>,
    pub discovery_splash: Option<String>,
    pub emojis: Vec<Emoji>,
    pub features: Vec<String>,
    pub id: GuildId,
    pub name: String,
    pub icon: Option<String>,
    pub splash: Option<String>,
}
