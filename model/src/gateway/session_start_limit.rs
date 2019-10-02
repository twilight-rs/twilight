#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SessionStartLimit {
    pub remaining: u64,
    pub reset_after: u64,
    pub total: u64,
}
