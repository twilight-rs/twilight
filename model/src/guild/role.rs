use crate::{guild::Permissions, id::RoleId};
use std::hash::{Hash, Hasher};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Role {
    pub id: RoleId,
    pub color: u32,
    pub hoist: bool,
    pub managed: bool,
    #[cfg_attr(feature = "serde-support", serde(default))]
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

#[cfg(feature = "serde-support")]
mod serde_support {
    use super::Role;
    use crate::id::RoleId;
    use serde_mappable_seq::Key;

    impl Key<'_, RoleId> for Role {
        fn key(&self) -> RoleId {
            self.id
        }
    }
}
