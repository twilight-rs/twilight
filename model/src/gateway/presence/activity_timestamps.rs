#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ActivityTimestamps {
    pub end: Option<u64>,
    pub start: Option<u64>,
}
