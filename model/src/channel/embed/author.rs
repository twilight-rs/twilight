#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct EmbedAuthor {
    pub icon_url: Option<String>,
    pub name: Option<String>,
    pub proxy_icon_url: Option<String>,
    pub url: Option<String>,
}
