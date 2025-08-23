use twilight_cache_inmemory::CacheableRole;
use twilight_model::{
    guild::{Permissions, Role},
    id::{Id, marker::RoleMarker},
};

#[derive(Clone, Debug, PartialEq)]
pub struct MinimalCachedRole {
    pub id: Id<RoleMarker>,
    pub permissions: Permissions,
    pub position: i64,
}

impl From<Role> for MinimalCachedRole {
    fn from(role: Role) -> Self {
        Self {
            id: role.id,
            permissions: role.permissions,
            position: role.position,
        }
    }
}

impl PartialEq<Role> for MinimalCachedRole {
    fn eq(&self, other: &Role) -> bool {
        self.id == other.id
            && self.permissions == other.permissions
            && self.position == other.position
    }
}

impl CacheableRole for MinimalCachedRole {
    fn id(&self) -> Id<RoleMarker> {
        self.id
    }

    fn permissions(&self) -> Permissions {
        self.permissions
    }

    fn position(&self) -> i64 {
        self.position
    }
}
