use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmbedField {
    #[serde(default)]
    pub inline: bool,
    pub name: String,
    pub value: String,
}
