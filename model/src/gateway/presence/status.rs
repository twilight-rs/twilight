#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Status {
    #[cfg_attr(feature = "serde-support", serde(rename = "dnd"))]
    DoNotDisturb,
    #[cfg_attr(feature = "serde-support", serde(rename = "idle"))]
    Idle,
    #[cfg_attr(feature = "serde-support", serde(rename = "invisible"))]
    Invisible,
    #[cfg_attr(feature = "serde-support", serde(rename = "offline"))]
    Offline,
    #[cfg_attr(feature = "serde-support", serde(rename = "online"))]
    Online,
}
