use crate::{guild::Permissions, id::GuildId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GuildInfo {
    pub icon: Option<String>,
    pub id: GuildId,
    pub name: String,
    pub owner: bool,
    pub permissions: Permissions,
}
