use crate::{
    id::{GuildId, UserId},
    oauth::{id::SkuId, team::Team},
    user::User,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct CurrentApplicationInfo {
    pub id: UserId,
    pub bot_public: bool,
    pub bot_require_code_grant: bool,
    pub cover_image: Option<String>,
    pub description: String,
    pub guild_id: Option<GuildId>,
    pub icon: Option<String>,
    pub name: String,
    pub owner: User,
    pub primary_sku_id: Option<SkuId>,
    #[serde(default)]
    pub rpc_origins: Vec<String>,
    pub slug: Option<String>,
    pub summary: String,
    pub team: Option<Team>,
    pub verify_key: String,
}
