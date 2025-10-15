use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct RoleColors {
    pub primary_color: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_color: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tertiary_color: Option<u32>,
}
