use crate::{
    guild::{Permissions, RoleTags},
    id::{marker, Id},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct TemplateRole {
    pub color: u32,
    pub hoist: bool,
    pub id: Id<marker::Role>,
    pub mentionable: bool,
    pub name: String,
    pub permissions: Permissions,
    /// Tags about the role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<RoleTags>,
}
