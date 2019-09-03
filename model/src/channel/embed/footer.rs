use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmbedFooter {
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
    pub text: String,
}
