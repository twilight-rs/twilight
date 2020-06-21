use serde::{Deserialize, Serialize};

#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct VoiceRegion {
    pub custom: bool,
    pub deprecated: bool,
    pub id: String,
    pub name: String,
    pub optional: bool,
    pub vip: bool,
}
