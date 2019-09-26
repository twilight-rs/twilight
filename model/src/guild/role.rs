use crate::{guild::Permissions, id::RoleId};
use serde::{Deserialize, Serialize};
use serde_mappable_seq::Key;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
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

impl Hash for Role {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Key<'_, RoleId> for Role {
    fn key(&self) -> RoleId {
        self.id
    }
}
