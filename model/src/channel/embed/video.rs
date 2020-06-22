use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct EmbedVideo {
    pub height: Option<u64>,
    pub url: Option<String>,
    pub width: Option<u64>,
}
