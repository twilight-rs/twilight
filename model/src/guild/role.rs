use crate::{
    guild::Permissions,
    id::RoleId,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Role {
    pub id: RoleId,
    pub color: u32,
    pub hoist: bool,
    pub managed: bool,
    #[serde(default)]
    pub mentionable: bool,
    pub name: String,
    pub permissions: Permissions,
    pub position: i64,
}
