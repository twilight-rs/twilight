use crate::{
    guild::Permissions,
    id::RoleId,
};
use serde::{Deserialize, Serialize};
use serde_mappable_seq::Key;

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

impl Key<'_, RoleId> for Role {
    fn key(&self) -> RoleId {
        self.id
    }
}
