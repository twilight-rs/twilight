use crate::{guild::Permissions, id::GuildId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GuildInfo {
    pub id: GuildId,
    pub icon: Option<String>,
    pub name: String,
    pub owner: bool,
    pub permissions: Permissions,
}
