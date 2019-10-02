#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ActivitySecrets {
    pub join: Option<String>,
    #[cfg_attr(feature = "serde-support", serde(rename = "match"))]
    pub match_: Option<String>,
    pub spectate: Option<String>,
}
