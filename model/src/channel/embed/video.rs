use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EmbedVideo {
    pub height: u64,
    pub url: String,
    pub width: u64,
}
