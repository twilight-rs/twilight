use crate::id::{RoleId, UserId};
use serde::{Deserialize, Serialize};

mod builder;

pub use self::builder::AllowedMentionsBuilder;

#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_false(value: &bool) -> bool {
    !value
}

/// Parse types.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "lowercase")]
pub enum ParseTypes {
    Users,
    Roles,
    Everyone,
}

/// Allowed mentions structure.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AllowedMentions {
    #[serde(default)]
    parse: Vec<ParseTypes>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    users: Vec<UserId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    roles: Vec<RoleId>,
    #[serde(default, skip_serializing_if = "is_false")]
    replied_user: bool,
}
