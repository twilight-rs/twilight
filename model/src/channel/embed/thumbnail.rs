#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct EmbedThumbnail {
    pub height: u64,
    pub proxy_url: String,
    pub url: String,
    pub width: u64,
}
