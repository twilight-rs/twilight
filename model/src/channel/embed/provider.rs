use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EmbedProvider {
    pub name: String,
    pub url: Option<String>,
}
