use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct EmbedImage {
    pub height: Option<u64>,
    pub proxy_url: Option<String>,
    pub url: Option<String>,
    pub width: Option<u64>,
}
