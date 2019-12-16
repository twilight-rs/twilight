#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct EmbedThumbnail {
    pub height: Option<u64>,
    pub proxy_url: Option<String>,
    pub url: Option<String>,
    pub width: Option<u64>,
}
