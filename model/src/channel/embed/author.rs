use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmbedAuthor {
    pub icon_url: Option<String>,
    pub name: String,
    pub proxy_icon_url: Option<String>,
    pub url: Option<String>,
}
