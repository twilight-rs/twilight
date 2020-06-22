use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct EmbedAuthor {
    pub icon_url: Option<String>,
    pub name: Option<String>,
    pub proxy_icon_url: Option<String>,
    pub url: Option<String>,
}
