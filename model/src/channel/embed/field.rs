use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct EmbedField {
    #[serde(default)]
    pub inline: bool,
    pub name: String,
    pub value: String,
}
