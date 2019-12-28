use crate::{guild::Permissions, id::RoleId};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Role {
    pub id: RoleId,
    pub color: u32,
    pub hoist: bool,
    pub managed: bool,
    pub mentionable: bool,
    pub name: String,
    pub permissions: Permissions,
    pub position: i64,
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
