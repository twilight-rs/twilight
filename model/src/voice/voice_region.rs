#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct VoiceRegion {
    pub custom: bool,
    pub deprecated: bool,
    pub id: String,
    pub name: String,
    pub optional: bool,
    pub vip: bool,
}
