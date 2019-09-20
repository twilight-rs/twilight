use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct VoiceRegion {
    pub custom: bool,
    pub deprecated: bool,
    pub id: String,
    pub name: String,
    pub optional: bool,
    pub vip: bool,
}
