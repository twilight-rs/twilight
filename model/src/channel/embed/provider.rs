use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmbedProvider {
    pub name: String,
    pub url: Option<String>,
}
