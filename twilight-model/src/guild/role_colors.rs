use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct RoleColors {
    pub primary_color: u32,
    pub secondary_color: Option<u32>,
    pub tertiary_color: Option<u32>,
}
