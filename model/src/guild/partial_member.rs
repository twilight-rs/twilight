use crate::id::RoleId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PartialMember {
    pub deaf: bool,
    pub joined_at: Option<String>,
    pub mute: bool,
    pub roles: Vec<RoleId>,
}
