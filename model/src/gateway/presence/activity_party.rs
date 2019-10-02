#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ActivityParty {
    pub id: Option<String>,
    pub size: Option<[u64; 2]>,
}
