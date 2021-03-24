mod guild;
mod role;

pub use guild::TemplateGuild;
pub use role::TemplateRole;

use crate::{
    id::{GuildId, UserId},
    user::User,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Template {
    pub code: String,
    pub created_at: String,
    pub creator: User,
    pub creator_id: UserId,
    pub description: Option<String>,
    pub is_dirty: Option<bool>,
    pub name: String,
    pub serialized_source_guild: TemplateGuild,
    pub source_guild_id: GuildId,
    pub updated_at: String,
    pub usage_count: u64,
}
