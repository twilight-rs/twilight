#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct EmbedFooter {
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
    pub text: String,
}
