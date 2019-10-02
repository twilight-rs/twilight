#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct EmbedVideo {
    pub height: u64,
    pub url: String,
    pub width: u64,
}
