#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct EmbedField {
    #[cfg_attr(feature = "serde-support", serde(default))]
    pub inline: bool,
    pub name: String,
    pub value: String,
}
