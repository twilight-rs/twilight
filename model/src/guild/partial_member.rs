use crate::id::RoleId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PartialMember {
    pub deaf: bool,
    #[cfg(feature = "chrono")]
    pub joined_at: Option<chrono::DateTime<chrono::FixedOffset>>,
    #[cfg(not(feature = "chrono"))]
    pub joined_at: Option<String>,
    pub mute: bool,
    pub roles: Vec<RoleId>,
}
